use automata::{M, State, StateSet, Alphabet, Delta};


fn prefix_state(prefix: &String, s: &State) -> String {
    let mut name: String = prefix.clone();
    name.push_str(s);
    name
}

fn prefix_automata(prefix: &String, m: &M) -> M {
    let q0: State = prefix_state(&prefix, &m.q0);
    let f: StateSet = m.f.iter().cloned().map(|s| prefix_state(&prefix, &s)).collect();
    let k: StateSet = m.k.iter().cloned().map(|s| prefix_state(&prefix, &s)).collect();

    let mut alphabet = alphabet!();
    let mut delta = delta!();

    //TODO abstract the way we iter over delta
    for (state, delta_value) in &m.delta {
        for (a, next_states) in delta_value {
            for next_state in next_states {
                let s = prefix_state(&prefix, &state.clone());
                let c = a.clone();
                let ns = prefix_state(&prefix, &next_state.clone());
                delta.insert( (s, c, ns) );
            }
        }
    }

    M::new(k, m.alphabet.clone(), q0, f, delta)
}

fn automata_union(m1: &M, m2: &M, prefix: String) -> M {
    let m1_prefix: String = {
        let mut p = prefix.clone();
        p.push_str("1");
        p
    };
    let m2_prefix: String = {
        let mut p = prefix.clone();
        p.push_str("2");
        p
    };

    let prefixed_m1: M = prefix_automata(&m1_prefix, &m1);
    let prefixed_m2: M = prefix_automata(&m2_prefix, &m2);

    let q0 = prefix_state(&prefix, &"q0".to_string());
    let f = prefix_state(&prefix, &"f0".to_string());
    let f_set = stateset!(f);
    let mut alphabet: Alphabet = m1.alphabet.union(&m2.alphabet).cloned().collect();
    let mut delta: Delta = delta!();

    let k: StateSet ={
        let mut k = stateset!(q0.clone(), f.clone());
        k = k.union(&prefixed_m1.k).cloned().collect::<StateSet>().union(&prefixed_m2.k).cloned().collect::<StateSet>();
        k
    };

    let f1: State =  prefixed_m1.f.iter().cloned().take(1).collect();
    let f2: State =  prefixed_m2.f.iter().cloned().take(1).collect();


    delta.insert( (q0.clone(), 'λ', prefixed_m1.q0.clone()) );
    delta.insert( (q0.clone(), 'λ', prefixed_m2.q0.clone()) );
    delta.insert( (f1.clone(), 'λ', f.clone()) );
    delta.insert( (f2.clone(), 'λ', f.clone()) );

    for (state, delta_value) in &prefixed_m1.delta {
        if prefixed_m1.f.contains(state) { continue };
        for (a, next_states) in delta_value {
            for next_state in next_states {
                delta.insert( (state.clone(), a.clone(), next_state.clone()) );
            }
        }
    }
    for (state, delta_value) in &prefixed_m2.delta {
        if prefixed_m2.f.contains(state) { continue };
        for (a, next_states) in delta_value {
            for next_state in next_states {
                delta.insert( (state.clone(), a.clone(), next_state.clone()) );
            }
        }
    }

    println!("{:?}", k);
    println!("{:?}", delta);
    println!("{:?}", q0);
    println!("{:?}", f_set);
    println!("{:?}", delta);

    M::new(k, alphabet, q0, f_set, delta)
}

fn automata_intersection(m1: &M, m2: &M) -> M {
    unimplemented!();
}

fn automata_star(m1: &M) -> M {
    unimplemented!();
}

