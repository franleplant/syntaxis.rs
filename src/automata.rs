use std::collections::{BTreeSet, BTreeMap};
use std::result;


pub type State = String;
pub type StateSet = BTreeSet<State>;
pub type Delta = BTreeSet<((State, char), State)>;
pub type DeltaValue = BTreeMap<char, StateSet>;
pub type DeltaInner = BTreeMap<State, DeltaValue>;
pub type Result = result::Result<(), ()>;


fn to_delta_inner(delta_input: Delta) -> DeltaInner {
    let mut delta: DeltaInner = BTreeMap::new();

    for &((ref s, a), ref ns) in delta_input.iter() {

        let mut delta_value = match delta.get(s) {
            Some(value) => value.clone(),
            None => BTreeMap::new()
        };

        let mut next_states = match delta_value.get(&a) {
            Some(states) => states.clone(),
            None => BTreeSet::new()
        };

        next_states.insert(ns.clone());
        delta_value.insert(a, next_states);
        delta.insert(s.clone(), delta_value);
    }
    println!("{:?}", delta);

    delta
}


#[allow(dead_code)]
#[derive(Debug)]
pub struct M {
    pub k: StateSet,
    pub alphabet: BTreeSet<char>,
    pub q0: State,
    pub f: StateSet,
    pub delta: DeltaInner,

    state: State,
}

impl M {
    pub fn new(k: StateSet, alphabet: BTreeSet<char>, q0: State, f: StateSet, delta: Delta) -> M {
        //TODO: if delta has lambda transitions then dont allow to check string!

        // Check that q0 belongs to K
        if !k.contains(&q0) {
            panic!("q0 must belong to K")
        }

        // Check that F is subset of K
        if !f.is_subset(&k) {
            panic!("F must be a subset of K")
        }

        // Check that each element of delta belongs to either K or Alphabet
        for &((ref current_state, c), ref next_state) in &delta {
            if !k.contains(current_state) {
                panic!("Delta is incorrect. In {:?} rule, \"{}\" does not belong to K", ((current_state, c), next_state), current_state)
            }

            if !k.contains(next_state) {
                panic!("Delta is incorrect. In {:?} rule, \"{}\" does not belong to K", ((current_state, c), next_state), next_state)
            }

            if c != 'λ' && !alphabet.contains(&c) {
                panic!("Delta is incorrect. In {:?} rule, '{}' does not belong to Alphabet", ((current_state, c), next_state), c)
            }
        }


        M {
            k: k,
            alphabet: alphabet,
            q0: q0.clone(),
            f: f,
            delta: to_delta_inner(delta),
            state: q0,
        }
    }

    pub fn next(&mut self, c: char) -> Result {
        if !self.alphabet.contains(&c) {
            return Err(())
        }

        let next_states = try!(self.get_next_states(&self.state, &c).ok_or( () ));

        if next_states.len() > 1 {
            println!("None determinist automata: found more than one next state for a given state and char");
        }

        for next_state in next_states.iter().take(1) {
            self.state = next_state.clone();
        }

        Ok(())
    }

