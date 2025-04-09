// use crate::res::lexer::*;
use crate::res::token::*;
// TODO, REDO
pub trait TokenRelationTrait {
    // Note: Do not add lifetimes
    fn as_relation() -> TokenRelation {
        TokenRelation::new(Self::tokens(), Self::ordered(), Self::name())
    }
    fn ordered() -> bool;
    fn tokens() -> Vec<Token>;
    fn name() -> String;
    fn check(tokens: &[Token]) -> bool {
        if tokens.len() != Self::tokens().len() {
            return false;
        }
        let checks = Self::tokens();
        if Self::ordered() {
            for (i, _) in checks.iter().enumerate() {
                if checks[i].eq(&tokens[i]) {
                    return false;
                }
            }
        } else {
            let mut tokens_copy = tokens.to_vec();
            for token in checks.iter() {
                if let Some(index) = tokens_copy.iter().position(|t| t.eq(token)) {
                    tokens_copy.remove(index);
                } else {
                    return false;
                }
            }
            return tokens_copy.is_empty();
        }
        false
    }
    // maybe
    fn add_tokens_automatically_to_lexer() -> bool {
        true
    }
}
#[derive(Clone, Debug)]
pub struct TokenRelation {
    relation: Vec<Token>,
    ordered: bool,
    name: String,
}
impl TokenRelation {
    pub fn new(tokens: Vec<Token>, ordered: bool, name: String) -> Self {
        Self {
            relation: tokens,
            ordered,
            name,
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn relation(&self) -> Vec<Token> {
        self.relation.clone()
    }
    pub fn ordered(&self) -> bool {
        self.ordered
    }
    pub fn check(&self, tokens: &[Token]) -> bool {
        if tokens.len() != self.relation.len() {
            return false;
        }
        if self.ordered {
            for (i, _) in self.relation().iter().enumerate() {
                if !self.relation[i].eq(&tokens[i]) {
                    return false;
                }
            }
            true
        } else {
            let mut tokens_copy = tokens.to_vec();
            for check_token in self.relation.iter() {
                if let Some(index) = tokens_copy.iter().position(|t| t.eq(check_token)) {
                    tokens_copy.remove(index);
                } else {
                    return false;
                }
            }
            tokens_copy.is_empty() // strange - ;) - should be true
        }
    }
}
pub struct TokenRelationResult {
    relation: TokenRelation,
    str: String,
    // some info like index-es, detail inside about tokens and etc
}
impl TokenRelationResult {
    pub fn new(relation: TokenRelation, str: String) -> Self {
        Self { relation, str }
    }
    pub fn relation(&self) -> TokenRelation {
        self.relation.clone()
    }
    pub fn str(&self) -> String {
        self.str.clone()
    }
}

// pub enum FillerToken {
//     Specific(Vec<Token>), // Temporary untill the idea on how to do this is complete
//     Any, // any is for a case like, [start,word,word,word,finish]-this qualifies as a sentece, and word can be anything.
//          // to be more precise: [Start, <ANYTHING IN BEtWEEN>, Finish]-we are supposed to recognize this as Start to finish, so we need any in this case
// }
pub struct FillerToken {
    set_tokens: Option<Vec<Token>>,
    set_amount: Option<usize>,
}
impl FillerToken {
    pub fn from(set_tokens: Option<Vec<Token>>, set_amount: Option<usize>) -> Token {
        // ehhh?
        // TODO finish this, set_tokens.and_then(|f| {f.iter()})
        Self {
            set_tokens,
            set_amount,
        };
        todo!()
    }
}
impl TokenTrait for FillerToken {
    fn identifiers() -> Vec<String> {
        vec![] //thing
    }
    fn name() -> String {
        "Filler Token".to_string()
    }
    fn case_sensetive() -> bool {
        true
    }
    fn prefix() -> Option<String> {
        None
    }
    fn suffix() -> Option<String> {
        None
    }
    fn can_be_filler() -> bool {
        true
    }
}
// enum ConfToken {
//     Custom(Token),
//     FillerTokenAnyCounted {},
//     FillerTokenSpecific,
// }
