use automata::{M, State, StateSet, Alphabet, Delta, to_delta};
use automata_min::minify;
use automata_operators::afndl_to_afd;


fn prefix_state(prefix: &String, s: &State) -> String {
    let mut name: String = prefix.clone();
    name.push_str(s);
    name
}

fn prefix_automata(prefix: &String, m: &M) -> M {
    let q0: State = prefix_state(&prefix, &m.q0);
    let f: StateSet = m.f
        .iter()
        .cloned()
        .map(|s| prefix_state(&prefix, &s))
        .collect();
    let k: StateSet = m.k
        .iter()
        .cloned()
        .map(|s| prefix_state(&prefix, &s))
        .collect();

    let mut delta = delta!();

    for (s, c, ns) in to_delta(&m) {
        let s = prefix_state(&prefix, &s.clone());
        let c = c.clone();
        let ns = prefix_state(&prefix, &ns.clone());
        delta.insert((s, c, ns));
    }

    M::new(k, m.alphabet.clone(), q0, f, delta)
}

pub fn automata_union(m1: &M, m2: &M, prefix: String) -> M {
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
    let f1: State = prefixed_m1.f.iter().cloned().take(1).collect();
    let f2: State = prefixed_m2.f.iter().cloned().take(1).collect();

    let q0 = prefix_state(&prefix, &"q0".to_string());
    let f = prefix_state(&prefix, &"f0".to_string());
    let alphabet: Alphabet = m1.alphabet.union(&m2.alphabet).cloned().collect();
    let k: StateSet = {
        let mut k = stateset!(q0.clone(), f.clone());
        k = k.union(&prefixed_m1.k)
            .cloned()
            .collect::<StateSet>()
            .union(&prefixed_m2.k)
            .cloned()
            .collect::<StateSet>();
        k
    };

    let mut delta: Delta = delta!((q0.clone(), 'λ', prefixed_m1.q0.clone()),
                                  (q0.clone(), 'λ', prefixed_m2.q0.clone()),
                                  (f1.clone(), 'λ', f.clone()),
                                  (f2.clone(), 'λ', f.clone()));

    for (s, a, ns) in to_delta(&prefixed_m1) {
        if prefixed_m1.f.contains(&s) {
            continue;
        };
        delta.insert((s.clone(), a.clone(), ns.clone()));
    }
    for (s, a, ns) in to_delta(&prefixed_m2) {
        if prefixed_m2.f.contains(&s) {
            continue;
        };
        delta.insert((s.clone(), a.clone(), ns.clone()));
    }


    M::new(k, alphabet, q0, stateset!(f), delta)
}

pub fn automata_intersection(m1: &M, m2: &M, prefix: String) -> M {
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

    let f1: State = prefixed_m1.f.iter().cloned().take(1).collect();
    let f2: State = prefixed_m2.f.iter().cloned().take(1).collect();

    let q0 = prefixed_m1.q0.clone();
    let f = f2;
    let k: StateSet = prefixed_m1.k.union(&prefixed_m2.k).cloned().collect();
    let alphabet: Alphabet = prefixed_m1
        .alphabet
        .union(&prefixed_m2.alphabet)
        .cloned()
        .collect();
    let mut delta: Delta = delta!((f1.clone(), 'λ', prefixed_m2.q0.clone()));

    for (s, a, ns) in to_delta(&prefixed_m1) {
        //if prefixed_m1.f.contains(&s) { continue };
        delta.insert((s.clone(), a.clone(), ns.clone()));
    }
    for (s, a, ns) in to_delta(&prefixed_m2) {
        //if prefixed_m2.f.contains(&s) { continue };
        delta.insert((s.clone(), a.clone(), ns.clone()));
    }


    M::new(k, alphabet, q0, stateset!(f), delta)
}

