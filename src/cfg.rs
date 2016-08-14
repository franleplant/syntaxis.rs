use std::collections::{BTreeSet, BTreeMap};
use std::fmt;


type Terminal = char;
type TerminalSet = BTreeSet<Terminal>;
type NonTerminal = char;
type NonTerminalSet = BTreeSet<NonTerminal>;

#[derive(Clone, Debug)]
enum TNT {
    T(Terminal),
    NT(NonTerminal),
    Lambda,
}

type Derivation = Vec<TNT>;
type DerivationVec = Vec<Derivation>;

type Productions<T> = Vec<(NonTerminal, T)>;
type ProductionsMap = BTreeMap<NonTerminal, DerivationVec>;

#[derive(Clone, Debug)]
struct CFG {
    vn: NonTerminalSet,
    vt: TerminalSet,
    p: ProductionsMap,
    s: NonTerminal,
}

impl CFG {
    pub fn new<T: Into<String>>(vn: NonTerminalSet, vt: TerminalSet, p: Productions<T>, s: NonTerminal) -> CFG {
        let mut p_map: ProductionsMap = BTreeMap::new();

        // TODO: this assumes terminals and non terminals can never have the same 
        // letter, but doesnot check it
        // TODO: check if vn and vt are effectively disjoint
        for (nt, der_str) in p {
            let mut dervec: DerivationVec = {
                if let Some(dervec) = p_map.get(&nt) {
                    dervec.clone()
                } else {
                    vec!()
                }
            };

            let mut der: Derivation = vec!();

            let der_string = der_str.into();

            if der_string.len() == 0 {
                der.push(TNT::Lambda);
            }

            for c in der_string.chars() {
                println!("c {:?}", c);
                if vn.contains(&c) {
                    println!("vn {:?}", c);
                    der.push( TNT::NT(c.clone()) );
                    continue
                }

                if vt.contains(&c) {
                    der.push( TNT::T(c.clone()) );
                    continue
                }

                panic!("Char in derivation does not belong to VN or VT {:?}", c);
            }

            dervec.push(der);
            p_map.insert(nt.clone(), dervec);
        }

        CFG {
            vn: vn,
            vt: vt,
            p: p_map,
            s: s,
        }
    }
}


impl fmt::Display for CFG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\n").unwrap();
        write!(f, "Context Free Grammar \n").unwrap();
        write!(f, "==================== \n").unwrap();
        write!(f, "VN: {:?} \n", self.vn).unwrap();
        write!(f, "VT: {:?} \n", self.vt).unwrap();
        write!(f, "S: {:?} \n", self.s).unwrap();
        write!(f, "Productions: \n").unwrap();
        for (nt, dervec) in &self.p {
            for der in dervec {
                let mut der_string = String::new();
                for e in der {
                    match *e {
                        TNT::T(t) => der_string.push(t),
                        TNT::NT(nt) => der_string.push(nt),
                        _ => {},
                    }
                }
                write!(f, "{:?} -> {:?} \n", nt, der_string).unwrap();
            }
        }

        write!(f, "\n")
    }
}


// TODO work on the tree impl
//
//struct Tree {
    //root: NonTerminal,
    //children: Vec<Terminal U NonTerminal U Lambda U Tree?>
//}

// 2nd iteration of tree
//struct Node {
    //val: NonTerminal or Terminal or Lambda,
    //children: Option<Node>
//}

//fn tree(cfg: CFG, nt: NonTerminal) -> Tree {
    //TODO
    //Creates a single level derivation tree for the given nonterminal
    //in the given cfg
//}



#[cfg(test)]
mod tests {
    #[test]
    fn cfg_new_test() {

        use super::{CFG, NonTerminalSet, TerminalSet, NonTerminal, Productions};

        let vn: NonTerminalSet = charset!('S');
        let vt: TerminalSet = charset!('a', '(', ')');
        let s: NonTerminal = 'S';
        let p: Productions<&'static str> = vec!(
            ('S', "(S)" ),
            ('S', "a" )
        );

        let g = CFG::new(vt, vn, p, s);
        println!("Resulted grammar {}", g);
    }
}
