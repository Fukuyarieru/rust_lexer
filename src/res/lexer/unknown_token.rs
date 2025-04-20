use std::sync::Arc;

use crate::{
    arc,
    res::token::{TokenSettings, TokenTrait},
};

pub struct UnknownToken {
    str: String,
}
impl TokenTrait for UnknownToken {
    fn identifiers() -> Arc<[&'static str]> {
        arc!()
    }
    fn name() -> &'static str {
        "Unknown Token"
    }
    fn settings() -> TokenSettings {
        TokenSettings::new()
            .case_sensetive_set(false)
            .prefixes_set(arc!())
            .suffixes_set(arc!())
    }
}
impl UnknownToken {
    pub fn get_str(&self) -> String {
        self.str.clone()
    }
    // i wish i could make this somehow inherit what it takes (after interpreted) so it would have orginized like prefixes and suffixes
}
