use std::io::{self, stdin};
use std::collections::HashMap;

#[derive(Clone)]
pub enum Command {
    Nav(Nav),
    Quit,
    Invalid(String),
}

#[derive(Clone)]
pub enum Nav {
    Shift,
    EnterExit,
    Look,
}

pub struct CommandInterface {
    hash_cmd_s: HashMap<String, Command>
}

impl CommandInterface {
    pub fn prompt(&self, prompt: &str) -> io::Result<(
        Command, Vec<String>,
    )> {
        println!("{}", prompt);
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let mut spl = input.split_whitespace();
        Ok((
            self.parse_command(spl.next().unwrap()),
            spl.map(|s| s.to_string()).collect(),
        ))
    }

    fn parse_command(&self, input_s: &str) -> Command {
        match self.hash_cmd_s.get(input_s) {
            Some(command) => command.clone(),
            None => Command::Invalid(input_s.to_string()),
        }
    }

    pub fn update_commands(&mut self, line: String) {
        let mut spl_line = line.split(":");
        let command = self.match_to_command(spl_line.next().unwrap()).unwrap();
        let aliases = spl_line.next().expect("string").split(", ");
        for alias in aliases {
            self.hash_cmd_s.insert(alias.to_string(), command.clone());
        }
    }

    fn match_to_command(&self, s: &str) ->
    Result<Command, String> {
        let cmd = match s {
            "Quit" => Command::Quit,
            "Move" => Command::Nav(Nav::Shift),
            "EnterExit" => Command::Nav(Nav::EnterExit),
            "Look" => Command::Nav(Nav::Look),
            _ => panic!("Tried to map aliases to invalid command name: {s}"),
        };
        Ok(cmd)
    }
}


pub fn initialize_commands() -> CommandInterface {
    let mut s_commands: HashMap<String, Command> = HashMap::new();
    ["x", "exit", "quit"].iter().for_each(
        |s| {s_commands.insert(s.to_string(), Command::Quit);}
    );
    ["go"].iter().for_each(
        |s| {s_commands.insert(s.to_string(), Command::Nav(Nav::Shift));}
    );
    ["in", "out"].iter().for_each(
        |s| {s_commands.insert(s.to_string(), Command::Nav(Nav::EnterExit));}
    );
    ["l", "look"].iter().for_each(
        |s| {s_commands.insert(s.to_string(), Command::Nav(Nav::Look));}
    );

    CommandInterface { hash_cmd_s: s_commands }
}
