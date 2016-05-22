use std::collections::{BTreeSet};
use automata::{M, StateSet, State};

pub type RelationMatrixRow = Vec<bool>;
pub type RelationMatrix = Vec<RelationMatrixRow>;


pub fn get_relation_matrix(m: &M) -> RelationMatrix {
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


pub fn warshall(matrix: &RelationMatrix) -> RelationMatrix {
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

pub fn get_reachable_states(m: &M, r: &RelationMatrix) -> StateSet {
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
        //for i in rm {
            //println!("{:?}", i);
        //}

        let rm_expected = vec![
            vec![true, true, false, false],
            vec![false, true, true, false],
            vec![true, false, true, false],
            vec![false, false, false, true]
        ];

        assert!(rm == rm_expected);
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

        //for i in &r {
            //println!("{:?}", i);
        //}

        assert!(r == r_expected);
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

        //for i in &states {
            //println!("{:?}", i);
        //}

        assert!(states == states_expected);
    }
}