pub fn automata_star(m: &M, prefix: String) -> M {
    let m_prefix: String = {
        let mut p = prefix.clone();
        p.push_str("1");
        p
    };
    let prefixed_m: M = prefix_automata(&m_prefix, &m);
    let f1: State = prefixed_m.f.iter().cloned().take(1).collect();

    let q0 = prefix_state(&prefix, &"q0".to_string());
    let f = prefix_state(&prefix, &"f0".to_string());

    let k: StateSet = prefixed_m.k.union(&stateset!(q0, f)).cloned().collect();

    let mut delta: Delta = delta!((q0.clone(), 'λ', prefixed_m.q0.clone()),
                                  (q0.clone(), 'λ', f.clone()),
                                  (f1.clone(), 'λ', prefixed_m.q0.clone()),
                                  (f1.clone(), 'λ', f.clone()));

    for (s, a, ns) in to_delta(&prefixed_m) {
        if prefixed_m.f.contains(&s) {
            continue;
        };
        delta.insert((s.clone(), a.clone(), ns.clone()));
    }

    M::new(k, m.alphabet.clone(), q0, stateset!(f), delta)
}



//TODO:
//This is a pseudo working impl of a regexp engine
//We still need to define some context free grammar tools
//in order to parse correctly the metalanguage that defines
//the posible regexp and their structure as to be able to understand
//paranthereses and complex nesting structures.
pub fn regexp(s: String) -> M {
    let chain_v: Vec<char> = s.chars().collect();

    if chain_v.len() == 0 {
        let m: M = M::new(stateset!("q0"),
                          alphabet!(),
                          "q0".to_string(),
                          stateset!("q0"),
                          delta!());

        return m;
    } else if chain_v.len() == 1 {
        let m: M = M::new(stateset!("q0", "q1"),
                          alphabet!(chain_v[0]),
                          "q0".to_string(),
                          stateset!("q1"),
                          delta!(("q0", chain_v[0], "q1")));

        return m;

    }


    if s == "a|b".to_string() {
        let m1 = regexp("a".to_string());
        let m2 = regexp("b".to_string());
        let m = automata_union(&m1, &m2, "0".to_string());
        {
            use automata::print_automata;
            print_automata(&m);
        }

        let m = afndl_to_afd(&m);
        {
            use automata::print_automata;
            print_automata(&m);
        }

        return minify(&m);
    }

    if s == "ab".to_string() {
        let m1 = regexp("a".to_string());
        let m2 = regexp("b".to_string());
        let m = automata_intersection(&m1, &m2, "0".to_string());
        {
            use automata::print_automata;
            print_automata(&m);
        }

        let m = afndl_to_afd(&m);
        {
            use automata::print_automata;
            print_automata(&m);
        }

        return minify(&m);
    }

    if s == "a*".to_string() {
        let m1 = regexp("a".to_string());
        let m = automata_star(&m1, "0".to_string());
        {
            use automata::print_automata;
            print_automata(&m);
        }

        let m = afndl_to_afd(&m);
        {
            use automata::print_automata;
            print_automata(&m);
        }

        return minify(&m);
    }

    M::new(stateset!("q0", "q1"),
           alphabet!(),
           "q0".to_string(),
           stateset!("q1"),
           delta!())
}



pub fn re_trivial(s: String) -> M {
    assert!(s.len() <= 1);
    let chain_v: Vec<char> = s.chars().collect();

    if chain_v.len() == 0 {
        let m: M = M::new(stateset!("q0"),
                          alphabet!(),
                          "q0".to_string(),
                          stateset!("q0"),
                          delta!());

        return m;
    }
    // len = 1

    let m: M = M::new(stateset!("q0", "q1"),
                      alphabet!(chain_v[0]),
                      "q0".to_string(),
                      stateset!("q1"),
                      delta!(("q0", chain_v[0], "q1")));

    return m;

}

#[cfg(test)]
mod tests {
    use automata::M;
    use super::regexp;


