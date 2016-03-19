#[macro_use] pub mod fsa;


use fsa::{M, State};
use std::collections::HashSet;


pub fn lambda_closure(q: &HashSet<State>, m: &M) -> HashSet<State>{
    let mut closure: HashSet<State> = q.clone();
    let mut marked: HashSet<State> = HashSet::new();

    // while closure and marked are not equal
    while !(closure.is_subset(&marked) && marked.is_subset(&closure)) {
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

//TODO: snippet to create macro state U in later algorithms
//let mut closure_vec = closure.iter().map(|s| s.clone()).collect::<Vec<State>>();
//closure_vec.sort();
//l.insert(closure_vec.join(""));

pub fn mover(q: &HashSet<State>, a: char, m: &M) -> HashSet<State> {
    let mut x = HashSet::new();
    for t in q.iter() {
        for &((ref s, c), ref ns) in m.delta.iter() {
            if s == t && c == a {
                x.insert(ns.clone());
            }
        }
    }

    lambda_closure(&x, &m)
}


//TODO: add more tests
#[cfg(test)]
mod tests {
    use fsa::M;
    use super::{lambda_closure, mover};
    use std::collections::HashSet;

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

        let fsa = M::new(k, alphabet, q0, f, delta);

        let q = stateset!("q0");
        let expected = stateset!("q0", "q2");
        let actual = lambda_closure(&q, &fsa);
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

        let fsa = M::new(k, alphabet, q0, f, delta);


        let q = stateset!("q0");
        let expected = stateset!("q1", "q2");
        let actual = mover(&q, 'a', &fsa);
        assert!(actual.is_subset(&expected) && expected.is_subset(&actual));
    }
}


