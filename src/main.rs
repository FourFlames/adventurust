mod navigation;
mod player_io;
mod file_io;
use player_io::{Command, Nav};

fn main() {
    let position = navigation::Position::new(0, 0, false);
    let files = file_io::initialize().unwrap();
    let mut commands = player_io::initialize_commands();
    for line in files.read_lines_of("commands").unwrap() {
        commands.update_commands(line);
    };
    
    loop {
        let (command, _args) = commands.prompt(
            &format!("you are at {}", position)
        ).unwrap();
        match command {
            Command::Quit => return,
            Command::Nav(cmd) => navigation::move_command(cmd),
            Command::Invalid(cmd) => println!("Invalid command entered: {}", cmd),
        }
    }
}

