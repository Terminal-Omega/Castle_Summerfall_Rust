use crate::items::Item;
use crate::entities::Entity;
pub struct Room{
    doors: Vec<Door>,
    items: Vec<Box<dyn Item>>,
    entities: Vec<Box<dyn Entity>>
}

enum Door{
    Locked(i32),
    Barred(i32),
    Open(i32)
}