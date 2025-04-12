use std::sync::Arc;

use crate::{
    arc,
    res::{
        token::TokenTrait,
        token_relation::{RelationToken, TokenRelationTrait},
    },
};

pub struct Color;
impl TokenTrait for Color {
    fn identifiers() -> Arc<[&'static str]> {
        arc!("")
    }

    fn name() -> &'static str {
        "Color"
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
}
struct ColoredThing;
impl TokenRelationTrait for ColoredThing {
    fn ordered() -> bool {
        true
    }

    fn relation() -> Arc<[RelationToken]> {
        arc!(
            RelationToken::Normal(Color::as_token()),
            RelationToken::Advanced {
                set_amount: Some(1),
                set_tokens: None
            }
        )
    }

    fn name() -> &'static str {
        "A Colored Thing"
    }
}
