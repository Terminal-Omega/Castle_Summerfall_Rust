pub trait Item {
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
}

pub struct Weapon {
    name: String,
    description: String
}

impl Item for Weapon {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_description(&self) -> &str {
        &self.description
    }
}

pub struct Container {
    name: String,
    description: String
}

impl Item for Container {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_description(&self) -> &str {
        &self.description
    }
}

impl Container {
    pub fn new<StringOrRef: Into<String>>(name: StringOrRef, description: StringOrRef) -> Self {
        Container {
            name: name.into(),
            description: description.into()
        }
    }
}
