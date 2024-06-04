use adventurust::prompt_user;

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn shift(&mut self, m_x: i32, m_y: i32) {
        self.x += m_x;
        self.y += m_y;
    }
}

fn main() {
    let mut position = Position { x: 0, y: 0 };
    
    loop {
        let user_answer = prompt_user(
            &format!("you are at {}, {}", position.x, position.y)
        ).unwrap();
        match user_answer[0].as_str() {
            "x" => return,
            "go" => position.shift(
                user_answer[1].parse::<i32>().expect("input an actual int32 my good sir/madame/other"),
                user_answer[2].parse::<i32>().expect("input an actual int32 my good sir/madame/other"),
            ),
            "look" => println!("That is the point {}, {}",
                position.x + user_answer[1].parse::<i32>().expect("input an actual int32 mygoodsir/madame/other"),
                position.y + user_answer[2].parse::<i32>().expect("input an actual int32 mygoodsir/madame/other"),
            ),
           _ => { println!("Fuck you"); return },
        }
        if user_answer[0] == "x" {return;}
    }
}

