use std::cell::RefCell;
use std::rc::Rc;

use automata::{M, print_automata};
use automata_min::{minify, pretify_automata};
use automata_operators::afndl_to_afd;

use regexp::{Token, re_trivial, automata_intersection, automata_union, automata_star};


#[derive(Debug)]
pub enum NodeCat {
    T(Token),
    NT(String),
}

impl NodeCat {
    pub fn as_string(&self) -> String {
        match self {
            &NodeCat::NT(ref cat) => {
                cat.clone()
            }

            &NodeCat::T(ref token) => {
                token.category.clone()
            }
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub category: NodeCat,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new_nt<T: Into<String>>(val: T) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
                                 category: NodeCat::NT(val.into()),
                                 children: vec![],
                             }))
    }

    pub fn new_t(val: Token) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
                                 category: NodeCat::T(val),
                                 children: vec![],
                             }))
    }

    pub fn as_string(&self) -> String {
        match self.category {
            NodeCat::NT(ref cat) => {
                format!("Node( {}, {} )", cat, self.children.len())
            }

            NodeCat::T(ref token) => {
                format!("Node( {}, {}, {} )", token.category, token.lexeme, self.children.len())
            }
        }
    }

    pub fn print(&self) {
        println!("{:?}", self.category);
        for c in &self.children {
            println!("- {:?}", c.borrow().category);
        }

        for c in &self.children {
            if let NodeCat::NT(_) = c.borrow().category {
                c.borrow().print();
            }
        }
    }

    pub fn preorder_walk(x: Rc<RefCell<Node>>, level: usize) {
        let x = x.borrow();
        let separator = "|-- ";
        let s = format!("{}{:?}", separator, x.category);
        let space_n = level * separator.len();

        let mut space = String::new();
        for _ in 0..space_n {
            space.push_str(" ");
        }
        println!("{}{}", space, s);

        for c in &x.children {
            Node::preorder_walk(c.clone(), level + 1);
        }
    }
}

pub fn tree_to_automata(x: Rc<RefCell<Node>>) -> Option<M> {
    let x = x.borrow();

    if let NodeCat::T(ref token) = x.category {
        println!("{:?}", token);

        match token.category.as_str() {
            "EOF" | "Lambda" | "(" | ")" | "*" | "|" | "+" => return None,
            "Lit" => {
                let m = re_trivial(token.lexeme.clone());
                {
                    println!("INTERMEDIATE Trivial\n++++++++++++++++");
                    let m = pretify_automata(&m);
                    print_automata(&m);
                }

                return Some(m);
            }
            _ => panic!("Don't know how to handle this yet"),
        }
    }

    let mut m = None;
    for c in &x.children {

        // Ops -> * Rel
        if let NodeCat::NT(ref cat) = c.borrow().category {
            if cat.as_str() == "Ops" {
                let c = c.borrow();
                let first_child = c.children[0].borrow();
                if let NodeCat::T(ref token) = first_child.category {
                    let cat = token.category.as_str();
                    match cat {
                        "*" => {
                            if m == None {
                                panic!("Something went wrong while expanding Lit *");
                            }

                            m = Some(automata_star(&m.unwrap(), "*".to_string()));
                            //{
                            //println!("INTERMEDIATE CASE star \n++++++++++++++++");
                            //let m = pretify_automata(&m.clone().unwrap());
                            //print_automata(&m);
                            //}
                            m = Some(afndl_to_afd(&m.unwrap()));
                            m = Some(minify(&m.unwrap()));
                        }

                        "+" => {
                            if m == None {
                                panic!("Something went wrong while expanding Lit *");
                            }

                            let m_star = automata_star(&m.clone().unwrap(), "*".to_string());
                            m = Some(automata_intersection(&m.unwrap(), &m_star, "+".to_string()));
                            //{
                            //println!("INTERMEDIATE CASE star \n++++++++++++++++");
                            //let m = pretify_automata(&m.clone().unwrap());
                            //print_automata(&m);
                            //}
                            m = Some(afndl_to_afd(&m.unwrap()));
                            m = Some(minify(&m.unwrap()));
                        }

                        "|" => {
                            if m == None {
                                panic!("Something went wrong while expanding Lit |");
                            }

                            let second_child = &c.children[1];
                            let next_m = tree_to_automata(second_child.clone())
                                .expect("Something went wrong while expanding Lit |");

                            m = Some(automata_union(&m.unwrap(), &next_m, "u".to_string()));
                            m = Some(afndl_to_afd(&m.unwrap()));
                            m = Some(minify(&m.unwrap()));
                            // Don't concat
                            continue;
                        }

                        _ => {}
                    }
                }
            }
        }

        let next_m = tree_to_automata(c.clone());
        if next_m == None {
            continue;
        }

        if m == None {
            m = next_m;
            continue;
        }

        m = Some(automata_intersection(&m.unwrap(), &next_m.unwrap(), "-".to_string()));
        m = Some(afndl_to_afd(&m.unwrap()));
        m = Some(minify(&m.unwrap()));
    }

    return m;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn engine_test() {
        use regexp::recursive_parser::Parser;

        let cases = vec!["a",
                         "(a)",
                         "(aa)",

                         "a*",
                         "a*b",
                         "(a*)b",
                         "(a)*b",

                         "a|b",
                         "(a)|b",
                         "(a)|(b)",
                         "a|(b)",

                         "a+",
                         "a+b",
                         "(a+)b",
                         "(a)+b",

                         "a|(cde)*a+",
                         "((((aaa))))"];

        let expect = vec![(vec!["a"], vec!["ab"]),

                          (vec!["a"], vec!["ab"]),
                          (vec!["aa"], vec!["a"]),

                          (vec!["", "a", "aaa"], vec!["ab"]),
                          (vec!["b", "aab", "aaaaaaab"], vec!["a", "aaa"]),
                          (vec!["b", "aab", "aaaaaaab"], vec!["a", "aaa"]),
                          (vec!["b", "aab", "aaaaaaab"], vec!["a", "aaa"]),

                          (vec!["a", "b"], vec!["ab", "bb", "aa", "cc"]),
                          (vec!["a", "b"], vec!["ab", "bb", "aa", "cc"]),
                          (vec!["a", "b"], vec!["ab", "bb", "aa", "cc"]),
                          (vec!["a", "b"], vec!["ab", "bb", "aa", "cc"]),

                          (vec!["a", "aaa"], vec!["ab", ""]),
                          (vec!["ab", "aab", "aaaaaaab"], vec!["a", "aaa", "b"]),
                          (vec!["ab", "aab", "aaaaaaab"], vec!["a", "aaa", "b"]),
                          (vec!["ab", "aab", "aaaaaaab"], vec!["a", "aaa", "b"])];

        for (c, e) in cases.iter().zip(expect.iter()) {
            println!("\nCase {:?}\n=========\n", c);
            let p = Parser::new(c.to_string());
            p.parse();
            println!("WALK");
            let mut m = tree_to_automata(p.tree.clone()).unwrap();
            println!("TREE");
            p.print();

            println!("AUTOMATA\n");
            {
                let m = pretify_automata(&m);
                print_automata(&m);
            }

            let &(ref oks, ref errs) = e;
            for ok in oks {
                assert!(m.check_string(ok).is_ok());
            }

            for err in errs {
                assert!(m.check_string(err).is_err());
            }
        }
    }
}
