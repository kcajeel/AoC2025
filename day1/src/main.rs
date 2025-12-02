use std::fs::{self};

const DIAL_MAX: i32 = 100;
const DIAL_MIN: i32 = 0;
const START_POS: i32 = 50;

fn main() {
    let mut dial = Dial {
        position: START_POS,
        password: 0,
    };
    let input_result = fs::read_to_string("input.txt");

    if let Ok(input) = input_result {
        for line in input.lines() {
            // do thing
            let operation = line.chars().nth(0).unwrap();
            let distance = line[1..].parse::<i32>().unwrap();

            match operation {
                'L' => dial.rotate_left(distance),
                'R' => dial.rotate_right(distance),
                _ => println!("Not L or R: {operation}"),
            };

            if dial.position == DIAL_MIN {
                dial.password += 1;
            }
        }
        println!("Password is: {}", dial.password);
    } else {
        let _ = input_result.inspect_err(|e| {
            println!("File reading failed: {e}");
        });
    }
}

struct Dial {
    position: i32,
    password: u32,
}
impl Dial {
    /*
     * dial from 0-99
     * L -> subtract number
     * R -> increase number
     * starts at 50
     * store sum of 0's
     */
    pub fn rotate_left(&mut self, distance: i32) {
        // rotate left
        self.position = (self.position - distance) % DIAL_MAX;
    }

    pub fn rotate_right(&mut self, distance: i32) {
        // rotate right
        self.position = (self.position + distance) % DIAL_MAX;
    }
}
