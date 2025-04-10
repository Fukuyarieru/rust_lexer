use std::ops::Not;

// use crate::res::lexer::*;
use crate::res::token::*;
// TODO, REDO
pub trait TokenRelationTrait {
    // Note: Do not add lifetimes
    fn as_relation() -> TokenRelation {
        TokenRelation::new(Self::relation(), Self::ordered(), Self::name())
    }
    fn ordered() -> bool;
    fn relation() -> Vec<RelationToken>;
    fn name() -> String;
    fn check(tokens: &[Token]) -> bool {
        if tokens.len() != Self::relation().len() {
            return false;
        }
        let checks = Self::relation();
        if Self::ordered() {
            for (i, rel_tok) in checks.iter().enumerate() {
                rel_tok.check_token(&tokens[i]).not().then_some(false);
            }
            true
        } else {
            let mut tokens_copy = tokens.to_vec();
            for rel_tok in checks.iter() {
                if let Some(index) = tokens_copy.iter().position(|t| rel_tok.check_token(t)) {
                    tokens_copy.remove(index);
                } else {
                    return false;
                }
            }
            // for relation_token in &self.relation {
            //     if let Some(index) = tokens_copy
            //         .iter()
            //         .position(|t| relation_token.check_token(t))
            //     {
            //         tokens_copy.remove(index);
            //     } else {
            //         return false;
            //     }
            // }
            tokens_copy.is_empty() // :\??
        }
    }
    // maybe
    fn add_tokens_automatically_to_lexer() -> bool {
        true
    }
}
#[derive(Clone, Debug)]
pub struct TokenRelation {
    relation: Vec<RelationToken>,
    ordered: bool,
    name: String,
}
impl TokenRelation {
    pub fn new(tokens: Vec<RelationToken>, ordered: bool, name: String) -> Self {
        Self {
            relation: tokens,
            ordered,
            name,
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn relation(&self) -> Vec<RelationToken> {
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
            for (i, r) in self.relation().iter().enumerate() {
                r.check_token(&tokens[i]).not().then_some(false);
            }
            true
            // match r {
            //     RelationToken::Normal(token) => {
            //         if !tokens[i].eq(token) {
            //             return false;
            //         }
            //     }
            //     RelationToken::Advanced {
            //         set_amount: amount,
            //         set_tokens: set_tokens,
            //     } => {
            //         set_tokens.iter().for_each(|t| {
            //             t.eq(&tokens[i]).then(|| return true);
            //         });
            //         return false;
            //     }
            // }
            // return true;
        } else {
            let mut tokens_copy = tokens.to_vec();
            // let mut check_copy = self.relation.clone();

            for relation_token in &self.relation {
                if let Some(index) = tokens_copy
                    .iter()
                    .position(|t| relation_token.check_token(t))
                {
                    tokens_copy.remove(index);
                } else {
                    return false;
                }
            }
            // tokens_copy.is_empty() // strange - ;) - should be true
            true
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
// pub struct FillerToken {
//     set_tokens: Option<Vec<Token>>,
//     set_amount: Option<usize>,
// }
// impl FillerToken {
//     pub fn from(set_tokens: Option<Vec<Token>>, set_amount: Option<usize>) -> Token {
//         // ehhh?
//         // TODO finish this, set_tokens.and_then(|f| {f.iter()})
//         Self {
//             set_tokens,
//             set_amount,
//         };
//         todo!()
//     }
// }
// impl TokenTrait for FillerToken {
//     fn identifiers() -> Vec<String> {
//         vec![] //thing
//     }
//     fn name() -> String {
//         "Filler Token".to_string()
//     }
//     fn case_sensetive() -> bool {
//         true
//     }
//     fn prefix() -> Option<String> {
//         None
//     }
//     fn suffix() -> Option<String> {
//         None
//     }
//     fn can_be_filler() -> bool {
//         true
//     }
// }
// enum ConfToken {
//     Custom(Token),
//     FillerTokenAnyCounted {},
//     FillerTokenSpecific,
// }
#[derive(Clone, Debug, PartialEq)]
pub enum RelationToken {
    Normal(Token),
    Advanced {
        set_amount: Option<usize>,
        set_tokens: Option<Vec<Token>>,
    },
}
impl RelationToken {
    pub fn check_token(&self, token: &Token) -> bool {
        match self {
            Self::Normal(t) => t.eq(token),
            Self::Advanced {
                set_amount,
                set_tokens,
            } => {
                let a = 3;
                set_tokens
                    .as_ref()
                    .and_then(|vec| vec.iter().any(|t| t.eq(token)).then_some(true));
                false
            }
        }
    }
    pub fn check_tokens(&self, tokens: &[Token]) -> bool {
        tokens.iter().any(|t| self.check_token(t))
    }
}
