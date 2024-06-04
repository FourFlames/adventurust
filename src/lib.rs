use std::io::{self, stdin};

pub fn prompt_user(prompt: &str) -> Result<Vec<String>, io::Error> {
    println!("{}", prompt);
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.split_whitespace().map(|s| s.to_string()).collect())
}
