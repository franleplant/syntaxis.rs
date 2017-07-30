use std::collections::{HashSet, HashMap};
use std::cell::RefCell;
use std::rc::Rc;

use regexp::{Token, Node, NodeCat, lex, tree_to_automata};



pub fn re2(search: String, into: &'static str) -> Result<(), ()> {
    let mut p = Parser::new(search);
    p.parse();
    let mut m = tree_to_automata(p.tree.clone()).unwrap();
    m.check_string(into)
}

#[derive(Debug)]
struct Production {
    from: String,
    to: Vec<String>,
}

impl Production {
    pub fn new<T: Into<String>>(from: T, to: Vec<T>) -> Production {
        let mut to_arr = vec![];
        for x in to {
            to_arr.push(x.into())
        }

        Production {
            from: from.into(),
            to: to_arr,
        }
    }
}

fn get_productions() -> Vec<Production> {
    let prods = vec![Production::new("Re", vec!["Lit", "Ops"]),
                     Production::new("Re", vec!["(", "Re", ")", "Ops"]),

                     Production::new("Ops", vec!["*", "ReL"]),
                     Production::new("Ops", vec!["+", "ReL"]),
                     Production::new("Ops", vec!["|", "Re"]),
                     Production::new("Ops", vec!["Re"]),
                     Production::new("Ops", vec!["Lambda"]),

                     Production::new("ReL", vec!["Re"]),
                     Production::new("ReL", vec!["Lambda"])];

    prods
}

fn get_table() -> HashMap<(String, String), usize> {
    let entries = [(("Re", "Lit"), 0),
                   (("Re", "("), 1),

                   (("Ops", "*"), 2),
                   (("Ops", "+"), 3),
                   (("Ops", "|"), 4),
                   (("Ops", "Lit"), 5),
                   (("Ops", "("), 5),
                   (("Ops", "EOF"), 6),
                   (("Ops", ")"), 6),
                   (("Ops", "Lambda"), 6),

                   (("ReL", "Lit"), 7),
                   (("ReL", "("), 7),
                   (("ReL", "EOF"), 8),
                   (("ReL", ")"), 8),
                   (("ReL", "Lambda"), 8)];

    let mut table = HashMap::new();
    for &((non_terminal, token_cat), value) in entries.iter() {
        table.insert((non_terminal.to_string(), token_cat.to_string()), value);
    }

    table
}

fn is_terminal(s: &String) -> bool {
    let t = vec!["Lit", "(", ")", "*", "+", "|", "EOF", "Lambda"];

    let terminals: HashSet<String> = t.iter().cloned().map(|s| s.to_string()).collect();
    terminals.contains(s)
}


#[derive(Debug)]
struct Parser {
    pub index: usize,
    pub src: String,
    pub tokens: Vec<Token>,
    pub productions: Vec<Production>,
    pub table: HashMap<(String, String), usize>,
    pub tree: Rc<RefCell<Node>>,
    stack: Vec<Rc<RefCell<Node>>>,
}


impl Parser {
    pub fn new(src: String) -> Parser {
        let tokens = lex(src.clone());
        let root = Node::new_nt("Re");

        let stack = vec![Node::new_t(Token {
                                         category: "EOF".to_string(),
                                         lexeme: "".to_string(),
                                     }),
                         root.clone()];

        Parser {
            index: 0,
            src: src,
            tokens: tokens,
            productions: get_productions(),
            table: get_table(),
            stack: stack,
            tree: root.clone(),
        }
    }

    fn parse_focus(&self) -> Option<Rc<RefCell<Node>>> {
        assert!(self.stack.len() != 0, "OVERFLOW {:?}", self);
        self.stack.get(self.stack.len() - 1).map(|s| s.clone())
    }

    pub fn parse(&mut self) -> bool {
        loop {
            let ref token = self.tokens[self.index];
            let parse_focus = self.parse_focus().expect("Something went wrong");

            let cat = {
                let parse_focus = parse_focus.borrow();
                parse_focus.category.as_string()
            };


            println!("pfocus {} {:?} stack {:?}",
                     parse_focus.borrow().as_string(),
                     token,
                     self.stack
                         .iter()
                         .map(|s| s.borrow().as_string())
                         .collect::<Vec<String>>());

            if cat == "EOF".to_string() && token.category == "EOF" {
                return true;
            } else if is_terminal(&cat) {
                if cat == token.category {
                    let node = self.stack.pop().unwrap();
                    let mut node = node.borrow_mut();
                    node.category = NodeCat::T(token.clone());
                    self.index += 1;
                } else {
                    println!("ERROR: wrong symbol at the top of the stack");
                    return false;
                }
            } else {
                let prod_number = self.table
                    .get(&(cat, token.category.clone()))
                    .expect("ERROR: expanding parse_focus 1");

                let prod = self.productions
                    .get(*prod_number)
                    .expect("ERROR: expanding parse_focus 2");

                println!("REDUCE {:?}", prod);

                let root = self.stack.pop().unwrap();
                let mut root = root.borrow_mut();
                for s in &prod.to {
                    match s {
                        _ if is_terminal(s) => {
                            let token = Token {
                                category: s.clone(),
                                lexeme: "".to_string(),
                            };
                            let node = Node::new_t(token);
                            root.children.push(node.clone());
                        }

                        _ => {
                            let node = Node::new_nt(s.clone().as_str());
                            root.children.push(node.clone());
                        }
                    }
                }

                for c in root.children.iter().rev() {
                    let cat = c.borrow().category.as_string();
                    if cat != "Lambda".to_string() {
                        self.stack.push(c.clone());
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {

        let cases = vec!["a",
                         "(a)",
                         "(aa)",
                         "a*b",
                         "(a*)b",
                         "(a)*b",
                         "a|(cde)*a+",
                         "((((aaa))))",
                         "(a"];

        let expect = vec![true, true, true, true, true, true, true, true, false];

        for (c, e) in cases.iter().zip(expect.iter()) {
            println!("\nCase {:?}\n", c);
            let mut p = Parser::new(c.to_string());
            assert_eq!(p.parse(), *e, "In {:?}", c);
            println!("TREE\n");
            Node::preorder_walk(p.tree.clone(), 0);
        }
    }

}
