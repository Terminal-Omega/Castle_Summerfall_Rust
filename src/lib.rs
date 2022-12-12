pub mod items;
pub mod entities;
pub mod rooms;


pub fn help_command() {
    let help_text = "Help 
                           exit -> Quit the program
                           describe -> Describes the current room the player is in
                           help -> print command list and what each command does\n";
    print!("{}", help_text);
}

