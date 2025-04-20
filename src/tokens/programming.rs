use std::sync::Arc;

use crate::arc;
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
            RelationToken::Advanced {
                set_amount: Some(1),
                set_tokens: None
            },
            RelationToken::Normal(Semicolom::as_token())
        ]
    }
    fn name() -> &'static str {
        "Variable Declaration"
    }
    fn prefixes() -> Arc<[&'static str]> {
        todo!()
    }
    fn suffixes() -> Arc<[&'static str]> {
        todo!()
    }
    fn settings() -> TokenRelationSettings {
        todo!()
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
    fn settings() -> TokenRelationSettings {
        todo!()
    }
    fn prefixes() -> Arc<[&'static str]> {
        todo!()
    }
    fn suffixes() -> Arc<[&'static str]> {
        todo!()
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

    fn settings() -> TokenSettings {
        TokenSettings::new()
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
    fn settings() -> TokenSettings {
        TokenSettings::new()
            .case_sensetive_set(false)
            .prefixes_set(arc!())
            .suffixes_set(arc!())
    }
}
pub struct Variable {}
impl TokenTrait for Variable {
    fn identifiers() -> Arc<[&'static str]> {
        arc!() // aaa
    }

    fn name() -> &'static str {
        "Variable"
    }

    fn settings() -> TokenSettings {
        TokenSettings::new()
    }
}
pub struct Type {}
impl TokenTrait for Type {
    fn identifiers() -> Arc<[&'static str]> {
        // arc!("char", "int", "bool")
        Arc::from(["char", "int", "bool"])
    }
    fn name() -> &'static str {
        "Type"
    }
    fn settings() -> TokenSettings {
        TokenSettings::new()
    }
}
