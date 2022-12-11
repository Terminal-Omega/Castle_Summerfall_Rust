use crate::items::Item;
use crate::entities::Entity;
struct Room{
    doors: Vec<Door>,
    items: Vec<Item>,
    entities: Vec<Entity>
}

enum Door{
    Locked(i32),
    Barred(i32),
    Open(i32)
}