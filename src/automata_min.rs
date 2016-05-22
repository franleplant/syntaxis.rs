use automata::{M};

type RelationMatrixRow = Vec<bool>;
type RelationMatrix = Vec<RelationMatrixRow>;


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


    //# Calculate Warshal on the relation matrix
    //for k in n:
        //for i in n:
            //for j in n:
                //T[i][j] = T[i][j] or ( T[i][k] and T[k][j] )

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

}
