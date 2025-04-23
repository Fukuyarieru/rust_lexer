use std::{ ops::{ Deref, Index, Not }, sync::Arc };

// use crate::res::lexer::*;
use crate::res::token::*;

use super::lexer::potential_tokens::PotentialTokens;

pub trait TokenRelationTrait {
    // Note: Do not add lifetimes
    fn as_relation() -> TokenRelation {
        TokenRelation::new(Self::relation(), Self::settings(), Self::name())
    }
    fn ordered() -> bool {
        Self::settings().ordered
    }
    fn settings() -> TokenRelationSettings;
    fn relation() -> Arc<[RelationToken]>;
    fn name() -> &'static str;
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

            // tokens_copy.is_empty() // :\??
            true
        }
    }
    fn check_ordered();
    fn check_potential(tokens: &[PotentialTokens]) -> bool {
        if Self::relation().len().eq(&tokens.len()).not() {
            return false;
        }
        if Self::ordered() {
            for (i, t) in Self::relation().iter().enumerate() {
                t.check_tokens(&tokens[i].tokens())
                    .not()
                    .then(|| {
                        return false;
                    });
            }
            return true;
        } else {
            let mut tokens = tokens.deref().clone();
            for rel in Self::relation().iter() {
                let a = tokens.iter().position(|f| rel.check_tokens(f.tokens()));
                a.is_some
            }
        }
        return true;
    }
    // maybe
    fn add_tokens_automatically_to_lexer() -> bool {
        true
    }
    fn prefixes() -> Arc<[&'static str]> {
        Self::settings().prefixes.clone()
    }
    fn suffixes() -> Arc<[&'static str]> {
        Self::settings().suffixes.clone()
    }
}
#[derive(Clone, Debug)]
pub struct TokenRelation {
    relation: Arc<[RelationToken]>,
    settings: TokenRelationSettings,
    name: &'static str,
}
impl TokenRelation {
    pub fn new(
        tokens: Arc<[RelationToken]>,
        settings: TokenRelationSettings,
        name: &'static str
    ) -> Self {
        Self {
            relation: tokens,
            settings,
            name,
        }
    }
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn relation(&self) -> Arc<[RelationToken]> {
        self.relation.clone()
    }
    pub fn ordered(&self) -> bool {
        self.settings.ordered.clone()
    }
    pub fn check(&self, tokens: &[Token]) -> bool {
        if tokens.len() != self.relation.len() {
            return false;
        }
        if self.ordered() {
            for (i, r) in self.relation().iter().enumerate() {
                r.check_token(&tokens[i]).not().then_some(false);
            }
            true
        } else {
            let mut tokens_copy = tokens.to_vec();
            // let mut check_copy = self.relation.clone();

            for relation_token in &self.relation.to_vec() {
                if let Some(index) = tokens_copy.iter().position(|t| relation_token.check_token(t)) {
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
#[derive(Debug)]
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
            Self::Advanced { set_amount, set_tokens } => {
                let a = 3;
                set_tokens.as_ref().and_then(|vec|
                    vec
                        .iter()
                        .any(|t| t.eq(token))
                        .then_some(true)
                );
                false
            }
        }
    }
    pub fn check_tokens(&self, tokens: &[Token]) -> bool {
        tokens.iter().any(|t| self.check_token(t))
    }
}
#[derive(Clone, Debug)]
pub struct TokenRelationSettings {
    pub ordered: bool,
    pub suffixes: Arc<[&'static str]>,
    pub prefixes: Arc<[&'static str]>,
}
