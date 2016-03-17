use std::fs::File;
use std::io::Read;

#[derive(Eq, PartialEq, Debug, Clone)]
enum TokenType {
    Identifier,
    Reserved,
    Semicolon,
    Number,
    None
}

#[derive(Debug)]
pub struct Token {
    text: String,
    line: usize,
    column: usize,
    ttype: TokenType
}

impl Token {
    fn new(text: &str, line: usize, column: usize, ttype: TokenType) -> Token {
        Token{
            text: String::from(text),
            line: line,
            column: column,
            ttype: ttype
        }
    }

    fn infer_type(&mut self) {
        self.ttype = match self.text.as_str() {
            ";" => TokenType::Semicolon,
            "function" => TokenType::Reserved,
            _ => TokenType::Identifier,
        }
    }
}


struct Lexer {
    index: usize,
    line: usize,
    source: String,
    chars: Vec<char>,
    len: usize,
    line_offset: usize,
}

impl Lexer {
    fn new(source: String) -> Lexer {
        let chars = source.chars().collect::<Vec<char>>();
        let len = chars.len();
        Lexer {
            chars: chars,
            len: len,
            source: source,
            line: 1,
            index: 0,
            line_offset: 0,
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let mut c = self.chars[self.index];

        while c.is_whitespace() {
            self.index += 1;
            if self.index == self.len {
                return None
            }

            if c.escape_default().collect::<String>().as_str() == "\\n" {
                self.line += 1;
                self.line_offset = self.index;
            }

            c = self.chars[self.index];
        }

        if c.is_alphabetic() {
            let mut token = Token::new("", self.line, self.index - self.line_offset, TokenType::None);
            while c.is_alphanumeric() {
                token.text.push(c);
                self.index += 1;
                c = self.chars[self.index];
            }
            token.infer_type();
            return Some(token);
        }

        if c.is_numeric() {
            let mut token = Token::new("", self.line, self.index - self.line_offset, TokenType::Number);
            while c.is_numeric() {
                token.text.push(c);
                self.index += 1;
                c = self.chars[self.index];
            }
            return Some(token);
        }

        if c == ';' {
            let token = Token::new(";", self.line, self.index - self.line_offset, TokenType::Semicolon);
            self.index += 1;
            return Some(token);
        }

        None
    }
}


fn main() {
    let mut file = File::open("demo-source.txt").unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    println!("{}", source);

    let lexer = Lexer::new(source);
    for t in lexer {
        println!("{:?}", t)
    }
}
