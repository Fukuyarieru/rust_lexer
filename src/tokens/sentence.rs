use crate::res::token_relation::{FillerToken, TokenRelationTrait};

pub struct Sentence;
impl TokenRelationTrait for Sentence {
    fn ordered() -> bool {
        true
    }

    fn tokens() -> Vec<crate::res::token::Token> {
        vec![FillerToken::from(None, Some(3))]
    }

    fn name() -> String {
        todo!()
    }
}
