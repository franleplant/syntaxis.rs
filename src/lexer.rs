use std::fmt::{Display, Formatter, Error};

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TokenType {
    Identifier,
    Reserved,
    Semicolon,
    Number,
    None
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::Reserved => write!(f, "Reserved"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Number => write!(f, "Number"),
            _ => write!(f, "None")
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub line: usize,
    pub column: usize,
    pub token_type: TokenType
}

impl Token {
    fn new(text: &str, line: usize, column: usize, token_type: TokenType) -> Token {
        Token{
            text: String::from(text),
            line: line,
            column: column,
            token_type: token_type
        }
    }

    fn infer_type(&mut self) {
        self.token_type = match self.text.as_str() {
            ";" => TokenType::Semicolon,
            "function" => TokenType::Reserved,
            _ => TokenType::Identifier,
        }
    }
}


pub struct Lexer {
    index: usize,
    line: usize,
    source: String,
    chars: Vec<char>,
    len: usize,
    line_offset: usize,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        let mut chars = source.chars().collect::<Vec<char>>();
        // Assure that the end of a file is a line feed
        chars.push('\n');
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

            if c == '\n' {
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