    pub fn end(&mut self) -> Result {
        let mut success = false;

        if self.f.contains(&self.state) {
            success = true;
        }

        self.state = self.q0.clone();

        if success {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn check_string(&mut self, string: &'static str) -> Result {
        self.state = self.q0.clone();

        for c in String::from(string).chars() {
            let res = self.next(c);
            if res.is_err() {
                return Err(())
            }
        }

        self.end()
    }

    pub fn get_next_states(&self, state: &State, a: &char) -> Option<StateSet> {
        if let Some(delta_value) = self.delta.get(state) {
            if let Some(next_states) = delta_value.get(a) {
                return Some(next_states.clone())
            }
        }

        None
    }
}

#[cfg(test)]
mod tests_automata {
    use super::M;
    use std::collections::BTreeSet;

    #[test]
    fn basic_functionality() {

        let k = stateset!("q0", "q1");
        let alphabet = alphabet!('a', 'b');
        let q0 = "q0".to_string();
        let f = stateset!("q1");
        let delta = delta!(
            (("q0", 'a'), "q1"),
            (("q0", 'b'), "q0"),
            (("q1", 'a'), "q0"),
            (("q1", 'b'), "q1")
        );


        let mut automata = M::new(k, alphabet, q0, f, delta);

        assert!(automata.check_string("ab").is_ok());
        assert!(automata.check_string("abc").is_err());
        assert!(automata.check_string("aaabbbabababa").is_ok());
    }

    #[test]
    fn basic_get_next_states() {
        let k = stateset!("q0", "q1");
        let alphabet = alphabet!('a', 'b');
        let q0 = "q0".to_string();
        let f = stateset!("q1");
        let delta = delta!(
            (("q0", 'a'), "q1"),
            (("q0", 'b'), "q0"),
            (("q1", 'a'), "q0"),
            (("q1", 'b'), "q1")
        );


        let automata = M::new(k, alphabet, q0, f, delta);

        let ns = automata.get_next_states(&"q0".to_string(), &'a').unwrap();
        assert!(ns.contains(&"q1".to_string()));
    }

    #[test]
    #[should_panic]
    fn panics_q0() {
        let k = stateset!("q0");
        let alphabet = alphabet!('a');
        let q0 = "not_valid".to_string();
        let f = stateset!();
        let delta = delta!(
            (("q0", 'a'), "q0")
        );

        M::new(k, alphabet, q0, f, delta);

        assert!(true);
    }

    #[test]
    #[should_panic]
    fn panics_f() {
        let k = stateset!("q0");
        let alphabet = alphabet!('a');
        let q0 = "q0".to_string();
        let f = stateset!("not_valid");
        let delta = delta!(
            (("q0", 'a'), "q0")
        );

        M::new(k, alphabet, q0, f, delta);

        assert!(true);
    }


    #[test]
    #[should_panic]
    fn panics_delta_1() {
        let k = stateset!("q0");
        let alphabet = alphabet!('a');
        let q0 = "q0".to_string();
        let f = stateset!("q0");
        let delta = delta!(
            (("q1", 'a'), "q0")
        );

        M::new(k, alphabet, q0, f, delta);

        assert!(true);
    }


    #[test]
    #[should_panic]
    fn panics_delta_2() {
        let k = stateset!("q0");
        let alphabet = alphabet!('a');
        let q0 = "q0".to_string();
        let f = stateset!("q0");
        let delta = delta!(
            (("q0", 'z'), "q0")
        );


        M::new(k, alphabet, q0, f, delta);

        assert!(true);
    }

    #[test]
    #[should_panic]
    fn panics_delta_3() {
        let k = stateset!("q0");
        let alphabet = alphabet!('a');
        let q0 = "q0".to_string();
        let f = stateset!("q0");
        let delta = delta!(
            (("q0", 'a'), "invalid")
        );


        M::new(k, alphabet, q0, f, delta);

        assert!(true);
    }

    #[test]
    fn test_to_delta_inner() {

        use super::{DeltaValue, to_delta_inner};

        let delta = delta!(
            (("q0", 'a'), "q1"),
            (("q0", 'a'), "q2"),
            (("q0", 'b'), "q1"),
            (("q1", 'a'), "q2")
        );

        let delta_inner = to_delta_inner(delta);

        assert!(delta_inner.contains_key(&"q0".to_string()));
        assert!(delta_inner.contains_key(&"q1".to_string()));

        let q0_value: &DeltaValue = delta_inner.get(&"q0".to_string()).unwrap();
        let q1_value: &DeltaValue = delta_inner.get(&"q1".to_string()).unwrap();

        assert!(q0_value.contains_key(&'a'));
        assert!(q0_value.contains_key(&'b'));
        assert!(q1_value.contains_key(&'a'));

        let q0_a_value = q0_value.get(&'a').unwrap();
        assert!(q0_a_value.contains(&"q1".to_string()));
        assert!(q0_a_value.contains(&"q2".to_string()));

        let q0_b_value = q0_value.get(&'b').unwrap();
        assert!(q0_b_value.contains(&"q1".to_string()));

        let q1_a_value = q1_value.get(&'a').unwrap();
        assert!(q1_a_value.contains(&"q2".to_string()));
    }
}
