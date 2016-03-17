mod lexer;

use std::fs::File;
use std::io::Read;
use lexer::Lexer;

fn main() {
    let mut file = File::open("demo-source.txt").unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    println!("{}", source);

    let lexer = Lexer::new(source);
    for token in lexer {
        println!("{: >2} {: >2} {:>15}   {:>15}", token.line, token.column, token.text, token.token_type)
    }
}
