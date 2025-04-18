// use super::lexer::UnknownToken;

// use crate::FillerToken;

use std::sync::Arc;

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
pub struct TokenFillters {
    // TODO
    case_sensetive: bool,
}

// experimental

// pub struct TokenBuilder {
//     identifiers: Arc<[&'static str]>,
//     name: &'static str,
//     case_sensetive: bool,
//     prefix: Option<&'static str>,
//     suffix: Option<&'static str>,
//     // token_type: TokenType,
// }
// impl TokenBuilder {
//     pub fn identifiers(mut self, identifiers: Arc<[&'static str]>) -> Self {
//         self.identifiers = identifiers;
//         self
//     }
//     // pub fn identifiers_add(mut self, mut identifiers: Vec<String>) -> Self {
//     //     self.identifiers.append(&mut identifiers);
//     //     self
//     // }
//     pub fn name(mut self, name: &'static str) -> Self {
//         self.name = name;
//         self
//     }
//     pub fn case_sensetive(mut self, case_sensetive: bool) -> Self {
//         self.case_sensetive = case_sensetive;
//         self
//     }
//     pub fn prefix(mut self, prefix: Option<&'static str>) -> Self {
//         self.prefix = prefix;
//         self
//     }
//     pub fn suffix(mut self, suffix: Option<&'static str>) -> Self {
//         self.suffix = suffix;
//         self
//     }
//     pub fn build(&self) -> Token {
//         Token {
//             identifiers: self.identifiers.clone(),
//             name: self.name,
//             // case_sensetive: self.case_sensetive,
//             // prefix: self.prefix,
//             // suffix: self.suffix,
//             settings: self
//             // token_type: self.token_type.clone(),
//         }
//     }
//     pub fn new() -> TokenBuilder {
//         TokenBuilder::from(Token::default())
//     }
// }
// impl From<Token> for TokenBuilder {
//     fn from(value: Token) -> Self {
//         Self {
//             case_sensetive: value.case_sensetive,
//             identifiers: value.identifiers,
//             name: value.name,
//             prefix: value.prefix,
//             suffix: value.suffix,
//         }
//     }
// }

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

#[derive(Debug, Clone, PartialEq)]
pub struct TokenSettings {
    // Settings for Token
    case_sensetive: bool,
    prefix: Option<&'static str>,
    suffix: Option<&'static str>,
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
    pub fn prefix_get(&self) -> Option<&'static str> {
        self.prefix
    }
    pub fn prefix_set(mut self, p: Option<&'static str>) -> Self {
        self.prefix = p;
        self
    }
    pub fn suffix_get(&self) -> Option<&'static str> {
        self.suffix
    }
    pub fn suffix_set(mut self, s: Option<&'static str>) -> Self {
        self.suffix = s;
        self
    }
}
impl Default for TokenSettings {
    // set parameters
    fn default() -> Self {
        Self {
            case_sensetive: true,
            prefix: None,
            suffix: None,
        }
    }
}
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
