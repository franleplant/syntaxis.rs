use std::cell::Cell;

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
pub struct Parser {
    index: Cell<usize>,
    src: String,
    tokens: Vec<Token>
}

impl Parser {
    fn new(src: String) -> Parser {
        let tokens = lex(src.clone());
        Parser {
            index: Cell::new(0),
            src: src,
            tokens: tokens,
        }
    }


    fn next(&self) {
        self.index.set(self.index.get() + 1);
    }

    //fn print(&self) {
        
    //}

    fn parse(&self) -> bool {
        return self.re();
    }

    fn re(&self) -> bool {
        let token = self.tokens.get(self.index.get()).expect("Re panic");

        println!("Re {:?}", token);

        match token.category.as_str() {
            "Lit" => {
                self.next();
                return self.ops();
            },

            "(" => {
                self.next();
                if self.re() {
                    let token = self.tokens.get(self.index.get()).expect("Re panic");
                    if token.category == ")" {
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
            // First
            "|" => {
                self.next();
                return self.re();
            },
            // First
            "*" | "+" => {
                self.next();
                return true;
            },

            // Follow
            "Lit" | "(" => {
                return self.re();
            },

            // Follow
            ")" => {
                return true;
            },

            // Follow
            "EOF" => {
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
            "a|(cde)*a+",
            "((((aaa))))",
        ];

        let expect = vec![
            true,
            true,
            true,
            true,
        ];

        for (c, e) in cases.iter().zip(expect.iter()) {
            println!("\nCase {:?}\n", c);
            let p = Parser::new(c.to_string());
            assert_eq!(p.parse(), *e, "In {:?}", c);
        }
    }
}
