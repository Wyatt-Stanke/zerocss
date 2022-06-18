pub struct BaseDeclaration {
    pub name: String,
    pub value: String
}

impl BaseDeclaration {
    pub fn new(name: &str, value: &str) -> BaseDeclaration {
        BaseDeclaration {
            name: name.to_string(),
            value: value.to_string()
        }
    }
}

pub fn _css(name: &str, value: &str) -> BaseDeclaration {
    BaseDeclaration::new(name, value)
}