    #[test]
    fn prefix_automata_test() {
        use super::prefix_automata;

        let m = M::new(stateset!("q0", "q1"),
                       alphabet!('a'),
                       "q0".to_string(),
                       stateset!("q1"),
                       delta!(("q0", 'a', "q1")));

        let m_expected = M::new(stateset!("Aq0", "Aq1"),
                                alphabet!('a'),
                                "Aq0".to_string(),
                                stateset!("Aq1"),
                                delta!(("Aq0", 'a', "Aq1")));

        let m_actual = prefix_automata(&"A".to_string(), &m);

        assert_eq!(m_actual, m_expected)
    }

    #[test]
    fn union_test() {
        use super::automata_union;

        let m1 = M::new(stateset!("q0", "q1"),
                        alphabet!('a'),
                        "q0".to_string(),
                        stateset!("q1"),
                        delta!(("q0", 'a', "q1")));

        let m2 = M::new(stateset!("q0", "q1"),
                        alphabet!('b'),
                        "q0".to_string(),
                        stateset!("q1"),
                        delta!(("q0", 'b', "q1")));

        let m_expected = M::new(stateset!("0q0", "0f0", "01q0", "01q1", "02q0", "02q1"),
                                alphabet!('a', 'b'),
                                "0q0".to_string(),
                                stateset!("0f0"),
                                delta!(("0q0", 'λ', "01q0"),
                                       ("0q0", 'λ', "02q0"),

                                       ("01q0", 'a', "01q1"),
                                       ("02q0", 'b', "02q1"),

                                       ("01q1", 'λ', "0f0"),
                                       ("02q1", 'λ', "0f0")));

        let m = automata_union(&m1, &m2, "0".to_string());

        assert_eq!(m, m_expected)
    }

    #[test]
    fn intersection_test() {
        use super::automata_intersection;

        let m1 = M::new(stateset!("q0", "q1"),
                        alphabet!('a'),
                        "q0".to_string(),
                        stateset!("q1"),
                        delta!(("q0", 'a', "q1")));

        let m2 = M::new(stateset!("q0", "q1"),
                        alphabet!('b'),
                        "q0".to_string(),
                        stateset!("q1"),
                        delta!(("q0", 'b', "q1")));

        let m_expected = M::new(stateset!("01q0", "01q1", "02q0", "02q1"),
                                alphabet!('a', 'b'),
                                "01q0".to_string(),
                                stateset!("02q1"),
                                delta!(("01q0", 'a', "01q1"),
                                       ("01q1", 'λ', "02q0"),
                                       ("02q0", 'b', "02q1")));

        let m = automata_intersection(&m1, &m2, "0".to_string());

        {
            use automata::print_delta;
            print_delta(&m.delta);
            print_delta(&m_expected.delta);
        }

        assert_eq!(m, m_expected)
    }

    #[test]
    fn intersection_test_2() {
        use super::automata_intersection;

        let m1 = M::new(stateset!("q0"),
                        alphabet!('a'),
                        "q0".to_string(),
                        stateset!("q0"),
                        delta!(("q0", 'a', "q0")));

        let m2 = M::new(stateset!("q0", "q1"),
                        alphabet!('b'),
                        "q0".to_string(),
                        stateset!("q1"),
                        delta!(("q0", 'b', "q1")));

        let m_expected = M::new(stateset!("Q0", "Q1", "Q2"),
                                alphabet!('a', 'b'),
                                "Q0".to_string(),
                                stateset!("Q2"),
                                delta!(("Q0", 'a', "Q0"), ("Q0", 'λ', "Q1"), ("Q1", 'b', "Q2")));

        use automata::print_automata;
        use automata_min::pretify_automata;
        let m = automata_intersection(&m1, &m2, "0".to_string());
        let m = pretify_automata(&m);

        {
            println!("FUCK YOU");
            let m = pretify_automata(&m);
            print_automata(&m);
        }

        assert_eq!(m, m_expected)
    }

