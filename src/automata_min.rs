use std::collections::{BTreeSet};
use automata::{M, StateSet, State};
use automata_operators::stateset_name;

pub type RelationMatrixRow = Vec<bool>;
pub type RelationMatrix = Vec<RelationMatrixRow>;


fn get_relation_matrix(m: &M) -> RelationMatrix {
    let mut matrix: RelationMatrix = Vec::new();

    for qi in m.k.iter() {
        let mut row: RelationMatrixRow = Vec::new();
        for qj in m.k.iter() {
            let mut i_relation_j = false;

            for a in m.alphabet.iter() {
                if m.get_next_states(qi, &a).contains(qj) {
                    i_relation_j = true
                }
            }

            row.push(i_relation_j);
        }
        matrix.push(row);
    }

    matrix
}


fn warshall(matrix: &RelationMatrix) -> RelationMatrix {
    let n = matrix.len();
    let mut r = matrix.clone();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                r[i][j] = r[i][j] || ( r[i][k] && r[k][j] )
            }
        }
    }

    r
}

fn get_reachable_states(m: &M, r: &RelationMatrix) -> StateSet {
    let states: Vec<State> = m.k.iter().cloned().collect();
    let q0_index = m.k.iter().position(|s| *s == m.q0).unwrap();
    let ref reachable_state_row = r[q0_index];
    let reachable_states = {
        let mut res: StateSet = BTreeSet::new();
        for (i, &reachable) in reachable_state_row.iter().enumerate() {
            if reachable {
                res.insert(states[i].clone());
            }
        }
        res
    };

    reachable_states
}


fn remove_unreachable_states_with_params(m: &M, reachable_states: StateSet) -> M {
    let mut m: M = (*m).clone();
    for u in m.k.difference(&reachable_states) {
        let _ = m.delta.remove(u);
    }

    m.k = reachable_states;

    m
}


pub type EquivalenceClass = StateSet;
pub type Quotient = BTreeSet<EquivalenceClass>;

fn get_equivalence_class(states: &StateSet, quotient: &Quotient) -> EquivalenceClass {
    for eq_class in quotient.iter() {
        if states.is_subset(&eq_class) {
            return eq_class.clone();
        }
    }

    stateset!()
}

fn get_quotient(m: &M) -> Quotient {
    let k_f: EquivalenceClass = m.k.difference(&m.f).cloned().collect();
    let f: EquivalenceClass = m.f.clone();

    let mut quotient: Quotient = {
        let mut q = BTreeSet::new();
        q.insert(k_f);
        q.insert(f);
        q
    };

    let mut fin = false;

    while !fin {
        let mut next_quotient: Quotient = BTreeSet::new();
        for x in &quotient {
            let mut x_marked: EquivalenceClass = BTreeSet::new();

            while x != &x_marked {
                for state in x {
                    if x_marked.contains(state) { continue }
                    println!("STATE {:?}", state);
                    let mut x1: EquivalenceClass = stateset!(state);

                    x_marked.insert(state.clone());

                    for other_state in x {
                        if x_marked.contains(other_state) { continue }
                        println!("OTHER_STATE {:?}", other_state);
                        let mut flag = true;
                        for a in &m.alphabet {
                            let state_eq_class = get_equivalence_class(&m.get_next_states(&state, &a), &quotient);
                            let other_state_eq_class = get_equivalence_class(&m.get_next_states(&other_state, &a), &quotient);
                            if  state_eq_class == other_state_eq_class  {
                                flag = flag && true
                            } else {
                                flag = flag && false
                            }
                        }
                        if flag {
                            println!("SAME CLASS {:?}, {:?}", state, other_state);
                            x1.insert(other_state.clone());
                            x_marked.insert(other_state.clone());
                        }
                    }

                    next_quotient.insert(x1);
                }
            }
        }

        println!("     QUOTIENT {:?}", quotient);
        println!("Next QUOTIENT {:?}", next_quotient);
        if next_quotient != quotient {
            quotient = next_quotient;
            fin = false;
        } else {
            fin = true;
        }
    }

    quotient
}

