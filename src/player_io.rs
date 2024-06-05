use std::io::{self, stdin};
use std::collections::HashMap;

#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
pub enum Command {
    Nav(Nav),
    Quit,
    Invalid(String),
}

#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
pub enum Nav {
    Shift,
    EnterExit,
    Look,
}

pub struct CommandInterface {
    hash_cmd_s: HashMap<Command, Vec<String>>
}

impl CommandInterface {
    pub fn prompt(&self, prompt: &str) -> Result<(
        Command, Vec<String>,
    ), io::Error> {
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
        match self.hash_cmd_s.iter().find(
                    |&(_k, vec)| vec.iter().any( |s| s == input_s )
                )
            {
                Some(cmd) => cmd.0.clone(),
                None => Command::Invalid(input_s.to_string()),
            }
    }
}


pub fn initialize_commands() -> CommandInterface {
    let mut s_commands: HashMap<Command, Vec<String>> = HashMap::new();
    s_commands.insert(
        Command::Quit,
        vec!["x", "exit", "quit"].iter().map(
            |str| str.to_string()
        ).collect()
    );
    s_commands.insert(
        Command::Nav(Nav::Shift),
        vec!["go"].iter().map(
            |str| str.to_string()
        ).collect()
    );
    s_commands.insert(
        Command::Nav(Nav::EnterExit),
        vec!["in", "out"].iter().map(
            |str| str.to_string()
        ).collect()
    );
    s_commands.insert(
        Command::Nav(Nav::Look),
        vec!["l", "look"].iter().map(
            |str| str.to_string()
        ).collect()
    );
    CommandInterface { hash_cmd_s: s_commands }
}
