use std::fmt::{self, Formatter, Display};
use crate::Nav;

enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
    In,
    Out,
}

pub struct Position {
    x: i32,
    y: i32,
    interior: bool,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
            "x: {0}, y: {1}, inside: {2}",
            self.x, self.y, self.interior
        )
    }
}

impl Position {
    fn shift(&mut self, m_x: i32, m_y: i32) {
        self.x += m_x;
        self.y += m_y;
    }

    pub fn new(x: i32, y: i32, interior: bool) -> Position {
        Position { x, y, interior }
    }
}

pub fn move_command(command: Nav) {
    match command {
        Nav::Shift => println!("Shift command received"),
        Nav::EnterExit => println!("EnterExit command received"),
        Nav::Look => println!("Look command received"),
    }
}
