#![allow(clippy::collapsible_if)]
#![allow(unused_imports)]
use std::ops::Deref;

#[allow(unused_imports)]
use crate::arc;

use crate::res::token::*;
use crate::res::token_relation::*;

use super::potential_tokens::PotentialTokens;
use super::unknown_token::UnknownToken;

// To use lexer, either
// make a struct of your own and implement the TokenTrait, add to lexer with lexer.add_token(YourStruct::as_token())
// or else, you could use the Token struct directly to create custom, without assosiactions to your build
pub struct Lexer {
    pub string: String,
    tokens: Vec<Token>,
    relations: Vec<TokenRelation>,
}
impl Lexer {
    pub fn new() -> Self {
        Self {
            string: "".to_string(),
            tokens: Vec::new(),
            relations: Vec::new(),
        }
    }
    pub fn interpret(&mut self, str: String) -> Vec<TokenResult> {
        self.string = str;
        self.interpret_tokens_results()
    }
    pub fn interpret_potential_tokens(&self) -> Vec<PotentialTokens> {
        self.string.split_whitespace().map(|str|
            PotentialTokens::new(
                self
                    .tokens()
                    .into_iter()
                    .filter(|tk| tk.check(str)).collect::<Vec<Token>>(),
                str.to_string()
            )
        ).collect()
    }
    pub fn interpret_tokens_results(&self) -> Vec<TokenResult> {
        self.string
            .split_whitespace()
            .map(|undefined_token| {
                self.tokens()
                    .clone()
                    .iter()
                    .find(|token| token.check(undefined_token))
                    .map(|token| TokenResult::new(token.clone(), undefined_token.to_string()))
                    .unwrap_or_else(|| {
                        TokenResult::new(UnknownToken::as_token(), undefined_token.to_string())
                    })
            })
            .collect()

        // let mut result = Vec::new();
        // let mut checked = false;
        // self.string.replace(" ", "\n").lines().for_each(|str| {
        //     self.tokens.iter().for_each(|token| {
        //         if token.check(str) {
        //             result.push(TokenResult::new(token.clone(), str.to_string()));
        //             checked = true;
        //         }
        //     });
        //     if !checked {
        //         result.push(TokenResult::new(UnknownToken::as_token(), str.to_string()));
        //     }
        //     checked = false;
        // });
        // result
    }
    pub fn interpret_tokens(&self) -> Vec<Token> {
        self.interpret_tokens_results()
            .iter()
            .map(|res| res.token())
            .collect()
    }
    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
    pub fn identifiers(&self) -> Vec<String> {
        let mut identifiers = Vec::new();
        for token in &self.tokens {
            for identifier in token.identifiers().iter() {
                identifiers.push(identifier.to_string());
            }
        }
        identifiers
    }
    pub fn add_relation(&mut self, relation: TokenRelation) {
        self.relations.push(relation);
    }
    pub fn add_relations(&mut self, relations: Vec<TokenRelation>) {
        for relation in relations {
            self.relations.push(relation);
        }
    }
    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
    pub fn add_tokens(&mut self, tokens: Vec<Token>) {
        for token in tokens {
            self.tokens.push(token);
        }
    }
    pub fn add_token_relation(&mut self, relation: TokenRelation) {
        self.relations.push(relation);
    }
    pub fn relations(&self) -> Vec<TokenRelation> {
        self.relations.clone()
    }
    pub fn interpret_relations(&self) -> Vec<TokenRelation> {
        self.interpret_relations_results()
            .iter()
            .map(|rel| rel.relation())
            .collect()
    }
    pub fn interpret_relations_results(&self) -> Vec<TokenRelationResult> {
        let tokens_results = self.interpret_tokens_results();
        let tokens = self.interpret_tokens();
        let relations = self.relations();
        let mut results = Vec::new();

        // check starting at each token
        for (token_index, token) in tokens.iter().enumerate() {
            // for each token check each relation
            for relation in &relations {
                let relation_len = relation.relation().len();
                // check if size is enough for a realtion
                if token_index + relation_len < tokens.len() {
                    // relation checks appropriate tokens
                    if relation.check(&tokens[token_index..token_index + relation_len]) {
                        // TODO, here is a problem
                        let str = tokens_results[token_index..token_index + relation_len]
                            .iter()
                            .map(|f| f.str())
                            .collect();
                        // make a result
                        let res = TokenRelationResult::new(relation.clone(), str);
                        results.push(res);
                    }
                }
            }
        }
        results
    }
    pub fn print_detail(&self) {
        println!();
        println!("===== Printing Lexer Detail =====");
        println!("String:\n{}", self.string);
        println!("Identifiers: {:?}", self.identifiers());
        print!("Interpreted Tokens: {}",self.interpret_tokens_results().iter().map(|tkr|tkr.token().name().to_string()+&tkr.str()).collect::<Vec<String>>());
        println!("Interpreted Relations: ");
        self.interpret_relations_results().iter().for_each(|r|println!("{} {:?}",r.relation().name(),r.str()));
        // let mut vec: Vec<String> = Vec::new();
        // for rel in self.interpret_relations_results() {
        //     vec.push(rel.relation().name());
        // }
        // println!("Interpreted Relations: {:?}", vec);
    }
}
