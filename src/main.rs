#![allow(unused_variables)]
#![allow(dead_code)]

mod example;
mod res;
mod tokens;

use res::lexer::*;
use res::token::*;
use res::token_relation::*;

use tokens::programming::*;

use std::io::{Read, stdin};
use std::sync::Arc;

fn main() {
    let mut stdin = stdin().lock();

    // let a = tokenize!("BBBBBBB");
    // println!("{:?}", a);

    // =================================
    let a = arc!("Dog".to_string());

    let mut lexer = Lexer::new();

    // lexer.add_token(Type::as_token());

    // lexer.string = "int b ;".to_string();
    // lexer.print_detail();
    // println!("{:?}", lexer.interpret_tokens());

    // lexer.check("a b c d e f g".to_string());
    // println!("{:?}", lexer.tokenize());

    // lexer.string = "Hello, World. int char bool a cat".to_string();

    let a = Token::new(arc!["a"], "a");

    lexer.add_token(Type::as_token());
    lexer.add_token(a.clone());
    lexer.add_token(Animal::as_token());
    lexer.add_token(Semicolom::as_token());
    lexer.add_token(Variable::as_token());
    lexer.add_token(EqualiSign::as_token());

    lexer.add_relation(VariableDeclaration::as_relation());
    lexer.add_relation(VariableModification::as_relation());

    // lexer.print_detail();

    let mut str = String::new();

    println!("Press CTRL+D to stop taking inputs");
    let _ = stdin.read_to_string(&mut str).unwrap();
    lexer.string = str;

    lexer.print_detail();
}

pub struct Animal {
    given_name: String,
}
impl TokenTrait for Animal {
    fn identifiers() -> Arc<[&'static str]> {
        arc!["Cat"]
    }

    fn name() -> &'static str {
        "Animal"
    }

    fn case_sensetive() -> bool {
        true
    }

    fn prefix() -> Option<&'static str> {
        None
    }

    fn suffix() -> Option<&'static str> {
        None
    }
}
