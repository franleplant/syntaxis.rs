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

#[cfg(test)]
mod tests {
    #[test]
    fn gr_new_test() {
        use super::{GR, NonTerminalSet, TerminalSet, NonTerminal, RegularProductions};

        let vt: NonTerminalSet = charset!('S');
        let vn: TerminalSet = charset!('a');
        let q0: NonTerminal = 'S';
        let productions: RegularProductions = r_productions!(
            ('S', "aS"),
            ('S', "a")
        );

        let gr = GR::new(vt, vn, productions, q0);
        println!("FUUU {:?}", gr);
    }
}
