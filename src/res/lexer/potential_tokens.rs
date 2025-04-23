use crate::res::token::Token;

pub struct PotentialTokens {
    // In the case where more than one token contains matching identifiers, PotenialTokens could represent them unified together and used for diffitiriation
    tokens: Vec<Token>,
    str: String
}
impl PotentialTokens {
    pub fn new(tokens: Vec<Token>, str: String) -> Self {
        Self { tokens ,str}
    }
    pub fn tokens(&self) -> &[Token] {
        self.tokens.as_slice()
    }
    pub fn add(&mut self, token_result: Token) {
        self.tokens.push(token_result);
    }
}
