use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    category: String,
    lexeme: String,
}

fn lex(s: String) -> Vec<Token> {
    let mut tokens = vec![];

    for c in s.chars() {
        let (cat, lexeme) = match c {
            '|' => ("|", "".to_string()),
            '*' => ("*", "".to_string()),
            '+' => ("+", "".to_string()),
            '(' => ("(", "".to_string()),
            ')' => (")", "".to_string()),
            _ => ("Lit", c.to_string()),
        };

        tokens.push( Token{ category: cat.to_string(), lexeme: lexeme.to_string() } );
    }

    tokens.push( Token{ category: "EOF".to_string(), lexeme: "".to_string() } );

    tokens
}


#[derive(Debug)]
pub enum NodeCat {
    T(String),
    NT(String),
}

#[derive(Debug)]
pub struct Node {
    category: NodeCat,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new_nt(val: &'static str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            category: NodeCat::NT(val.to_string()),
            children: vec![],
        }))
    }

    fn new_t(val: &'static str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            category: NodeCat::T(val.to_string()),
            children: vec![],
        }))
    }

    fn print(&self) {
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

    fn preorder_walk(x: Rc<RefCell<Node>>, level: usize) {
        let x = x.borrow();
        let separator = "|-- ";
        let s = format!("{}{:?}", separator, x.category);
        let space_n = level * separator.len();

        let mut space = String::new();
        for i in 0..space_n {
            space.push_str(" ");
        }
        println!("{}{}", space, s);

        for c in &x.children {
            Node::preorder_walk(c.clone(), level + 1);
        }
    }
}



#[derive(Debug)]
pub struct Parser {
    index: Cell<usize>,
    src: String,
    tokens: Vec<Token>,
    tree: Rc<RefCell<Node>>,
    focus: RefCell<Rc<RefCell<Node>>>,
}

impl Parser {
    fn new(src: String) -> Parser {
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


    fn next(&self) {
        self.index.set(self.index.get() + 1);
    }

    fn print(&self) {
        println!("\nParser");
        let focus = self.focus.borrow();
        println!("focus {:?}", focus.borrow().category);
        Node::preorder_walk(self.tree.clone(), 0);
    }

    fn parse(&self) -> bool {
        return self.re();
    }

    fn re(&self) -> bool {
        let token = self.tokens.get(self.index.get()).expect("Re panic");

        println!("Re {:?}", token);

        match token.category.as_str() {
            // Re -> Lit Ops
            "Lit" => {
                {
                    let a = Node::new_t("Lit");
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
            "(" => {
                let new_focus = Node::new_nt("Re");
                let ops_focus = Node::new_nt("Ops");
                {
                    {
                        let focus = self.focus.borrow();
                        let mut focus = focus.borrow_mut();
                        focus.children.push(Node::new_t("("));
                        focus.children.push(new_focus.clone());
                        focus.children.push(Node::new_t(")"));
                        focus.children.push(ops_focus.clone());
                    }

                    *self.focus.borrow_mut() = new_focus;
                }

                self.next();
                if self.re() {
                    let token = self.tokens.get(self.index.get()).expect("Re panic");
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

    fn ops(&self) -> bool {
        let token = self.tokens.get(self.index.get()).expect("Ops panic");

        println!("Ops {:?}", token);

        match token.category.as_str() {
            // Ops -> | Re
            // First
            "|" => {
                {
                    let a = Node::new_t("|");
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
                    let a = Node::new_t("*");
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
                    let a = Node::new_t("+");
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

            // Follow
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
                    focus.children.push(Node::new_t("Lambda"));
                }
                return true;
            },

            // Follow
            "EOF" => {
                {
                    let focus = self.focus.borrow();
                    let mut focus = focus.borrow_mut();
                    focus.children.push(Node::new_t("EOF"));
                }
                return true;
            },

            _ => return false,
        }
    }

    fn re_finish(&self) -> bool {
        let token = self.tokens.get(self.index.get()).expect("ReFinish panic");

        println!("ReFinish {:?}", token);

        match token.category.as_str() {
            // ReFinish -> Re
            // Follow
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
                    focus.children.push(Node::new_t("Lambda"));
                }
                return true;
            },
            // Follow
            "EOF" => {
                {
                    let focus = self.focus.borrow();
                    let mut focus = focus.borrow_mut();
                    focus.children.push(Node::new_t("EOF"));
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
}