fn apply_quotient(m: &M, quotient: &Quotient) -> M {
    let states: StateSet = quotient.iter().map(|eq_class| stateset_name(eq_class)).collect();

    let q0 = stateset_name(&get_equivalence_class(&stateset!(m.q0), quotient));
    let mut f = stateset!();
    for state in &m.f {
        let final_state = stateset_name(&get_equivalence_class(&stateset!(state), quotient));
        f.insert(final_state);
    }

    let mut delta = delta!();
    for (state, delta_value) in &m.delta {
        for (a, next_states) in delta_value {
            for next_state in next_states {
                let s = stateset_name(&get_equivalence_class(&stateset!(state), quotient));
                let ns = stateset_name(&get_equivalence_class(&stateset!(next_state), quotient));
                delta.insert( (s, a.clone(), ns) );
            }
        }
    }

    M::new(
        states,
        m.alphabet.clone(),
        q0,
        f,
        delta
    )
}


//TODO: test
fn remove_unreachable_states(m: &M) -> M {
    let relation_matrix: RelationMatrix = get_relation_matrix(&m);
    let r_star: RelationMatrix = warshall(&relation_matrix);
    let reachable_states: StateSet = get_reachable_states(&m, &r_star);
    let m: M = remove_unreachable_states_with_params(&m, reachable_states);

    m
}

//TODO: test
pub fn minify(m: &M) -> M {
    let m: M = remove_unreachable_states(m);
    let quotient: Quotient = get_quotient(&m);
    let m: M = apply_quotient(&m, &quotient);

    m
}



#[cfg(test)]
mod tests {
    #[test]
    fn get_relation_matrix_test() {
        use super::get_relation_matrix;
        use automata::M;

        let k = stateset!("q0", "q1", "q2", "q3");
        let alphabet = alphabet!('a', 'b');
        let q0 = "q0".to_string();
        let f = stateset!("q1");
        let delta = delta!(
            ("q0", 'a', "q0"),
            ("q0", 'b', "q1"),
            ("q1", 'a', "q1"),
            ("q1", 'b', "q2"),
            ("q2", 'a', "q0"),
            ("q2", 'b', "q2"),
            ("q3", 'a', "q3")
        );


        let m = M::new(k, alphabet, q0, f, delta);

        let rm = get_relation_matrix(&m);
        let rm_expected = vec![
            vec![true, true, false, false],
            vec![false, true, true, false],
            vec![true, false, true, false],
            vec![false, false, false, true]
        ];

        assert_eq!(rm, rm_expected);
    }

    #[test]
    fn warshall_test() {
        use super::warshall;

        let rm = vec![
            vec![true, true, false, false],
            vec![false, true, true, false],
            vec![true, false, true, false],
            vec![false, false, false, true]
        ];

        let r = warshall(&rm);
        let r_expected = vec![
            vec![true, true, true, false],
            vec![true, true, true, false],
            vec![true, true, true, false],
            vec![false, false, false, true]
        ];

        assert_eq!(r, r_expected);
    }

    #[test]
    fn get_reachable_states_test() {
        use super::get_reachable_states;
        use automata::M;

        let k = stateset!("q0", "q1", "q2", "q3");
        let alphabet = alphabet!('a', 'b');
        let q0 = "q0".to_string();
        let f = stateset!("q1");
        let delta = delta!(
            ("q0", 'a', "q0"),
            ("q0", 'b', "q1"),
            ("q1", 'a', "q1"),
            ("q1", 'b', "q2"),
            ("q2", 'a', "q0"),
            ("q2", 'b', "q2"),
            ("q3", 'a', "q3")
        );


        let m = M::new(k, alphabet, q0, f, delta);
        let r = vec![
            vec![true, true, true, false],
            vec![true, true, true, false],
            vec![true, true, true, false],
            vec![false, false, false, true]
        ];

        let states = get_reachable_states(&m, &r);
        let states_expected = stateset!("q0", "q1", "q2");

        assert_eq!(states, states_expected);
    }

