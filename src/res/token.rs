// use super::lexer::UnknownToken;

// use crate::FillerToken;

use std::sync::Arc;

use super::token_relation::RelationToken;

pub trait TokenTrait {
    // Note: Do not add lifetimes
    fn identifiers() -> Arc<[String]>;
    fn name() -> String;
    fn check(str: &str) -> bool {
        if Self::case_sensetive() {
            Self::identifiers().contains(&str.to_string())
        } else {
            Self::identifiers()
                .iter()
                .map(|f| f.to_lowercase())
                .collect::<Vec<String>>()
                .contains(&str.to_lowercase())
        }
    }
    fn case_sensetive() -> bool;
    fn as_token() -> Token {
        TokenBuilder::new()
            .identifiers(Self::identifiers())
            .name(Self::name())
            .case_sensetive(Self::case_sensetive())
            .prefix(Self::prefix())
            .suffix(Self::suffix())
            .build()
    }
    fn can_be_filler() -> bool;
    fn prefix() -> Option<String>;
    fn suffix() -> Option<String>;
    // fn token_result(&self) -> TokenResult {};
    fn as_relation_token() -> RelationToken {
        RelationToken::Normal(Self::as_token())
    }
    // fn interpret(underfined_token: UndefinedToken) -> Option<Token> {
    //     Self::check(&underfined_token.str).then_some(Self::as_token())
    // }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Token {
    identifiers: Arc<[String]>,
    name: String,
    case_sensetive: bool,
    prefix: Option<String>,
    suffix: Option<String>,
    // token_type: TokenType, // do not put str/input in here, Token is A CELL, not an actuall use struct
}
impl Token {
    pub fn case_sensetive(&self) -> bool {
        self.case_sensetive
    }

    pub fn identifiers(&self) -> Arc<[String]> {
        self.identifiers.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn prefix(&self) -> Option<String> {
        self.prefix.clone()
    }

    pub fn suffix(&self) -> Option<String> {
        self.suffix.clone()
    }
    pub fn check(&self, str: &str) -> bool {
        if self.case_sensetive {
            self.identifiers().contains(&str.to_string())
        } else {
            self.identifiers()
                .iter()
                .map(|f| f.to_lowercase())
                .collect::<Vec<_>>()
                .contains(&str.to_lowercase())
        }
    }
    // fn interpret(&self, underfined_token: UndefinedToken) -> Option<Token> {
    //     self.check(&underfined_token.str).then_some(self.clone())
    // }
    pub fn new(identifiers: Arc<[String]>, name: String) -> Self {
        TokenBuilder::new()
            .identifiers(identifiers)
            .name(name)
            .build()
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.clone())
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
pub struct TokenFillters {
    // TODO
    case_sensetive: bool,
}

// experimental

pub struct TokenBuilder {
    identifiers: Arc<[String]>,
    name: String,
    case_sensetive: bool,
    prefix: Option<String>,
    suffix: Option<String>,
    // token_type: TokenType,
}
impl TokenBuilder {
    pub fn identifiers(mut self, identifiers: Arc<[String]>) -> Self {
        self.identifiers = identifiers;
        self
    }
    // pub fn identifiers_add(mut self, mut identifiers: Vec<String>) -> Self {
    //     self.identifiers.append(&mut identifiers);
    //     self
    // }
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    pub fn case_sensetive(mut self, case_sensetive: bool) -> Self {
        self.case_sensetive = case_sensetive;
        self
    }
    pub fn prefix(mut self, prefix: Option<String>) -> Self {
        self.prefix = prefix;
        self
    }
    pub fn suffix(mut self, suffix: Option<String>) -> Self {
        self.suffix = suffix;
        self
    }
    pub fn build(&self) -> Token {
        Token {
            identifiers: self.identifiers.clone(),
            name: self.name.clone(),
            case_sensetive: self.case_sensetive,
            prefix: self.prefix.clone(),
            suffix: self.suffix.clone(),
            // token_type: self.token_type.clone(),
        }
    }
    pub fn new() -> TokenBuilder {
        TokenBuilder::from(Token::default())
    }
}
impl From<Token> for TokenBuilder {
    fn from(value: Token) -> Self {
        Self {
            case_sensetive: value.case_sensetive,
            identifiers: value.identifiers,
            name: value.name,
            prefix: value.prefix,
            suffix: value.suffix,
        }
    }
}

#[macro_export]
macro_rules! tokenize {
    ($name:expr) => {{
        let token = TokenBuilder::new();
        let name = String::from($name);
        token
            .name(name.clone())
            .identifiers(vec![name.clone()])
            .case_sensetive(true)
            .build()
    }};
}

#[macro_export]
macro_rules! arc {
    ($($x:expr),*) => {{
        let mut vec = Vec::new();
        $(
            vec.push($x);
        )*
        let vec=vec;
        std::sync::Arc::new(vec.as_slice())
    }};
}
