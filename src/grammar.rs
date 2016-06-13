use std::collections::{BTreeSet};

pub type Terminal = char;
pub type TerminalSet = BTreeSet<Terminal>;
pub type NonTerminal = char;
pub type NonTerminalSet = BTreeSet<NonTerminal>;
pub type Chain = String;


pub type RegularProductions = BTreeSet<(NonTerminal, Chain)>;
//pub type RegularProductionsMap = BTreeMap<NonTerminal, BTreeSet<Chain>>;

#[derive(Clone, Debug)]
pub struct GR {
    vt: TerminalSet,
    vn: NonTerminalSet,
    productions: RegularProductions,
    q0: Terminal,
}

impl GR {
    pub fn new(vt: TerminalSet, vn: NonTerminalSet, productions: RegularProductions, q0: Terminal) -> GR {
        GR {
            q0: q0,
            vt: vt,
            vn: vn,
            productions: productions
        }
    }
}



use automata::{M, StateSet};
pub fn gr_to_afndl(gr: &GR) -> M {

    let alphabet = gr.vt.clone();
    let q0 = gr.q0.to_string();
    let f = "F".to_string();
    let f_set = stateset!(f);
    let k: StateSet = {
        let mut k = stateset!();
        k.insert(f.clone());
        for vn in &gr.vn {
            k.insert(vn.to_string());
        }
        k
    };

    let mut delta = delta!();
    for &(ref vn, ref chain) in &gr.productions {
        let chain: Vec<char> = chain.chars().collect();
        match chain.len() {
            1 => {
                let c = chain[0];
                delta.insert( (vn.to_string(), c, f.clone()) );
            },
            2 => {
                let c = chain[0];
                let ns = chain[1];
                delta.insert( (vn.to_string(), c, ns.to_string()) );
            },
            _ => {
                println!("NOT A REGULAR GRAMAR. Error in production {} -> {:?}", vn, chain);
                panic!("NOT A REGULAR GRAMMAR")
            }
        }
    }


    M::new(k, alphabet, q0, f_set, delta)
}

#[cfg(test)]
mod tests {
    #[test]
    fn gr_new_test() {
        use super::{GR, NonTerminalSet, TerminalSet, NonTerminal, RegularProductions};

        let vn: NonTerminalSet = charset!('S');
        let vt: TerminalSet = charset!('a');
        let q0: NonTerminal = 'S';
        let productions: RegularProductions = r_productions!(
            ('S', "aS"),
            ('S', "a")
        );

        let gr = GR::new(vt, vn, productions, q0);
        println!("Resulted grammar {:?}", gr);
    }

    #[test]
    fn gr_to_afndl_test() {
        use super::{GR, NonTerminalSet, TerminalSet, NonTerminal, RegularProductions, gr_to_afndl};
        use automata::to_delta_inner;

        let vn: NonTerminalSet = charset!('S');
        let vt: TerminalSet = charset!('a');
        let q0: NonTerminal = 'S';
        let productions: RegularProductions = r_productions!(
            ('S', "aS"),
            ('S', "a")
        );

        let gr = GR::new(vt, vn, productions, q0);
        let m = gr_to_afndl(&gr);

        let delta_expected = delta!(
            ("S", 'a', "S"),
            ("S", 'a', "F")
        );

        assert_eq!(m.k, stateset!("S", "F"));
        assert_eq!(m.q0, "S".to_string());
        assert_eq!(m.f, stateset!("F"));
        assert_eq!(m.alphabet, alphabet!('a'));
        assert_eq!(m.delta, to_delta_inner(delta_expected));
    }
}
