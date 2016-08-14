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


// TODO:
// - handy function to print trees!
// - handy function to go from TNT -> Char
// - handy function to go from Derivation -> String

impl CFG {
    pub fn new<T: Into<String> + fmt::Debug>(vn: NonTerminalSet, vt: TerminalSet, p: Productions<T>, s: NonTerminal) -> CFG {
        let mut p_map: ProductionsMap = BTreeMap::new();

        if !vn.is_disjoint(&vt) {
            panic!("VN and VT must be disjoint.\nVN: {:?} \nVT: {:?}", vn, vt);
        }

        for (nt, der_str) in p {
            if !vn.contains(&nt) {
                panic!("NonTerminal in production rule does not belong to VN {:?} -> {:?} \n {:?}", nt, der_str, vn);
            }

            let dervec = p_map.entry(nt).or_insert(vec!());

            let mut der: Derivation = vec!();

            let der_string = der_str.into();

            if der_string.len() == 0 {
                der.push(TNT::Lambda);
            }

            for c in der_string.chars() {
                if vn.contains(&c) {
                    der.push( TNT::NT(c.clone()) );
                    continue
                }

                if vt.contains(&c) {
                    der.push( TNT::T(c.clone()) );
                    continue
                }

                panic!("Char in derivation {:?} -> {:?} does not belong to VN or VT {:?}", nt, der_string, c);
            }

            dervec.push(der);
        }

        CFG {
            vn: vn,
            vt: vt,
            p: p_map,
            s: s,
        }
    }

    pub fn get_nt_derivations(&self, nt: &NonTerminal) -> DerivationVec {
        //TODO: optimize the case where the nt is not found
        self.p.get(nt).unwrap().clone()
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


#[derive(Debug, Clone)]
struct TNode {
    val: TNT,
    children: Vec<TNode>
}

impl TNode {
    pub fn new(val: TNT, children: Vec<TNode>) -> TNode {
        TNode {
            val: val,
            children: children,
        }
    }

}

impl fmt::Display for TNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Need to create a single derivation tree for... a single production rule like this
// A -> (A)
//TODO
//Creates a single level derivation tree for the given nonterminal
//in the given cfg
fn tree(nt: &NonTerminal, der: &Derivation) -> TNode {
    let mut children: Vec<TNode> = vec!();
    for e in der {
        let node = TNode::new(e.clone(), vec!());
        children.push(node);
    }
    // TODO: check if the nt is effectively a nonterminal or not
    let root = TNode::new(TNT::NT(nt.clone()), children);
    root
}



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

        let g = CFG::new(vn, vt, p, s);
        println!("Resulted grammar {}", g);
    }

    #[test]
    fn tree_test() {
        use super::{TNT, tree};

        let der = vec!( TNT::T('('), TNT::NT('S'), TNT::T(')') );
        let t = tree(&'S', &der);
        println!("Resulted tree {}", t);
    }
}
