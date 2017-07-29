
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub category: String,
    pub lexeme: String,
}


pub fn lex(s: String) -> Vec<Token> {
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

        tokens.push(Token {
                        category: cat.to_string(),
                        lexeme: lexeme.to_string(),
                    });
    }

    tokens.push(Token {
                    category: "EOF".to_string(),
                    lexeme: "".to_string(),
                });

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_test() {
        let cases = vec![
            ("a", vec![("Lit", "a"), ("EOF", "")]),
            ("aa", vec![("Lit", "a"), ("Lit", "a"), ("EOF", "")]),
            ("a|b|c", vec![("Lit", "a"),
                            ("|", ""),
                            ("Lit", "b"),
                            ("|", ""),
                            ("Lit", "c"),
                            ("EOF", "")]),
            ("(a)", vec![("(", ""), ("Lit", "a"), (")", ""), ("EOF", "")]),
            ("a(b)", vec![("Lit", "a"), ("(", ""), ("Lit", "b"), (")", ""), ("EOF", "")]),
            //"(a|b)",
            //"(a|b)*",
        ];

        for (c, e) in cases {
            let expected: Vec<Token> = e.iter()
                .map(|&(cat, lexeme)| {
                         Token {
                             category: cat.to_string(),
                             lexeme: lexeme.to_string(),
                         }
                     })
                .collect();
            assert_eq!(lex(c.to_string()), expected);
        }
    }
}