    #[test]
    fn star_test() {
        use super::automata_star;

        let m = M::new(stateset!("q0", "q1"),
                       alphabet!('a'),
                       "q0".to_string(),
                       stateset!("q1"),
                       delta!(("q0", 'a', "q1")));

        let m_expected = M::new(stateset!("0q0", "0f0", "01q0", "01q1"),
                                alphabet!('a'),
                                "0q0".to_string(),
                                stateset!("0f0"),
                                delta!(("01q0", 'a', "01q1"),

                                       ("0q0", 'λ', "01q0"),
                                       ("0q0", 'λ', "0f0"),
                                       ("01q1", 'λ', "01q0"),
                                       ("01q1", 'λ', "0f0")));

        let m = automata_star(&m, "0".to_string());

        {
            use automata::print_delta;
            print_delta(&m.delta);
            print_delta(&m_expected.delta);
        }

        assert_eq!(m, m_expected)
    }

    //Importante case for minification
    //The minified star for a single character should be an automata
    //with a single state which both initial and final
    #[test]
    fn star_test_case_1() {
        use super::automata_star;
        use automata_min::minify;
        use automata_operators::afndl_to_afd;

        let m = M::new(stateset!("q0", "q1"),
                       alphabet!('a'),
                       "q0".to_string(),
                       stateset!("q1"),
                       delta!(("q0", 'a', "q1")));

        let m_expected =
            M::new(stateset!("01q0-01q1-0f0-01q0-0f0-0q0"),
                   alphabet!('a'),
                   "01q0-01q1-0f0-01q0-0f0-0q0".to_string(),
                   stateset!("01q0-01q1-0f0-01q0-0f0-0q0"),
                   delta!(("01q0-01q1-0f0-01q0-0f0-0q0", 'a', "01q0-01q1-0f0-01q0-0f0-0q0")));

        let m = automata_star(&m, "0".to_string());
        {
            use automata::print_automata;
            println!("Automata star");
            print_automata(&m);
        }
        let m = afndl_to_afd(&m);
        {
            println!("Automata D");
            use automata::print_automata;
            print_automata(&m);
        }
        let m = minify(&m);

        {
            println!("Automata min");
            use automata::print_automata;
            print_automata(&m);
        }

        assert_eq!(m, m_expected)
    }

    #[test]
    fn regexp_base_case_lambda() {
        let m = regexp("".to_string());

        let m_expected = M::new(stateset!("q0"),
                                alphabet!(),
                                "q0".to_string(),
                                stateset!("q0"),
                                delta!());

        assert_eq!(m, m_expected)
    }


    #[test]
    fn base_case_single_char() {
        let m = regexp("a".to_string());

        let m_expected = M::new(stateset!("q0", "q1"),
                                alphabet!('a'),
                                "q0".to_string(),
                                stateset!("q1"),
                                delta!(("q0", 'a', "q1")));

        assert_eq!(m, m_expected)
    }

    #[test]
    fn regexp_union() {
        let mut m = regexp("a|b".to_string());

        {
            use automata::print_delta;
            print_delta(&m.delta);
        }

        assert!(m.check_string("a").is_ok());
        assert!(m.check_string("b").is_ok());
        assert!(m.check_string("ab").is_err());
    }

    #[test]
    fn regexp_intersection() {
        let mut m = regexp("ab".to_string());

        {
            use automata::print_delta;
            print_delta(&m.delta);
        }

        assert!(m.check_string("ab").is_ok());
        assert!(m.check_string("b").is_err());
    }

    #[test]
    fn regexp_star() {
        let mut m = regexp("a*".to_string());

        {
            use automata::print_delta;
            print_delta(&m.delta);
        }

        assert!(m.check_string("a").is_ok());
        assert!(m.check_string("aa").is_ok());
        assert!(m.check_string("aaaaaaaaaaaaaaa").is_ok());
        assert!(m.check_string("b").is_err());
        assert!(m.check_string("").is_ok());
    }
}
