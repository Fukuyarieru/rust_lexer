use std::sync::Arc;

use crate::{
    arc,
    res::{
        lexer::UnknownToken,
        token::TokenTrait,
        token_relation::{RelationToken, TokenRelationTrait},
    },
};

pub struct Sentence;
impl TokenRelationTrait for Sentence {
    fn ordered() -> bool {
        true
    }

    fn relation() -> Arc<[RelationToken]> {
        arc![
            RelationToken::Normal(UnknownToken::as_token()),
            RelationToken::Advanced {
                set_amount: None,
                set_tokens: None,
            },
            RelationToken::Normal(UnknownToken::as_token())
        ]
    }

    fn name() -> &'static str {
        "Sentence"
    }
}
