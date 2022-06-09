struct BaseDeclaration {
    pub name: String,
    pub value: String
}

impl BaseDeclaration {
    pub fn new(name: String, value: String) -> BaseDeclaration {
        BaseDeclaration {
            name: name,
            value: value
        }
    }
}