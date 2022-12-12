use crate::items::{Item, Container};
use crate::entities::Entity;
pub struct Room{
    doors: Vec<Door>,
    items: Vec<Box<dyn Item>>,
    entities: Vec<Box<dyn Entity>>
}

impl Room {
    pub fn new() -> Room{
        let doors = vec!();
        let items: Vec<Box<dyn Item>> =vec!(Box::new(Container::new("This is a test", "I'm a test!")));
        let entities =vec!();
        let room = Self { doors, items, entities };
        room
    }

    pub fn from(items: Vec<Box<dyn Item>>) -> Self {
        Self {
            doors: vec!(),
            items,
            entities: vec!(),
        }
    }

    pub fn describe(&self) -> String {
        let mut description = String::new();
        for item in &self.items {
            description += item.get_name();
        }

        description
    }
}

enum Door{
    Locked(i32),
    Barred(i32),
    Open(i32)
}