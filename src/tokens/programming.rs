use std::sync::Arc;

use crate::arc;
use crate::res::lexer::*;
use crate::res::token::*;
use crate::res::token_relation::*;

pub struct VariableDeclaration {}
impl TokenRelationTrait for VariableDeclaration {
    fn ordered() -> bool {
        true
    }
    fn relation() -> Arc<[RelationToken]> {
        arc![
            RelationToken::Normal(Type::as_token()),
            // Variable::as_token(),
            RelationToken::Normal(UnknownToken::as_token()),
            RelationToken::Normal(Semicolom::as_token())
        ]
    }
    fn name() -> &'static str {
        "Variable Declaration"
    }
}
pub struct VariableModification {}
impl TokenRelationTrait for VariableModification {
    fn ordered() -> bool {
        true
    }
    fn name() -> &'static str {
        "Variable Modification"
    }
    fn relation() -> Arc<[RelationToken]> {
        arc!(
            // RelationToken::Normal(UnknownToken::as_token()),
            RelationToken::Advanced {
                set_amount: Some(1),
                set_tokens: None
            },
            RelationToken::Normal(EqualiSign::as_token()),
            RelationToken::Advanced {
                set_amount: Some(1),
                set_tokens: None
            }
        )
    }
}
pub struct EqualiSign {}
impl TokenTrait for EqualiSign {
    fn identifiers() -> Arc<[&'static str]> {
        arc!["="]
    }

    fn name() -> &'static str {
        "Equal Sign"
    }

    fn case_sensetive() -> bool {
        false
    }

    fn prefix() -> Option<&'static str> {
        None
    }

    fn suffix() -> Option<&'static str> {
        None
    }

    fn can_be_filler() -> bool {
        false
    }
}

pub struct Semicolom {}
impl TokenTrait for Semicolom {
    fn identifiers() -> Arc<[&'static str]> {
        arc!(";")
    }

    fn name() -> &'static str {
        "Semicolom"
    }

    fn case_sensetive() -> bool {
        false
    }

    fn prefix() -> Option<&'static str> {
        None
    }

    fn suffix() -> Option<&'static str> {
        None
    }

    fn can_be_filler() -> bool {
        false
    }
}
pub struct Variable {}
impl TokenTrait for Variable {
    fn identifiers() -> Arc<[&'static str]> {
        UnknownToken::identifiers() // aaa
    }

    fn name() -> &'static str {
        "Variable"
    }

    fn case_sensetive() -> bool {
        false
    }

    fn prefix() -> Option<&'static str> {
        None
    }

    fn suffix() -> Option<&'static str> {
        None
    }

    fn can_be_filler() -> bool {
        false
    }
}
pub struct Type {}
impl TokenTrait for Type {
    fn identifiers() -> Arc<[&'static str]> {
        arc!("char", "int", "bool")
    }
    fn name() -> &'static str {
        "Type"
    }
    fn case_sensetive() -> bool {
        false
    }
    fn prefix() -> Option<&'static str> {
        None
    }
    fn suffix() -> Option<&'static str> {
        None
    }
    fn can_be_filler() -> bool {
        false
    }
}
