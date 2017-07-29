use std::collections::{HashSet, HashMap};
use std::cell::{Cell, RefCell};

use regexp::{Token, lex};



pub fn re2(search: String, into: &'static str) -> Result<(), ()> {
    Ok(())
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


    //table.insert(("Re", "Lit"), 0);
    //table.insert(("Re", "("), 1);

    //table.insert(("Ops", "*"), 2);
    //table.insert(("Ops", "+"), 3);
    //table.insert(("Ops", "|"), 4);
    //table.insert(("Ops", "Lit"), 5);
    //table.insert(("Ops", "("), 5);
    //table.insert(("Ops", "eof"), 6);
    //table.insert(("Ops", ")"), 6);
    //table.insert(("Ops", "Lambda"), 6);

    //table.insert(("ReL", "Lit"), 7);
    //table.insert(("ReL", "("), 7);
    //table.insert(("ReL", "eof"), 8);
    //table.insert(("ReL", ")"), 8);
    //table.insert(("ReL", "Lambda"), 8);

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
    stack: Vec<String>,
}


impl Parser {
    pub fn new(src: String) -> Parser {
        let tokens = lex(src.clone());
        //let root = Node::new_nt("Re");
        Parser {
            index: 0,
            src: src,
            tokens: tokens,
            productions: get_productions(),
            table: get_table(),
            stack: vec!["EOF".to_string(), "Re".to_string()],
        }
    }

    fn parse_focus(&self) -> Option<String> {
        assert!(self.stack.len() != 0, "OVERFLOW {:?}", self);
        self.stack.get(self.stack.len() - 1).map(|s| s.clone())
    }

    pub fn parse(&mut self) -> bool {
        loop {
            let ref token = self.tokens[self.index];
            let parse_focus = self.parse_focus().expect("Something went wrong");

            println!("pfocus {:?} {:?} stack {:?}",
                     parse_focus,
                     token,
                     self.stack);

            if parse_focus == "EOF" && token.category == "EOF" {
                return true;
            } else if is_terminal(&parse_focus) {
                if *parse_focus == token.category {
                    self.stack.pop().unwrap();
                    self.index += 1;
                } else {
                    panic!("ERROR: no entry found in table. parse focus {:?}, token {:?}, stack {:?}",
                           parse_focus,
                           token,
                           self.stack)
                }
            } else {
                let prod_number = self.table
                    .get(&(parse_focus, token.category.clone()))
                    .expect("ERROR: expanding parse_focus");

                let prod = self.productions
                    .get(*prod_number)
                    .expect("ERROR: expanding parse_focus");

                self.stack.pop().unwrap();
                for s in prod.to.iter().rev() {
                    if s != "Lambda" {
                        self.stack.push(s.clone());
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
                         "((((aaa))))"];

        let expect = vec![true, true, true, true, true, true, true, true];

        for (c, e) in cases.iter().zip(expect.iter()) {
            println!("\nCase {:?}\n", c);
            let mut p = Parser::new(c.to_string());
            assert_eq!(p.parse(), *e, "In {:?}", c);
            //p.print();
        }
    }
}