//pub fn regexp(s: String) -> M {
    //let chain_v: Vec<char> = s.chars().collect();
    //if (chain_v.len() == 0) {
        ////improve states names
        //let m: M = M::new(
            //stateset!("q0", "q1"),
            //alphabet!(),
            //"q0".to_string(),
            //stateset!("q1"),
            //delta!()
        //);

        //return m;
    //} else if (chain_v.len() == 1 && chain_v[0] == 'λ') {
        //let m: M = M::new(
            //stateset!("q0"),
            //alphabet!(),
            //"q0".to_string(),
            //stateset!("q0"),
            //delta!()
        //);

        //return m;
    //}  else if (chain_v.len() == 1) {
        //let m: M = M::new(
            //stateset!("q0", "q1"),
            //alphabet!(chain_v[0]),
            //"q0".to_string(),
            //stateset!("q1"),
            //delta!(("q0", chain_v[0], "q1"))
        //);

        //return m;

    //} else {
        ////TODO: Abtract constants
        //let q0 = "0q0".to_string();
        //let f = "0f0".to_string();
        //let mut k = stateset!(q0.clone(), f.clone());
        //let mut alphabet = alphabet!();
        //let f_set = stateset!(f);
        //let mut delta = delta!();

        ////TODO Need to create independent state names
        //let mut index: u32 = 1;
        //for m in s.split("|").map(|r| regexp(r.to_string())) {
            //alphabet = alphabet.union(&m.alphabet).cloned().collect();
            //for e in m.k {
                //let mut q_name = index.to_string();
                //q_name.push_str(&e);
                //k.insert(q_name);
            //}

            //let mq0 = prefix_state(index, m.q0.clone());

            //let f_i = {
                //let mut aux: String = index.to_string();
                //let f: State = m.f.iter().cloned().take(1).collect();
                //aux.push_str(&f.clone());
                //aux
            //};

            //delta.insert( (q0.clone(), 'λ', mq0) );
            //delta.insert( (f_i, 'λ', f.clone()) );

            //for (state, delta_value) in &m.delta {
                //if m.f.contains(&state) { continue };
                //for (a, next_states) in delta_value {
                    //for next_state in next_states {
                        //delta.insert( (prefix_state(index, state.clone()), a.clone(), prefix_state(index, next_state.clone())) );
                    //}
                //}
            //}
            //index += 1;
        //}
        //return M::new(k, alphabet, q0, f_set, delta);
    //}

    //M::new(
        //stateset!("q0", "q1"),
        //alphabet!(),
        //"q0".to_string(),
        //stateset!("q1"),
        //delta!()
    //)
//}

#[cfg(test)]
mod tests {
    //use super::regexp;
    use automata::M;

    //#[test]
    //fn base_case_empty() {
        //let m = regexp("".to_string());

        //let m_expected = M::new(
            //stateset!("q0", "q1"),
            //alphabet!(),
            //"q0".to_string(),
            //stateset!("q1"),
            //delta!()
        //);

        //assert_eq!(m, m_expected)
    //}

    //#[test]
    //fn base_case_lambda() {
        //let m = regexp("λ".to_string());

        //let m_expected = M::new(
            //stateset!("q0"),
            //alphabet!(),
            //"q0".to_string(),
            //stateset!("q0"),
            //delta!()
        //);

        //assert_eq!(m, m_expected)
    //}

    //#[test]
    //fn base_case_single_char() {
        //let m = regexp("a".to_string());

        //let m_expected = M::new(
            //stateset!("q0", "q1"),
            //alphabet!('a'),
            //"q0".to_string(),
            //stateset!("q1"),
            //delta!(("q0", 'a', "q1"))
        //);

        //assert_eq!(m, m_expected)
    //}

    #[test]
    fn prefix_automata_test() {
        use super::prefix_automata;

        let m = M::new(
            stateset!("q0", "q1"),
            alphabet!('a'),
            "q0".to_string(),
            stateset!("q1"),
            delta!(("q0", 'a', "q1"))
        );

        let m_expected = M::new(
            stateset!("Aq0", "Aq1"),
            alphabet!('a'),
            "Aq0".to_string(),
            stateset!("Aq1"),
            delta!(("Aq0", 'a', "Aq1"))
        );

        let m_actual = prefix_automata(&"A".to_string(), &m);

        assert_eq!(m_actual, m_expected)
    }

    #[test]
    fn union_test() {
        use super::automata_union;

        let m1 = M::new(
            stateset!("q0", "q1"),
            alphabet!('a'),
            "q0".to_string(),
            stateset!("q1"),
            delta!(("q0", 'a', "q1"))
        );

        let m2 = M::new(
            stateset!("q0", "q1"),
            alphabet!('b'),
            "q0".to_string(),
            stateset!("q1"),
            delta!(("q0", 'b', "q1"))
        );

        let m_expected = M::new(
            stateset!("0q0", "0f0", "01q0", "01q1", "02q0", "02q1"),
            alphabet!('a', 'b'),
            "0q0".to_string(),
            stateset!("0f0"),
            delta!(
                ("0q0", 'λ', "01q0"),
                ("0q0", 'λ', "02q0"),

                ("01q0", 'a', "01q1"),
                ("02q0", 'b', "02q1"),

                ("01q1", 'λ', "0f0"),
                ("02q1", 'λ', "0f0")
            )
        );

        let m = automata_union(&m1, &m2, "0".to_string());

        assert_eq!(m, m_expected)
    }
}
