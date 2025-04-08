// use super::lexer::UnknownToken;

use crate::FillerToken;

pub trait TokenTrait {
    // Note: Do not add lifetimes
    fn identifiers() -> Vec<String>;
    fn name() -> String;
    fn check(&self, str: &str) -> bool {
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
    // fn from_unknown() -> UnknownToken {
    //     UnknownToken::from(Self::as_token())
    // }
    // fn from_token(token: impl TokenTrait) -> Self {
    //     token::as_token()
    // }
    fn case_sensetive() -> bool;
    fn as_token() -> Token {
        Token {
            identifiers: Self::identifiers(),
            name: Self::name(),
            case_sensetive: Self::case_sensetive(),
            prefix: Self::prefix(),
            suffix: Self::suffix(),
        }
    }
    // fn as_token_dyn(&self) -> Token {
    //     Self::as_token()
    // }
    fn can_be_filler() -> bool;
    fn prefix() -> Option<String>;
    fn suffix() -> Option<String>;
    // fn get_str(&self) -> String;
    // fn token_result(&self) -> TokenResult {};
    fn as_filler_token() -> FillerToken
    where
        Self: From<Token>,
    {
        // FillerToken::Specific(vec![]);
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    identifiers: Vec<String>,
    name: String,
    case_sensetive: bool,
    prefix: Option<String>,
    suffix: Option<String>,
    // do not put str/input in here, Token is A CELL, not an actuall use struct
}
impl Token {
    pub fn case_sensetive(&self) -> bool {
        self.case_sensetive
    }

    pub fn identifiers(&self) -> Vec<String> {
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
    pub fn new(identifiers: Vec<String>, name: String) -> Self {
        Token {
            identifiers,
            name,
            case_sensetive: false,
            prefix: None,
            suffix: None,
        }
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
