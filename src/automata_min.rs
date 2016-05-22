use automata::{M};

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

#[cfg(test)]
mod tests {
    #[test]
    fn get_relation_matrix_test() {
        use super::get_relation_matrix;
        use automata::M;

        let k = stateset!("q0", "q1", "q2");
        let alphabet = alphabet!('a', 'b');
        let q0 = "q0".to_string();
        let f = stateset!("q1");
        let delta = delta!(
            ("q0", 'a', "q1"),
            ("q0", 'b', "q0"),
            ("q1", 'a', "q0"),
            ("q1", 'b', "q1"),
            ("q2", 'a', "q2")
        );


        let m = M::new(k, alphabet, q0, f, delta);

        let rm = get_relation_matrix(&m);
        //for i in rm {
            //println!("{:?}", i);
        //}

        let rm_expected = vec![
            [true, true, false],
            [true, true, false],
            [false, false, true]
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
        for i in r {
            println!("{:?}", i);
        }

    }
}
