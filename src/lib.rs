mod items;
mod entities;
mod rooms;
use rooms::Room;

pub fn generateRoom() -> Room {
    Room::new()
}