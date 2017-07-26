use std::cell::{Cell, RefCell};
use std::rc::Rc;
use automata::{M, print_automata};
use regexp::{re_trivial, automata_intersection, automata_union, automata_star};
use automata_min::{minify, pretify_automata};
use automata_operators::{afndl_to_afd};



pub fn re(search: String, into: &'static str) -> Result<(), ()> {
    let p = Parser::new(search);
    p.parse();
    let mut m = Node::postorder_walk(p.tree.clone()).unwrap();
    m.check_string(into)
}

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
    T(Token),
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

    fn new_t(val: Token) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            category: NodeCat::T(val),
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

    fn postorder_walk(x: Rc<RefCell<Node>>) -> Option<M> {
        let x = x.borrow();

        if let NodeCat::T(ref token) = x.category {
            println!("{:?}", token);

            match token.category.as_str() {
                "EOF" | "Lambda" | "(" | ")" | "*" | "|" | "+" => return None,
                "Lit" => {
                    let m  = re_trivial(token.lexeme.clone());
                    {
                        println!("INTERMEDIATE Trivial\n++++++++++++++++");
                        let m = pretify_automata(&m);
                        print_automata(&m);
                    }

                    return Some(m);
                },
                _ => panic!("Don't know how to handle this yet")
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
                            },

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
                            },

                            "|" => {
                                if m == None {
                                    panic!("Something went wrong while expanding Lit |");
                                }

                                let second_child = &c.children[1];
                                let next_m = Node::postorder_walk(second_child.clone()).expect("Something went wrong while expanding Lit |");

                                m = Some(automata_union(&m.unwrap(), &next_m, "u".to_string()));
                                m = Some(afndl_to_afd(&m.unwrap()));
                                m = Some(minify(&m.unwrap()));
                                // Don't concat
                                continue
                            },

                            _ => {}
                        }
                    }
                }
            }

            let next_m = Node::postorder_walk(c.clone());
            if next_m == None {
                continue
            }

            if m == None {
                m = next_m;
                continue
            }

            m = Some(automata_intersection(&m.unwrap(), &next_m.unwrap(), "-".to_string()));
            m = Some(afndl_to_afd(&m.unwrap()));
            m = Some(minify(&m.unwrap()));
        }

        return m
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
    fn engine_test() {

        let cases = vec![
            "a",
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
            "((((aaa))))",
        ];

        let expect = vec![
            (vec!["a"], vec!["ab"]),

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
            (vec!["ab", "aab", "aaaaaaab"], vec!["a", "aaa", "b"]),
        ];

        for (c, e) in cases.iter().zip(expect.iter()) {
            println!("\nCase {:?}\n=========\n", c);
            let p = Parser::new(c.to_string());
            p.parse();
            println!("WALK");
            let mut m = Node::postorder_walk(p.tree.clone()).unwrap();
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

    #[test]
    fn engine_hl_test() {
        assert!(re("hola".to_string(), "hola").is_ok());
        assert!(re("hola+".to_string(), "holaaaaaa").is_ok());
        assert!(re("(ho)+la+".to_string(), "hoholaaaaaa").is_ok());
    }

}
