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

enum EvenOdd {
    Even(i32),
    Odd(i32),
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
                user_answer[1].parse::<i32>().expect("input an actual int32 dumbass"),
                user_answer[2].parse::<i32>().expect("input an actual int32 dumbass"),
            ),
            "look" => println!("That is the point {}, {}",
                position.x + user_answer[1].parse::<i32>().expect("input an actual int32 dumbass"),
                position.y + user_answer[2].parse::<i32>().expect("input an actual int32 dumbass"),
            ),
           _ => { println!("Fuck you"); return },
        }
        let even_odd = match user_answer[1].parse::<i32>().expect("an actual int32 dumbass") {
            i if  i%2       ==0 => EvenOdd::Even(i),
            i if (i%2).abs()==1 => EvenOdd::Odd(i),
            _ => {println!("Something awful happened!"); EvenOdd::Even(69)}
        };
        match even_odd {
            EvenOdd::Even(i) => println!("{} is even", i),
            EvenOdd::Odd(i)  => println!("{} is odd", i),
        };
        if user_answer[0] == "x" {return;}
    }
}

