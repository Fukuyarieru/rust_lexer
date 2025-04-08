use crate::res::lexer::*;
use crate::res::token::*;
use crate::res::token_relation::*;

pub struct VariableDeclaration {}
impl TokenRelationTrait for VariableDeclaration {
    fn ordered() -> bool {
        true
    }
    fn tokens() -> Vec<Token> {
        vec![
            Type::as_token(),
            // Variable::as_token(),
            UnknownToken::as_token(),
            Semicolom::as_token(),
        ]
    }
    fn name() -> String {
        "Variable Declaration".to_string()
    }
}
pub struct VariableModification {}
impl TokenRelationTrait for VariableModification {
    fn ordered() -> bool {
        true
    }
    fn name() -> String {
        "Variable Modification".to_string()
    }
    fn tokens() -> Vec<Token> {
        vec![
            UnknownToken::as_token(),
            EqualiSign::as_token(),
            UnknownToken::as_token(),
            // Variable::as_token(),
            // EqualiSign::as_token(),
            // Variable::as_token(),
        ]
    }
}
pub struct EqualiSign {}
impl TokenTrait for EqualiSign {
    fn identifiers() -> Vec<String> {
        vec!["=".to_string()]
    }

    fn name() -> String {
        "Equal Sign".to_string()
    }

    fn case_sensetive() -> bool {
        false
    }

    fn prefix() -> Option<String> {
        None
    }

    fn suffix() -> Option<String> {
        None
    }

    fn can_be_filler() -> bool {
        false
    }
}

pub struct Semicolom {}
impl TokenTrait for Semicolom {
    fn identifiers() -> Vec<String> {
        vec![";".to_string()]
    }

    fn name() -> String {
        "Semicolom".to_string()
    }

    fn case_sensetive() -> bool {
        false
    }

    fn prefix() -> Option<String> {
        None
    }

    fn suffix() -> Option<String> {
        None
    }

    fn can_be_filler() -> bool {
        false
    }
}
pub struct Variable {}
impl TokenTrait for Variable {
    fn identifiers() -> Vec<String> {
        UnknownToken::identifiers() // aaa
    }

    fn name() -> String {
        "Variable".to_string()
    }

    fn case_sensetive() -> bool {
        false
    }

    fn prefix() -> Option<String> {
        None
    }

    fn suffix() -> Option<String> {
        None
    }

    fn can_be_filler() -> bool {
        false
    }
}
pub struct Type {}
impl TokenTrait for Type {
    fn identifiers() -> Vec<String> {
        vec![
            String::from("int"),
            String::from("char"),
            String::from("bool"),
        ]
    }
    fn name() -> String {
        "Type".to_string()
    }
    fn case_sensetive() -> bool {
        false
    }
    fn prefix() -> Option<String> {
        None
    }
    fn suffix() -> Option<String> {
        None
    }
    fn can_be_filler() -> bool {
        false
    }
}
