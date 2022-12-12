mod items;
mod entities;
mod rooms;
use rooms::Room;

pub fn generate_room() -> Room {
    Room::new()
}

pub fn help_command() {
    let help_text = "Help 
                           exit -> Quit the program
                           describe -> Describes the current room the player is in
                           help -> print command list and what each command does\n";
    print!("{}", help_text);
}