    #[test]
    fn remove_unreachable_states_with_params_test() {
        use super::remove_unreachable_states_with_params;
        use automata::{M, to_delta_inner};

        let k = stateset!("q0", "q1", "q2", "q3");
        let alphabet = alphabet!('a', 'b');
        let q0 = "q0".to_string();
        let f = stateset!("q1");
        let delta = delta!(
            ("q0", 'a', "q0"),
            ("q0", 'b', "q1"),
            ("q1", 'a', "q1"),
            ("q1", 'b', "q2"),
            ("q2", 'a', "q0"),
            ("q2", 'b', "q2"),
            ("q3", 'a', "q3")
        );


        let m = M::new(k, alphabet, q0, f, delta);
        let reachable_states = stateset!("q0", "q1", "q2");

        let m_new = remove_unreachable_states_with_params(&m, reachable_states.clone());


        let delta_expected = delta!(
            ("q0", 'a', "q0"),
            ("q0", 'b', "q1"),
            ("q1", 'a', "q1"),
            ("q1", 'b', "q2"),
            ("q2", 'a', "q0"),
            ("q2", 'b', "q2")
        );

        let delta_expected = to_delta_inner(delta_expected);


        assert_eq!(m_new.k, reachable_states);
        assert_eq!(m_new.delta, delta_expected);
    }

    #[test]
    fn get_quotient_test() {
        use std::collections::{BTreeSet};
        use super::{Quotient, get_quotient};
        use automata::{M};

        let k = stateset!("q0", "q1", "q2", "q3");
        let alphabet = alphabet!('a', 'b');
        let q0 = "q0".to_string();
        let f = stateset!("q3");
        let delta = delta!(
            ("q0", 'a', "q1"),
            ("q0", 'b', "q2"),
            ("q1", 'a', "q3"),
            ("q2", 'a', "q3")
        );

        let m = M::new(k, alphabet, q0, f, delta);

        let quotient: Quotient = get_quotient(&m);
        let quotient_expected: Quotient = {
            let mut q = BTreeSet::new();
            q.insert(stateset!("q0"));
            q.insert(stateset!("q1", "q2"));
            q.insert(stateset!("q3"));
            q
        };



        assert_eq!(quotient, quotient_expected);
    }

    #[test]
    fn apply_quotient_test() {
        use std::collections::{BTreeSet};
        use super::{Quotient, apply_quotient};
        use automata::{M, to_delta_inner};

        let k = stateset!("q0", "q1", "q2", "q3");
        let alphabet = alphabet!('a', 'b');
        let q0 = "q0".to_string();
        let f = stateset!("q3");
        let delta = delta!(
            ("q0", 'a', "q1"),
            ("q0", 'b', "q2"),
            ("q1", 'a', "q3"),
            ("q2", 'a', "q3")
        );

        let m = M::new(k, alphabet, q0, f, delta);

        let quotient: Quotient = {
            let mut q = BTreeSet::new();
            q.insert(stateset!("q0"));
            q.insert(stateset!("q1", "q2"));
            q.insert(stateset!("q3"));
            q
        };

        let min_m = apply_quotient(&m, &quotient);

        let delta_expected = delta!(
            ("q0", 'a', "q1q2"),
            ("q0", 'b', "q1q2"),
            ("q1q2", 'a', "q3")
        );



        assert_eq!(min_m.alphabet, m.alphabet);
        assert_eq!(min_m.k, stateset!("q0", "q1q2", "q3"));
        assert_eq!(min_m.q0, "q0");
        assert_eq!(min_m.f, stateset!("q3"));
        assert_eq!(min_m.delta, to_delta_inner(delta_expected));
    }
}
