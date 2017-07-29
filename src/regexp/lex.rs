
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
