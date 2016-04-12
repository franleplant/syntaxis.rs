use std::collections::{BTreeSet};
use std::result;


pub type State = String;
pub type StateSet = BTreeSet<State>;
pub type Delta = BTreeSet<((State, char), State)>;
pub type Result = result::Result<(), ()>;

macro_rules! stateset {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_set: BTreeSet<String> = BTreeSet::new();
            $(
                temp_set.insert($x.to_string());
            )*
            temp_set
        }
    };
}

macro_rules! alphabet {
    ( $( $c:expr ),* ) => {
        {
            let mut alphabet_set: BTreeSet<char> = BTreeSet::new();
            $(
                alphabet_set.insert($c);
            )*
            alphabet_set
        }
    };
}

macro_rules! delta {
    ( $( (($s:expr, $c:expr), $ns:expr) ),* ) => {
        {
            let mut temp_delta: BTreeSet<((String, char), String)> = BTreeSet::new();
            $(
                temp_delta.insert( (($s.to_string(), $c), $ns.to_string()) );
            )*
            temp_delta
        }
    };
}


#[allow(dead_code)]
#[derive(Debug)]
pub struct M {
    pub k: StateSet,
    pub alphabet: BTreeSet<char>,
    pub q0: State,
    pub f: StateSet,
    pub delta: Delta,

    state: State,
}

//TODO: separate this functions in different traits
//- M it only provides the constructor and some other potential utilities
//- AFD
//- AFND ?
//- AFND-lambda ?
impl M {
    pub fn new(k: StateSet, alphabet: BTreeSet<char>, q0: State, f: StateSet, delta: Delta) -> M {
        //TODO: every alphabet has lambda implicit so always allow lambda
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
            delta: delta,
            state: q0,
        }
    }

    //TODO: from down here it should part of the AFD trait
    pub fn next(&mut self, c: char) -> Result {
        if !self.alphabet.contains(&c) {
            return Err(())
        }

        for &(ref rule, ref next_state) in &self.delta {
            if (&self.state, c) == (&rule.0, rule.1) {
                self.state = next_state.clone();
                return Ok(())
            }
        }

        Err(())
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
}

#[cfg(test)]
mod tests_fsa {
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


        let mut fsa = M::new(k, alphabet, q0, f, delta);

        assert!(fsa.check_string("ab").is_ok());
        assert!(fsa.check_string("abc").is_err());
        assert!(fsa.check_string("aaabbbabababa").is_ok());
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

        let fsa = M::new(k, alphabet, q0, f, delta);

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

        let fsa = M::new(k, alphabet, q0, f, delta);

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

        let fsa = M::new(k, alphabet, q0, f, delta);

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


        let fsa = M::new(k, alphabet, q0, f, delta);

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


        let fsa = M::new(k, alphabet, q0, f, delta);

        assert!(true);
    }
}
