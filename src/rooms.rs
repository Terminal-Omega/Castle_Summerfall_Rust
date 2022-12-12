use std::ops::Add;

use crate::items::Item;
use crate::items::Object;
use crate::entities::Entity;
pub struct Room{
    doors: Vec<Door>,
    items: Vec<Box<dyn Item>>,
    entities: Vec<Box<dyn Entity>>
}

impl Room {
    pub fn new() -> Room{
        let mut doors = vec!();
        let mut items: Vec<Box<dyn Item>> =vec!(Box::new(Object::new("This is a test", "I'm a test!")));
        let mut entities =vec!();
        let mut room = Self { doors, items, entities };
        room
    }

    pub fn describe(&self) -> String {
        let mut description = String::new();
        for item in &self.items {
            description += &*item.get_name();
        }

        description
    }
}

enum Door{
    Locked(i32),
    Barred(i32),
    Open(i32)
}