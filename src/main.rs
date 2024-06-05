mod navigation;
mod player_io;
use player_io::{Command, Nav};

fn main() {
    let mut position = navigation::Position::new(0, 0, false);
    let cmd_ifc = player_io::initialize_commands();
    
    loop {
        let (command, args) = cmd_ifc.prompt(
            &format!("you are at {}", position)
        ).unwrap();
        match command {
            Command::Quit => return,
            Command::Nav(cmd) => navigation::move_command(cmd),
            Command::Invalid(cmd) => println!("Invalid command entered: {}", cmd),
        }
    }
}

