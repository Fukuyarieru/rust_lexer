use std::sync::Arc;

use crate::{
    arc,
    res::{
        token::{ TokenSettings, TokenTrait },
        token_relation::{ RelationToken, TokenRelationTrait },
    },
};

pub struct Color;
impl TokenTrait for Color {
    fn identifiers() -> Arc<[&'static str]> {
        arc!(
            "Black",
            "White",
            "Red",
            "Blue",
            "Green",
            "Pink",
            "Purple",
            "Yellow",
            "Orange",
            "Brown",
            "Gray"
        )
    }

    fn name() -> &'static str {
        "Color"
    }

    fn settings() -> crate::res::token::TokenSettings {
        TokenSettings::new()
    }
}
struct ColoredThing;
impl TokenRelationTrait for ColoredThing {
    fn ordered() -> bool {
        true
    }

    fn relation() -> Arc<[RelationToken]> {
        arc!(RelationToken::Normal(Color::as_token()), RelationToken::Advanced {
            set_amount: Some(1),
            set_tokens: None,
        })
    }

    fn name() -> &'static str {
        "A Colored Thing"
    }
    fn suffixes() -> Arc<[&'static str]> {
        todo!()
    }
    fn settings() -> crate::res::token_relation::TokenRelationSettings {
        todo!()
    }

    fn prefixes() -> Arc<[&'static str]> {
        todo!()
    }
}
