use std::cell::{Cell, RefCell};
use std::rc::Rc;
use lex::{Token, lex};
use regexp_tree::{Node, tree_to_automata};



pub fn re(search: String, into: &'static str) -> Result<(), ()> {
    let p = Parser::new(search);
    p.parse();
    let mut m = tree_to_automata(p.tree.clone()).unwrap();
    m.check_string(into)
}






#[derive(Debug)]
pub struct Parser {
    pub index: Cell<usize>,
    pub src: String,
    pub tokens: Vec<Token>,
    pub tree: Rc<RefCell<Node>>,
    pub focus: RefCell<Rc<RefCell<Node>>>,
}

impl Parser {
    pub fn new(src: String) -> Parser {
        let tokens = lex(src.clone());
        let root = Node::new_nt("Re");
        Parser {
            index: Cell::new(0),
            src: src,
            tokens: tokens,
            tree: root.clone(),
            focus: RefCell::new(root.clone()),
        }
    }


    pub fn next(&self) {
        self.index.set(self.index.get() + 1);
    }

    pub fn print(&self) {
        println!("\nParser");
        let focus = self.focus.borrow();
        println!("focus {:?}", focus.borrow().category);
        Node::preorder_walk(self.tree.clone(), 0);
    }

    pub fn parse(&self) -> bool {
        return self.re();
    }

    pub fn re(&self) -> bool {
        let token = self.tokens.get(self.index.get()).expect("Re panic");

        println!("Re {:?}", token);

        match token.category.as_str() {
            // Re -> Lit Ops
            // First
            "Lit" => {
                {
                    let a = Node::new_t(token.clone());
                    let new_focus = Node::new_nt("Ops");

                    {
                        let focus = self.focus.borrow();
                        let mut focus = focus.borrow_mut();
                        focus.children.push(a);
                        focus.children.push(new_focus.clone());
                    }

                    *self.focus.borrow_mut() = new_focus;
                }
                self.next();
                return self.ops();
            },

            // Re -> ( Re ) Ops
            // First
            "(" => {
                let new_focus = Node::new_nt("Re");
                let ops_focus = Node::new_nt("Ops");
                {
                    {
                        let focus = self.focus.borrow();
                        let mut focus = focus.borrow_mut();
                        focus.children.push(Node::new_t(token.clone()));
                        focus.children.push(new_focus.clone());
                        focus.children.push(Node::new_t(Token { category: ")".to_string(), lexeme: "".to_string()}));
                        focus.children.push(ops_focus.clone());
                    }

                    *self.focus.borrow_mut() = new_focus;
                }

                self.next();
                if self.re() {
                    let token = self.tokens.get(self.index.get()).expect("Re panic");
                    // Re -> ( Re ) Ops
                    // follow Re
                    if token.category == ")" {
                        {
                            *self.focus.borrow_mut() = ops_focus;
                        }
                        self.next();
                        return self.ops();
                    }
                }

                return false;
            },

            "EOF" => return true,

            _ => return false,
        }
    }

