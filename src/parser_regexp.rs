

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

fn re(i: usize, tokens: &Vec<Token>) -> (usize, bool) {
    println!("Re {:?}", tokens[i]);

    let token = tokens.get(i).expect("Re panic");
    let cat = token.category.as_str();

    if cat == "EOF" {
        return (i, true);
    }

    if cat == "Lit" {
        return ops(i + 1, tokens);
    }

    if cat == "(" {
        let (i, res) = re(i + 1, tokens);
        if res {
            let token = tokens.get(i).expect("Re panic");
            if token.category == ")" {
                return ops(i + 1, tokens);
            }
        }
    }

    return (i, false);
}


fn ops(i: usize, tokens: &Vec<Token>) -> (usize, bool) {
    println!("Ops {:?}", tokens[i]);

    let token = tokens.get(i).expect("Ops panic");

    match token.category.as_str() {
        "|" => {
            return re(i + 1, tokens);
        },
        "*" | "+" => {
            return (i + 1, true);
        },

        "Lit" | "(" => {
            return re(i + 1, tokens);
        },

        ")" => {
            return (i, true);
        },


        "EOF" => {
            return (i, true);
        },

        _ => {},
    }

    return (i, false);
}

//#[derive(Debug)]
//pub struct Parser {
    //index: usize,
    //src: String,
    //tokens: Vec<Token>
//}

//impl Parser {
    //fn new(src: String) -> Parser {
        //let tokens = lex(src.clone());
        //Parser {
            //index: 0,
            //src: src,
            //tokens: tokens,
        //}
    //}


    //fn next(&mut self) {
        //self.index += 1;
    //}

    //fn parse(self) -> bool {
        //let (res, _) = re(self);
        //return res
    //}
//}

pub fn parse(src: String) -> bool {
    let tokens = lex(src.clone());
    let i = 0;

    let (_, res) = re(i, &tokens);
    return res;
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
}
