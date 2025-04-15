use super::token::Token;

pub struct PotentialTokens {
    // In the case where more than one token contains matching identifiers, PotenialTokens could represent them unified together and used for diffitiriation
    tokens: Vec<Token>,
}
impl PotentialTokens {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}

pub struct TokenCombination {
    ordered_tokens: Vec<Token>,
    // as a string: [P][TokenOne][S][P][TokenTwo][S]
}
