#[derive(Debug, Clone, PartialEq)]
pub struct UndefinedToken {
    str: String,
}
impl UndefinedToken {
    pub fn new(str: String) -> Self {
        Self { str }
    }
    pub fn str(&self) -> &str {
        &self.str
    }
}
impl From<String> for UndefinedToken {
    fn from(value: String) -> Self {
        todo!()
    }
}