    pub fn ops(&self) -> bool {
        let token = self.tokens.get(self.index.get()).expect("Ops panic");

        println!("Ops {:?}", token);

        match token.category.as_str() {
            // Ops -> | Re
            // First
            "|" => {
                {
                    let a = Node::new_t(token.clone());
                    let new_focus = Node::new_nt("Re");

                    {
                        let focus = self.focus.borrow();
                        let mut focus = focus.borrow_mut();
                        focus.children.push(a);
                        focus.children.push(new_focus.clone());
                    }

                    *self.focus.borrow_mut() = new_focus;
                }
                self.next();
                return self.re();
            },
            // Ops -> * ReFinish
            // First
            "*" => {
                {
                    let a = Node::new_t(token.clone());
                    let new_focus = Node::new_nt("ReFinish");

                    {
                        let focus = self.focus.borrow();
                        let mut focus = focus.borrow_mut();
                        focus.children.push(a);
                        focus.children.push(new_focus.clone());
                    }

                    *self.focus.borrow_mut() = new_focus;
                }
                self.next();
                return self.re_finish();
            },
            // Ops -> + ReFinish
            // First
            "+" => {
                {
                    let a = Node::new_t(token.clone());
                    let new_focus = Node::new_nt("ReFinish");

                    {
                        let focus = self.focus.borrow();
                        let mut focus = focus.borrow_mut();
                        focus.children.push(a);
                        focus.children.push(new_focus.clone());
                    }

                    *self.focus.borrow_mut() = new_focus;
                }
                self.next();
                return self.re_finish();
            },

            // Ops -> Re
            // First
            "Lit" | "(" => {
                {
                    let new_focus = Node::new_nt("Re");

                    {
                        let focus = self.focus.borrow();
                        let mut focus = focus.borrow_mut();
                        focus.children.push(new_focus.clone());
                    }

                    *self.focus.borrow_mut() = new_focus;
                }
                return self.re();
            },

            // Follow
            ")" => {
                {
                    let focus = self.focus.borrow();
                    let mut focus = focus.borrow_mut();
                    focus.children.push(Node::new_t(Token { category: "Lambda".to_string(), lexeme: "".to_string()}));
                }
                return true;
            },

            // Follow
            "EOF" => {
                {
                    let focus = self.focus.borrow();
                    let mut focus = focus.borrow_mut();
                    focus.children.push(Node::new_t(Token { category: "EOF".to_string(), lexeme: "".to_string()}));
                }
                return true;
            },

            _ => return false,
        }
    }

    pub fn re_finish(&self) -> bool {
        let token = self.tokens.get(self.index.get()).expect("ReFinish panic");

        println!("ReFinish {:?}", token);

        match token.category.as_str() {
            // ReFinish -> Re
            // first
            "Lit" | "(" => {
                {
                    let new_focus = Node::new_nt("Re");

                    {
                        let focus = self.focus.borrow();
                        let mut focus = focus.borrow_mut();
                        focus.children.push(new_focus.clone());
                    }

                    *self.focus.borrow_mut() = new_focus;
                }
                return self.re();
            },
            // Follow
            ")" => {
                {
                    let focus = self.focus.borrow();
                    let mut focus = focus.borrow_mut();
                    focus.children.push(Node::new_t(Token { category: "Lambda".to_string(), lexeme: "".to_string()}));
                }
                return true;
            },
            // Follow
            "EOF" => {
                {
                    let focus = self.focus.borrow();
                    let mut focus = focus.borrow_mut();
                    focus.children.push(Node::new_t(Token { category: "EOF".to_string(), lexeme: "".to_string()}));
                }
                return true;
            },

            _ => return false,
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_test() {
        let cases = vec![
            ("a", vec![("Lit", "a"), ("EOF", "")]),
            ("aa", vec![("Lit", "a"), ("Lit", "a"), ("EOF", "")]),
            ("a|b|c", vec![("Lit", "a"), ("|", ""), ("Lit", "b"), ("|", ""), ("Lit", "c"), ("EOF", "")]),
            ("(a)", vec![("(", ""), ("Lit", "a"), (")", ""), ("EOF", "")]),
            ("a(b)", vec![("Lit", "a"), ("(", ""), ("Lit", "b"), (")", ""), ("EOF", "")]),
            //"(a|b)",
            //"(a|b)*",
        ];

        for (c, e) in cases {
            let expected: Vec<Token> = e.iter()
                .map(|&(cat, lexeme)| Token { category: cat.to_string(), lexeme: lexeme.to_string()})
                .collect();
            assert_eq!(lex(c.to_string()), expected);
        }
    }


    #[test]
    fn parse_test() {

        let cases = vec![
            "a",
            "(a)",
            "(aa)",
            "a*b",
            "(a*)b",
            "(a)*b",
            "a|(cde)*a+",
            "((((aaa))))",
        ];

        let expect = vec![
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
        ];

        for (c, e) in cases.iter().zip(expect.iter()) {
            println!("\nCase {:?}\n", c);
            let p = Parser::new(c.to_string());
            assert_eq!(p.parse(), *e, "In {:?}", c);
            p.print();
        }
    }


    #[test]
    fn engine_hl_test() {
        assert!(re("hola".to_string(), "hola").is_ok());
        assert!(re("hola+".to_string(), "holaaaaaa").is_ok());
        assert!(re("(ho)+la+".to_string(), "hoholaaaaaa").is_ok());
    }

}
