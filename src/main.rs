#![allow(unused_variables)]
#![allow(dead_code)]

mod res;
mod tokens;

use res::lexer::*;
use res::token::*;
use res::token_relation::*;

use tokens::programming::*;

use std::io::{Read, stdin};

fn main() {
    let mut stdin = stdin().lock();
    // =================================

    let mut lexer = Lexer::new();

    // lexer.string = "Hello, World. int char bool a cat".to_string();

    let a = Token::new(vec!["a".to_string()], "a".to_string());

    lexer.add_token(Type::as_token());
    lexer.add_token(a.clone());
    lexer.add_token(Animal::as_token());
    lexer.add_token(Semicolom::as_token());
    lexer.add_token(Variable::as_token());
    lexer.add_token(EqualiSign::as_token());

    lexer.add_relation(VariableDeclaration::as_relation());
    lexer.add_relation(VariableModification::as_relation());
    // lexer.add_relation(TokenRelation::new(vec![a.clone()], false, "A".to_string()));

    // lexer.print_detail();

    let mut str = String::new();

    println!("Press CTRL+D to stop taking inputs");
    let _ = stdin.read_to_string(&mut str).unwrap();
    lexer.string = str;

    lexer.print_detail();

    // println!("{:?}", lexer.interpret_tokens_results());

    // =================================

    let relation0 = 0;

    // let relations1 = TokenRelation::new(
    //     vec![
    //         Type::as_token(),
    //         UnknownToken::as_token(),
    //         UnknownToken::as_token(),
    //     ],
    //     true,
    // );

    // let relation2 = TokenRelation::new(
    //     vec![
    //         Animal::as_token(),
    //         FillerToken::as_token(), // specific?
    //         Animal::as_token(),
    //     ],
    //     true,
    // );
}

pub struct Animal {
    given_name: String,
}
impl TokenTrait for Animal {
    fn identifiers() -> Vec<String> {
        vec!["Cat".to_string()]
    }

    fn name() -> String {
        let a = "a";
        "Animal".to_string()
    }

    fn case_sensetive() -> bool {
        true
    }

    fn prefix() -> Option<String> {
        None
    }

    fn suffix() -> Option<String> {
        None
    }
    // fn care_about_body() -> bool {
    //     todo!()
    // }
    fn can_be_filler() -> bool {
        true
    }
}
