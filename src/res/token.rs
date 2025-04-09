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
        // Token {
        //     identifiers: Self::identifiers(),
        //     name: Self::name(),
        //     case_sensetive: Self::case_sensetive(),
        //     prefix: Self::prefix(),
        //     suffix: Self::suffix(),
        // }
        TokenBuilder::new()
            .identifiers_set(Self::identifiers())
            .name_set(Self::name())
            .case_sensetive_set(Self::case_sensetive())
            .prefix_set(Self::prefix())
            .suffix_set(Self::suffix())
            .build()
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
    token_type: TokenType, // do not put str/input in here, Token is A CELL, not an actuall use struct
}
#[derive(Clone, Debug)]
pub enum TokenType {
    Normal,
    Filler {
        set_amount: Option<usize>,
        set_tokesn: Option<Vec<Token>>,
    },
}
impl Default for TokenType {
    fn default() -> Self {
        TokenType::Normal
    }
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
        TokenBuilder::new()
            .identifiers_set(identifiers)
            .name_set(name)
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
impl Default for Token {
    fn default() -> Self {
        Self {
            identifiers: Default::default(),
            name: Default::default(),
            case_sensetive: Default::default(),
            prefix: Default::default(),
            suffix: Default::default(),
            token_type: TokenType::default(),
        }
    }
}

// experimental

pub struct TokenBuilder {
    identifiers: Vec<String>,
    name: String,
    case_sensetive: bool,
    prefix: Option<String>,
    suffix: Option<String>,
    token_type: TokenType,
}
impl TokenBuilder {
    pub fn identifiers_set(mut self, identifiers: Vec<String>) -> Self {
        self.identifiers = identifiers;
        self
    }
    pub fn identifiers_add(mut self, mut identifiers: Vec<String>) -> Self {
        self.identifiers.append(&mut identifiers);
        self
    }
    pub fn name_set(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn case_sensetive_set(mut self, case_sensetive: bool) -> Self {
        self.case_sensetive = case_sensetive;
        self
    }

    pub fn prefix_set(mut self, prefix: Option<String>) -> Self {
        self.prefix = prefix;
        self
    }

    pub fn suffix_set(mut self, suffix: Option<String>) -> Self {
        self.suffix = suffix;
        self
    }

    pub fn identifiers_get(&self) -> &Vec<String> {
        &self.identifiers
    }

    pub fn name_get(&self) -> &String {
        &self.name
    }

    pub fn case_sensetive_get(&self) -> bool {
        self.case_sensetive
    }

    pub fn prefix_get(&self) -> &Option<String> {
        &self.prefix
    }

    pub fn suffix_get(&self) -> &Option<String> {
        &self.suffix
    }

    pub fn build(&self) -> Token {
        Token {
            identifiers: self.identifiers.clone(),
            name: self.name.clone(),
            case_sensetive: self.case_sensetive,
            prefix: self.prefix.clone(),
            suffix: self.suffix.clone(),
            token_type: self.token_type.clone(),
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
            // :\
            token_type: value.token_type,
        }
    }
}
