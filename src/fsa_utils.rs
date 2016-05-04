use automata::{M, State, StateSet};
use std::collections::{BTreeSet};
use std::cmp::Ord;


pub fn btreeset_eq<T: Ord>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> bool {
    a.is_subset(&b) && b.is_subset(&a)
}

pub fn stateset_name(states: &StateSet) -> String {
    let states_vec: Vec<State> = states.iter().cloned().collect();
    states_vec.join("")
}


pub fn lambda_closure(q: &StateSet, m: &M) -> StateSet{
    let mut closure: StateSet = q.clone();
    let mut marked: StateSet = BTreeSet::new();

    // while closure and marked are not equal
    while !btreeset_eq(&closure, &marked) {
        let l = closure.clone();
        for t in l.iter() {
            if marked.contains(t) {
                continue
            }
            marked.insert(t.clone());
            for &((ref s, c), ref ns) in m.delta.iter() {
                if s == t && c == 'λ' {
                    closure.insert(ns.clone());
                }
            }
        }
    }

    closure
}


pub fn mover(q: &StateSet, a: char, m: &M) -> StateSet {
    let mut x = BTreeSet::new();
    for t in q.iter() {
        for &((ref s, c), ref ns) in m.delta.iter() {
            if s == t && c == a {
                x.insert(ns.clone());
            }
        }
    }

    lambda_closure(&x, &m)
}



pub fn afndl_to_afd(m: &M) -> M {
    let q0: StateSet = lambda_closure(&stateset!(m.q0), &m);
    let mut k: BTreeSet<StateSet> = BTreeSet::new();
    let q0_str = stateset_name(&q0);
    k.insert(q0);
    let mut f = stateset!();
    let mut delta = delta!();
    let mut marked = BTreeSet::new();

    // while k and marked are not equal
    while !btreeset_eq(&k, &marked) {
        //t: StateSet
        for t in k.clone().difference(&marked.clone()) {
            marked.insert(t.clone());
            for a in m.alphabet.iter() {
                let u = mover(&t, *a, &m);
                if u.is_empty() { continue; }
                println!("u {:?}", u);

                let intersection: StateSet = u.intersection(&m.f).cloned().collect();
                if !intersection.is_empty() {
                    f.insert(stateset_name(&u));
                }

                k.insert(u.clone());
                delta.insert( ((stateset_name(&t), *a), stateset_name(&u)) );
            }
        }
    }


    let k = k.iter().map(|set| stateset_name(set)).collect::<StateSet>();
    M::new(k, m.alphabet.clone(), q0_str, f, delta)
}


//TODO: add more tests
#[cfg(test)]
mod tests {
    use automata::M;
    use super::{btreeset_eq, lambda_closure, mover, afndl_to_afd};
    use std::collections::{BTreeSet};

    #[test]
    fn lambda_closure_test() {
        let k = stateset!("q0", "q1", "q2");
        let alphabet = alphabet!('b');
        let q0 = "q0".to_string();
        let f = stateset!("q1");
        let delta = delta!(
            (("q0", 'λ'), "q2"),
            (("q0", 'b'), "q1")
        );

        let automata = M::new(k, alphabet, q0, f, delta);

        let q = stateset!("q0");
        let expected = stateset!("q0", "q2");
        let actual = lambda_closure(&q, &automata);
        assert!(actual.is_subset(&expected) && expected.is_subset(&actual));
    }

    #[test]
    fn mover_test() {
        let k = stateset!("q0", "q1", "q2");
        let alphabet = alphabet!('a', 'b');
        let q0 = "q0".to_string();
        let f = stateset!("q1");
        let delta = delta!(
            (("q0", 'a'), "q1"),
            (("q0", 'b'), "q0"),
            (("q1", 'λ'), "q2"),
            (("q1", 'b'), "q1")
        );

        let automata = M::new(k, alphabet, q0, f, delta);


        let q = stateset!("q0");
        let expected = stateset!("q1", "q2");
        let actual = mover(&q, 'a', &automata);
        assert!(actual.is_subset(&expected) && expected.is_subset(&actual));
    }


    #[test]
    fn afndl_to_afd_test() {
        let k = stateset!("q0", "q1", "q2", "q3", "q4", "q5");
        let alphabet = alphabet!('a', 'b');
        let q0 = "q0".to_string();
        let f = stateset!("q5");
        let delta = delta!(
            (("q0", 'a'), "q1"),
            (("q0", 'a'), "q2"),
            (("q1", 'b'), "q3"),
            (("q2", 'a'), "q4"),
            (("q3", 'λ'), "q2"),
            (("q4", 'λ'), "q3"),
            (("q4", 'b'), "q5")
        );

        let afndl = M::new(k, alphabet, q0, f, delta);

        let afd: M = afndl_to_afd(&afndl);
        println!("{:?}", afd);

        let k_expected = stateset!("q0", "q1q2", "q2q3q4", "q2q3", "q5");
        assert!( btreeset_eq(&afd.k, &k_expected) );

        assert!( btreeset_eq(&afd.alphabet, &afndl.alphabet) );

        let f_expected = stateset!("q5");
        assert!( btreeset_eq(&afd.f, &f_expected) );

        assert!(afd.q0 == "q0");

        let delta_expected = delta!(
            (("q0", 'a'), "q1q2"),
            (("q1q2", 'a'), "q2q3q4"),
            (("q1q2", 'b'), "q2q3"),
            (("q2q3", 'a'), "q2q3q4"),
            (("q2q3q4", 'a'), "q2q3q4"),
            (("q2q3q4", 'b'), "q5")
        );

        assert!( btreeset_eq(&afd.delta, &delta_expected) );
    }
}

