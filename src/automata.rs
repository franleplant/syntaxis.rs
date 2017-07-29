use std::collections::{BTreeSet, BTreeMap};
use std::result;

pub type State = String;
pub type StateSet = BTreeSet<State>;
pub type Delta = BTreeSet<(State, char, State)>;
pub type DeltaValue = BTreeMap<char, StateSet>;
pub type DeltaMap = BTreeMap<State, DeltaValue>;
pub type Alphabet = BTreeSet<char>;
pub type Result = result::Result<(), ()>;


pub static TRAP_STATE: &'static str = "trap_state";


//TODO test
//TODO: rething if the parameter should be an automata or a DeltaMap
pub fn to_delta(m: &M) -> Delta {
    let mut delta: Delta = delta!();
    for (state, delta_value) in &m.delta {
        for (a, next_states) in delta_value {
            for next_state in next_states {
                delta.insert((state.clone(), a.clone(), next_state.clone()));
            }
        }
    }

    delta
}


pub fn to_delta_inner(delta_input: Delta) -> DeltaMap {
    let next_states_blueprint: StateSet = BTreeSet::new();
    let delta_value_blueprint: DeltaValue = BTreeMap::new();
    let mut delta: DeltaMap = BTreeMap::new();

    for &(ref s, a, ref ns) in delta_input.iter() {
        let mut delta_value: DeltaValue = match delta.get(s) {
            Some(delta_value) => delta_value.clone(),
            None => delta_value_blueprint.clone(),
        };

        let mut next_states: StateSet = match delta_value.get(&a) {
            Some(next_states) => next_states.clone(),
            None => next_states_blueprint.clone(),
        };

        next_states.insert(ns.clone());
        delta_value.insert(a, next_states);
        delta.insert(s.clone(), delta_value);
    }

    delta
}


pub fn print_delta(delta: &DeltaMap) {
    println!("");
    for (key, value) in delta.iter() {
        println!("{:<20}    {:?}", key, value)
    }
    println!("");
}

pub fn print_automata(m: &M) {
    println!("AUTOMATA");
    println!("K:  {:?}", m.k);
    println!("q0: {:?}", m.q0);
    println!("f:  {:?}", m.f);
    println!("a:  {:?}", m.alphabet);
    print_delta(&m.delta);

}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M {
    pub k: StateSet,
    pub alphabet: BTreeSet<char>,
    pub q0: State,
    pub f: StateSet,
    pub delta: DeltaMap,

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
        for &(ref current_state, c, ref next_state) in &delta {
            if !k.contains(current_state) {
                panic!("Delta is incorrect. In {:?} rule, \"{}\" does not belong to K",
                       (current_state, c, next_state),
                       current_state)
            }

            if !k.contains(next_state) {
                panic!("Delta is incorrect. In {:?} rule, \"{}\" does not belong to K",
                       (current_state, c, next_state),
                       next_state)
            }

            if c != 'λ' && !alphabet.contains(&c) {
                panic!("Delta is incorrect. In {:?} rule, '{}' does not belong to Alphabet",
                       (current_state, c, next_state),
                       c)
            }
        }

        let delta = to_delta_inner(delta);


        M {
            k: k,
            alphabet: alphabet,
            q0: q0.clone(),
            f: f,
            delta: delta,
            state: q0,
        }
    }

    pub fn next(&mut self, c: char) {
        let next_states = self.get_next_states(&self.state, &c);

        if next_states.len() > 1 {
            println!("None determinist automata: found more than one next state for a given state and char",);
        }

        for next_state in next_states.iter().take(1) {
            self.state = next_state.clone();
        }
    }

    pub fn end(&mut self) -> Result {
        let mut success = false;

        if self.f.contains(&self.state) {
            success = true;
        }

        //Reset the automata state
        self.state = self.q0.clone();

        if success { Ok(()) } else { Err(()) }
    }

    pub fn check_string(&mut self, string: &'static str) -> Result {
        self.state = self.q0.clone();

        for c in String::from(string).chars() {
            self.next(c);
        }

        self.end()
    }

    pub fn get_next_states(&self, state: &State, a: &char) -> StateSet {
        if let Some(delta_value) = self.delta.get(state) {
            if let Some(next_states) = delta_value.get(a) {
                return next_states.clone();
            }
        }

        stateset!(TRAP_STATE)
    }
}

#[cfg(test)]
mod tests_automata {
    use super::M;

    #[test]
    fn basic_functionality() {

        let k = stateset!("q0", "q1");
        let alphabet = alphabet!('a', 'b');
        let q0 = "q0".to_string();
        let f = stateset!("q1");
        let delta = delta!(("q0", 'a', "q1"),
                           ("q0", 'b', "q0"),
                           ("q1", 'a', "q0"),
                           ("q1", 'b', "q1"));


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
        let delta = delta!(("q0", 'a', "q1"),
                           ("q0", 'b', "q0"),
                           ("q1", 'a', "q0"),
                           ("q1", 'b', "q1"));


        let automata = M::new(k, alphabet, q0, f, delta);

        let ns = automata.get_next_states(&"q0".to_string(), &'a');
        assert_eq!(ns, stateset!("q1"));
    }

    #[test]
    #[should_panic]
    fn panics_q0() {
        let k = stateset!("q0");
        let alphabet = alphabet!('a');
        let q0 = "not_valid".to_string();
        let f = stateset!();
        let delta = delta!(("q0", 'a', "q0"));

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
        let delta = delta!(("q0", 'a', "q0"));

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
        let delta = delta!(("q1", 'a', "q0"));

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
        let delta = delta!(("q0", 'z', "q0"));


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
        let delta = delta!(("q0", 'a', "invalid"));


        M::new(k, alphabet, q0, f, delta);

        assert!(true);
    }

    #[test]
    fn test_to_delta_inner() {
        //TODO: improve tests

        use super::{DeltaValue, to_delta_inner};

        //let states = stateset!("q0", "q1", "q2");
        //let alphabet = alphabet!('a', 'b');

        let delta = delta!(("q0", 'a', "q1"),
                           ("q0", 'a', "q2"),
                           ("q0", 'b', "q1"),
                           ("q1", 'a', "q2"));

        let delta_inner = to_delta_inner(delta);
        //println!("asdasdasd {:?}", delta_inner);

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
