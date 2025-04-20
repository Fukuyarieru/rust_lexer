use std::sync::Arc;

use crate::{
    arc,
    res::token_relation::{RelationToken, TokenRelationTrait},
};

pub struct Sentence;
impl TokenRelationTrait for Sentence {
    fn ordered() -> bool {
        true
    }

    fn relation() -> Arc<[RelationToken]> {
        arc![
            RelationToken::Advanced { set_amount: Some(1), set_tokens: None },
            RelationToken::Advanced {
                set_amount: None,
                set_tokens: None,
            },
            RelationToken::Advanced { set_amount: Some(1), set_tokens: None }
        ]
    }

    fn name() -> &'static str {
        "Sentence"
    }
    
    fn settings() -> crate::res::token_relation::TokenRelationSettings {
        todo!()
    }
    
    fn prefixes() -> Arc<[&'static str]> {
        todo!()
    }
    
    fn suffixes() -> Arc<[&'static str]> {
        todo!()
    }
}
