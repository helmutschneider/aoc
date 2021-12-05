use std::fs;

#[derive(Debug)]
struct Position {
    aim: i64,
    depth: i64,
    horiz: i64,
}

#[derive(Debug, Clone, Copy)]
enum Part {
    One,
    Two,
}

impl Position {
    const ZERO: Self = Self {
        aim: 0,
        depth: 0,
        horiz: 0,
    };

    fn up(&self, value: i64, part: Part) -> Self {
        return match part {
            Part::One => Self {
                aim: self.aim,
                depth: self.depth - value,
                horiz: self.horiz,
            },
            Part::Two => Self {
                aim: self.aim - value,
                depth: self.depth,
                horiz: self.horiz,
            },
        };
    }

    fn down(&self, value: i64, part: Part) -> Self {
        return match part {
            Part::One => Self {
                aim: self.aim,
                depth: self.depth + value,
                horiz: self.horiz,
            },
            Part::Two => Self {
                aim: self.aim + value,
                depth: self.depth,
                horiz: self.horiz,
            },
        };
    }

    fn forward(&self, value: i64, part: Part) -> Self {
        let maybe_add_to_depth = match part {
            Part::One => 0,
            Part::Two => self.aim * value,
        };

        return Self {
            aim: self.aim,
            depth: self.depth + maybe_add_to_depth,
            horiz: self.horiz + value,
        };
    }
}

pub fn run() {
    let input = fs::read_to_string("./src/day2_input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let pos = do_the_parsing(&input, Part::One);
    println!("{:?}", pos);
}

fn part2(input: &str) {
    let pos = do_the_parsing(&input, Part::Two);
    println!("{:?}", pos);
}

fn do_the_parsing(input: &str, part: Part) -> Position {
    let pos = input.lines().fold(Position::ZERO, |carry, line| {
        let (direction, amount_str) = line.split_once(" ").unwrap();
        let amount = amount_str.parse::<i64>().unwrap();
        return match direction {
            "up" => carry.up(amount, part),
            "down" => carry.down(amount, part),
            "forward" => carry.forward(amount, part),
            _ => carry,
        };
    });
    return pos;
}
