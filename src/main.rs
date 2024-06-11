mod navigation;
mod player_io;
mod file_io;
use player_io::{Command, Nav};

fn main() {
    let position = navigation::Position::new(0, 0, false);
    let _files = file_io::initialize();
    let cmd_ifc = player_io::initialize_commands();
    
    loop {
        let (command, _args) = cmd_ifc.prompt(
            &format!("you are at {}", position)
        ).unwrap();
        match command {
            Command::Quit => return,
            Command::Nav(cmd) => navigation::move_command(cmd),
            Command::Invalid(cmd) => println!("Invalid command entered: {}", cmd),
        }
    }
}

