use std::fs::File;
use std::io::Read;


#[derive(Debug)]
enum TokenType {
    Identifier,
    Reserved,
    Semicolon,
    None
}

#[derive(Debug)]
struct Token {
    text: String,
    line: u64,
    column: Option<u64>,
    ttype: TokenType
}

impl Token {
    fn new(text: Option<&str>, line: usize, column: Option<u64>) -> Token {
        let text = String::from(text.unwrap_or(""));

        Token{ text: text, line: line as u64, column: column, ttype: TokenType::None}
    }

    fn new_on_line(line: usize) -> Token {
        Token{ text: String::new(), line: line as u64, column: None, ttype: TokenType::None}
    }

    fn infer_type(&mut self) {
        self.ttype = match self.text.as_str() {
            ";" => TokenType::Semicolon,
            "function" => TokenType::Reserved,
            _ => TokenType::Identifier,
        }
    }

    fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

fn main() {
    let mut file = File::open("demo-source.txt").unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    println!("{}", source);

    //
    let chars: Vec<char> = source.chars().collect();

    let mut token_vec: Vec<Token> = vec![];
    let mut token: Token;
    // TODO:
    // Attempt to produce an iterator that yields tokens
    // return source.lines().enumerate().map(|line, line_str| {
    //   token = ...
    //   return line_str.char_indices().map(|column, c| {
    //     stuff...
    //     return token ?
    //   })
    // })
    for (line, line_string) in source.lines().enumerate() {
        token = Token::new(None, line+1, None);

        println!("{}, {}", line, line_string);
        for (column, c) in line_string.char_indices() {
            if token.column.is_none() {
                token.column = Some(column as u64);
            }

            if c.is_whitespace() {
                if !token.is_empty() {
                    token.infer_type();
                    token_vec.push(token);
                    token = Token::new_on_line(line+1);
                } else {
                    continue;
                }
            } else if c == ';' {
                token.infer_type();
                token_vec.push(token);

                token = Token::new(Some(";"), line+1, Some(column as u64));
                token.infer_type();
                token_vec.push(token);

                token = Token::new_on_line(line+1);
            } else {
                token.text.push(c);
            }
        }
        if !token.is_empty(){
            token.infer_type();
            token_vec.push(token)
        }
    }

    for t in token_vec {
        println!("{:?}", t);
    }
}


