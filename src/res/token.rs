// use super::lexer::UnknownToken;

// use crate::FillerToken;

use std::sync::Arc;

use crate::arc;

use super::token_relation::RelationToken;

pub trait TokenTrait {
    // Note: Do not add lifetimes
    fn identifiers() -> Arc<[&'static str]>;
    fn name() -> &'static str;
    fn check(str: &str) -> bool {
        Self::as_token().check(str)
    }
    fn as_token() -> Token {
        Token {
            name: Self::name(),
            identifiers: Self::identifiers(),
            settings: Self::settings(),
        }
    }
    fn as_relation_token() -> RelationToken {
        RelationToken::Normal(Self::as_token())
    }
    fn settings() -> TokenSettings;
    // fn interpret(underfined_token: UndefinedToken) -> Option<Token> {
    //     Self::check(&underfined_token.str).then_some(Self::as_token())
    // }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Token {
    identifiers: Arc<[&'static str]>,
    name: &'static str,
    // case_sensetive: bool,
    // prefix: Option<&'static str>,
    // suffix: Option<&'static str>,
    settings: TokenSettings, // token_type: TokenType, // do not put str/input in here, Token is A CELL, not an actuall use struct
}
impl Token {
    // pub fn case_sensetive(&self) -> bool {
    //     self.case_sensetive
    // }

    pub fn identifiers(&self) -> Arc<[&'static str]> {
        self.identifiers.clone()
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn check(&self, str: &str) -> bool {
        if self.settings.case_sensetive_get() {
            self.identifiers().contains(&str)
        } else {
            self.identifiers()
                .iter()
                .map(|f| f.to_lowercase())
                .collect::<Vec<_>>()
                .contains(&str.to_lowercase())
        }
    }
    pub fn new(
        identifiers: Arc<[&'static str]>,
        name: &'static str,
        settings: TokenSettings,
    ) -> Self {
        Self {
            identifiers,
            name,
            settings,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
#[derive(Debug, Clone)]
pub struct TokenResult {
    token: Token,
    str: String,
}
impl TokenResult {
    pub fn new(token: Token, str: String) -> Self {
        TokenResult { token, str }
    }
    pub fn token(&self) -> Token {
        self.token.clone()
    }
    pub fn str(&self) -> String {
        self.str.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenSettings {
    // Settings for Token
    case_sensetive: bool,
    prefixes: Arc<[&'static str]>,
    suffixes: Arc<[&'static str]>,
}
impl TokenSettings {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn case_sensetive_get(&self) -> bool {
        self.case_sensetive
    }
    pub fn case_sensetive_set(mut self, b: bool) -> Self {
        self.case_sensetive = b;
        self
    }
    pub fn prefix_get(&self) -> Arc<[&'static str]> {
        self.prefixes.clone()
    }
    pub fn prefixes_set(mut self, p: Arc<[&'static str]>) -> Self {
        self.prefixes = p;
        self
    }
    pub fn suffix_get(&self) -> Arc<[&'static str]> {
        self.suffixes.clone()
    }
    pub fn suffixes_set(mut self, s: Arc<[&'static str]>) -> Self {
        self.suffixes = s;
        self
    }
}
impl Default for TokenSettings {
    // set parameters
    fn default() -> Self {
        Self {
            case_sensetive: true,
            prefixes: arc!(),
            suffixes: arc!(),
        }
    }
}

