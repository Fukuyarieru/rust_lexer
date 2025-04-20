use crate::res::token::Token;

pub struct TokenCombination {
    ordered_tokens: Vec<Token>,
    // as a string: [P][TokenOne][S][P][TokenTwo][S]
}
