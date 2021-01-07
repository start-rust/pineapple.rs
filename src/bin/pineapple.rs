use pineapple::parser::parser::Parse;
use pineapple::parser::lexer::Lexer;

fn main() {

    let sr = r#"
let a = "pen pineapple apple pen."
print(a)
        "#;

    let mut parser = Parse::new(Lexer::new(sr));
    println!("{:?}", parser.parse());
}

