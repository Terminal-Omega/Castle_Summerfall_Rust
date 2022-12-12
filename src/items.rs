pub trait Item {
    fn get_name(&self) -> &str;
}

pub struct Weapon {

}

pub struct Container {
    name: String,
    description: String
}

impl Item for Container {
    fn get_name(&self) -> &str {
        &&self.name.as_str()
    }
}

pub struct Object {
    name: String,
    description: String
}

impl Object {
    pub fn new(name: &str, description: &str) -> Object {
        Object {
            name: name.to_string(),
            description: description.to_string()
        }
    }
}

impl Item for Object {
    fn get_name(&self) -> &str {
        &&self.name.as_str()
    }
}