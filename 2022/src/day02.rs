use usb_device::endpoint::Out;

use crate::{println, util::Day};

type Vec<T> = heapless::Vec<T, 4196>;

pub const DAY_02: Day<i32> = Day {
    year: 2022,
    day: 2,
    parts: &[part1, part2],
    tests: &[do_test_1, do_test_2],
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn score(&self) -> i32 {
        return match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        };
    }

    fn parse(value: &str) -> Self {
        return match value {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("Bad!"),
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn score(self) -> i32 {
        return match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        };
    }

    fn parse(value: &str) -> Self {
        return match value {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Bad"),
        };
    }
}

const OUTCOME_TABLE: &[(Hand, Hand, Outcome)] = &[
    (Hand::Rock, Hand::Rock, Outcome::Draw),
    (Hand::Rock, Hand::Paper, Outcome::Lose),
    (Hand::Rock, Hand::Scissors, Outcome::Win),
    (Hand::Paper, Hand::Rock, Outcome::Win),
    (Hand::Paper, Hand::Paper, Outcome::Draw),
    (Hand::Paper, Hand::Scissors, Outcome::Lose),
    (Hand::Scissors, Hand::Rock, Outcome::Lose),
    (Hand::Scissors, Hand::Paper, Outcome::Win),
    (Hand::Scissors, Hand::Scissors, Outcome::Draw),
];

#[derive(Debug)]
struct GamePart1 {
    me: Hand,
    opponent: Hand,
}

impl GamePart1 {
    fn score(&self) -> i32 {
        let outcome = OUTCOME_TABLE
            .iter()
            .find(|(a, b, _)| self.me == *a && self.opponent == *b)
            .expect("Did not find an outcome.")
            .2;

        return outcome.score() + self.me.score();
    }
}

fn part1() -> i32 {
    let games = parse_input_p1(INPUT);

    return games.iter().map(|g| g.score()).sum();
}

fn parse_input_p1(input: &str) -> Vec<GamePart1> {
    let mut stuff = Vec::<GamePart1>::new();

    for line in input.trim().lines() {
        let (a, b) = line.split_once(" ").expect("Bad!");
        let game = GamePart1 {
            me: Hand::parse(b),
            opponent: Hand::parse(a),
        };
        stuff.push(game).expect("Bad vec!");
    }

    return stuff;
}

fn part2() -> i32 {
    let games = parse_input_p2(INPUT);
    let mut score: i32 = 0;

    for game in games {
        score += game.score();
    }

    return score;
}

#[derive(Debug)]
struct GamePart2 {
    opponent: Hand,
    outcome: Outcome,
}

impl GamePart2 {
    fn score(&self) -> i32 {
        let my_hand = OUTCOME_TABLE
            .iter()
            .find(|(a, b, out)| self.opponent == *b && self.outcome == *out)
            .expect("Did not find a suitable hand.")
            .0;

        let g = GamePart1 {
            me: my_hand,
            opponent: self.opponent,
        };

        return g.score();
    }
}

fn parse_input_p2(input: &str) -> Vec<GamePart2> {
    let mut stuff = Vec::<GamePart2>::new();

    for line in input.trim().lines() {
        let (a, b) = line.split_once(" ").expect("Bad!");
        let game = GamePart2 {
            opponent: Hand::parse(a),
            outcome: Outcome::parse(b),
        };
        stuff.push(game).expect("Bad vec!");
    }

    return stuff;
}

fn do_test_1() {
    let input = r###"
A Y
B X
C Z
"###;
    let games = parse_input_p1(input);

    assert_eq!(3, games.len());
    assert_eq!(8, games[0].score());
    assert_eq!(1, games[1].score());
    assert_eq!(6, games[2].score());
}

fn do_test_2() {
    let input = r###"
A Y
B X
C Z
"###;
    let games = parse_input_p2(input);

    assert_eq!(3, games.len());
    assert_eq!(4, games[0].score());
    assert_eq!(1, games[1].score());
    assert_eq!(7, games[2].score());
}

const INPUT: &'static str = r###"
B Z
A Z
B Z
C Z
C Z
B X
A X
C X
A Z
C Y
C X
C Y
C Y
A X
A Z
A Z
A X
B Z
B X
A Z
A X
C Y
A X
B Z
B Z
A X
C Z
A Z
A X
B Z
A Z
A Y
C Y
A Z
C Z
A Z
C Y
C Z
C Z
A Z
A X
A X
B X
A Z
B Z
A X
A Z
A Z
A X
A X
C Y
A Z
B X
C Y
A X
B Y
A Z
A X
A Z
A X
C Z
A Z
A Y
A X
C Y
A X
B X
A X
A Z
C Y
A Z
A X
C X
C Z
C Z
A Z
A X
A Z
C X
C Z
B Z
A Z
C Y
C Z
B X
A X
A Z
A X
A X
C Y
A Z
C Z
B X
A X
A X
A Z
A Y
A X
C X
A Z
B X
C Y
A X
A X
A X
C X
B Z
B Z
A Z
A X
A Z
C Z
C X
C Y
B X
C Z
A Z
C X
A Z
A X
A X
A X
A X
A Z
B X
A X
B X
C Z
A Z
A Z
A X
A X
A X
B X
A Z
A X
A Z
A X
B Z
A Z
A X
B Z
A Z
A Z
C Y
B Z
A Z
B Z
A Z
A X
C X
A X
C Y
C X
A X
A X
C Y
A Z
A X
B Z
A Z
A Z
B X
B Z
A X
A Z
A X
A X
A Z
A X
A Z
A Z
C X
A Z
A X
C Y
A Z
A Z
A X
A X
A X
A X
B Z
B X
A Z
A X
A Z
C Y
B Y
C Y
B X
A Z
A Z
A Z
C Y
A Z
A X
B Z
C X
A X
C Z
C X
C Y
A Z
A X
A Z
C Z
A Z
A Z
A Y
C Z
A X
A X
B Z
A Z
C Z
A Z
A X
A Z
A Z
A Z
A Z
A Z
A Z
A Z
A Z
C X
A X
A X
A X
B Y
B X
A X
A Z
A Z
A X
A Z
A X
A X
A Z
B X
A Z
C Z
A Z
C Z
A Z
A Z
C Z
A X
C Z
A X
C X
A Z
A Z
B Z
A Y
B Z
A X
B Z
A Z
A X
A Z
A Z
A X
A Z
B Z
C X
A Z
A X
B Z
C Y
B Z
C X
A Z
A Z
C X
B X
C Z
A Z
A X
A Z
C X
A Z
C Z
C Z
B Z
B Z
A Z
C X
A X
B Y
A Y
A Z
C X
B X
A Z
A Z
B Y
A Z
A Z
C Z
A Z
A X
A Z
B X
C Z
B X
A Z
B Z
C Y
A X
A X
A Z
A Z
A Z
C Y
A X
A Z
A Z
A X
B Z
A Z
C X
C Z
A Z
A X
B Z
A X
C Y
A X
A Z
A Y
C Z
A Y
A Z
C X
C Y
A Z
C Y
A Z
A X
C Y
A Z
A X
A X
B Z
A X
C X
A X
C X
A X
A Z
A X
A Z
A Z
A X
C X
C Z
A Z
C Z
C X
A Z
C Y
A X
A Z
A Z
C Y
A X
B X
C Y
B Z
C Y
A X
A X
C Z
A X
A Z
A X
A X
C X
A X
A Z
B X
C Z
A Y
B Y
A Z
A Z
A Z
A Z
B Y
A Y
A Z
B Z
A Z
A X
C Z
B Y
C Y
A X
A Z
C Z
B Z
A Z
A Z
A Y
C Z
A Z
A Z
C Z
C Z
C Z
A X
B Y
C X
A X
A Z
A Z
B Z
A Y
A X
A Z
B X
A X
A X
A Z
A Y
A Z
A X
B X
A Z
C X
A Z
A X
C X
B X
B Y
B X
A Z
A Z
A Z
A X
B X
A X
B Z
A Y
B Z
C Z
A X
C Z
C X
A Z
C Y
C Y
A Z
A X
A X
A X
B X
A Z
A Y
C Y
B X
A X
A X
A X
C Y
A Z
A Y
A X
C Z
A Z
A Z
A Z
C Z
A X
A Z
C Z
B X
C Y
A Z
B Z
B Z
C Z
C Z
A X
A Z
A Z
B X
B X
A Z
A Z
A Y
C Z
A Z
A X
C Z
A Z
C Z
C Z
A Z
A Z
B Z
A X
B Y
A Z
A X
C Z
A X
B X
A Z
C Y
A Z
C X
C Y
A X
C X
C X
A Z
C Z
C Y
A Z
A Z
A X
A X
C Z
A Y
A Z
A X
B Z
A X
A Z
A Z
A X
B Z
A X
A Z
C X
A Z
A Z
C X
A Z
A X
A Z
A X
B Z
A Z
A Z
C Y
C Y
C Z
C Z
C Y
A X
A X
A X
A Z
A Z
C Y
A Z
C Z
A Z
C Z
B Z
B X
A Y
C Y
A X
A Z
A X
A X
A Z
C Z
C Y
C Y
A Y
A X
A Z
A X
C Z
A X
A Z
A Z
C Y
A X
A Z
A Z
A X
B Y
A Z
A Z
A Z
A Z
C Y
A Z
A X
A X
A Z
A Z
C Y
A Z
A Z
B Z
A Z
A X
A X
A Y
C Z
C Z
A Z
B Z
A Z
A Z
A X
A X
C Z
A Z
A X
A X
A Z
A Z
B Y
A Y
C Y
A X
A Z
C Z
C Y
A Z
C Y
A X
A X
A X
A Z
C Y
C Y
A Z
B X
C Y
A Z
A X
A Z
A Y
C Y
C Z
B Z
A Z
C Y
A X
C Z
A X
C Y
C Z
A Z
C Y
C Y
A Y
B X
A Z
C Z
B Z
B Y
A Z
C Y
A X
A Z
A Z
C Y
A Z
C Z
A Z
A Z
A X
A Z
A Z
C Z
A X
A Z
C Z
C Y
A X
A X
C Z
C Y
A Z
C Y
A Z
A X
A X
A Z
A Z
A X
A Z
C Y
C Z
A Z
B X
C Z
A Z
A Z
A X
B X
A Z
A Y
A X
C Z
B X
A Z
C Y
C Z
C Z
C Z
A Z
A Z
A X
A Z
C Z
C Y
A X
A Z
A Z
A Y
A X
A Z
A Z
C X
B Z
A X
A Z
A X
C Z
A X
C X
A X
C Y
C Y
A X
A X
A Z
B Z
A Z
A Y
B Z
A X
C X
A X
C X
A X
C X
A Z
A X
A X
A X
C Y
A X
A X
C X
C Z
A X
B Y
A X
B Z
A Z
A X
C Z
A Z
A X
A Z
A X
C X
C Z
A X
C Z
C Y
C Y
A X
A X
A X
C Z
C Z
A Z
C Z
A Z
B Z
A Y
B Z
A X
A Z
A Z
A Z
C Z
A Z
A X
A X
B Z
A Z
C Y
C Z
B Z
C Y
C Z
A X
A X
C X
B X
C Z
A Z
A Z
C Z
A X
A Z
A Z
B X
B Z
A Z
A Z
B Z
A Z
A Z
B Z
B Z
C Z
A Z
C Z
A Z
A Z
C Z
A Z
A Z
B Z
A Z
A X
A X
A X
A Z
C Z
A X
A Z
A X
A Z
B Z
B Z
C Z
A Z
A Y
C X
A X
A Z
A X
A X
C X
C Z
B Y
A X
A X
C Y
C Y
A X
A Z
A Z
B Z
B Z
A Z
C Z
A Z
C Z
C X
A Z
A X
A Y
A X
A Z
C Z
A X
A Z
A Z
A Z
A X
C X
B Z
A Z
A Y
C X
A Z
A Z
A Z
A X
A Y
A X
C Y
A Z
C Y
A Z
A Z
C X
A Z
A Y
C Z
A X
A X
A Z
A Y
A Z
A X
C Z
A Z
A Z
B Z
A Z
A Z
C X
B Z
C Y
A X
A Y
B X
A Z
C Z
C X
C Z
A X
A X
A Z
A Z
A X
A Z
B Z
B Z
A Z
B Y
B Z
A X
A X
C Z
A Z
C Y
B Z
C Z
C Z
A Z
A X
B Z
A X
A X
A Z
A Z
A X
B Z
A Z
C Z
A Z
A Z
B X
C Z
B Z
C Y
A Z
A X
A X
B X
A X
B Z
C Y
C Y
B Z
C Y
C Y
A X
A Z
A Z
C Y
B Z
A X
A Z
A X
B X
A X
C Y
A X
A X
B Z
A X
B X
A Z
C Y
B Z
A X
A Z
A X
A X
B X
A X
A X
A X
A Z
A Z
C Y
A Z
B Z
C Z
C Y
C Z
A X
A Z
A X
C Z
C Z
C Z
C Y
A X
B X
B X
C X
C Z
C X
C Z
B Z
A Y
A X
C X
B X
A X
A X
A Z
A X
A Z
A X
A X
A Z
A Z
C Z
A X
C Y
C Y
C Z
A Z
A X
A Z
A X
A Z
A Y
A Z
A X
A X
A Z
A X
A Y
A Y
C X
A Y
C Y
C X
A Z
C Z
C Y
C Z
A Z
C X
A X
C Z
A Z
C Y
B Z
A Z
A Z
A Z
C Y
A X
A X
A Z
A Z
A Z
B Z
C Z
A X
C Y
A X
A Z
A X
C X
C Z
A Z
C Y
A Y
A Y
A Z
A X
A Z
A Y
A X
C Z
A Z
A Z
A X
A X
B Z
B Y
B Y
C X
A Z
A Z
C Z
A X
A Z
C Y
B X
C Y
A Z
A Z
A X
A Z
A X
A X
C Z
A Z
A Z
A X
A X
C Y
A X
A X
C Y
A Y
C X
A X
C Y
A Y
A Z
B Y
A X
C Z
B X
A X
C Z
A Z
B Z
A Z
A Z
C X
A Z
B Z
C X
A Z
C Y
A Z
C Z
A Z
C X
A X
A Z
A Y
B X
A X
B X
A Z
A Z
C Y
A Z
C Z
A Z
A Z
A Z
C X
A Z
B X
A X
A Z
C Z
A Z
A X
C Y
A Z
C Y
A Z
A Z
C Z
A X
A Z
A Z
B X
A Z
A X
C Z
B Y
A Z
A X
C Z
A X
A Z
A Z
A Z
C Z
A Z
A Z
C X
A X
A X
C X
A X
A X
B Z
A Z
A X
B X
A X
A Z
C Y
A Z
B X
C Y
A X
C Z
A Z
B Z
A X
A Z
A Z
C Z
A X
C Z
A X
C X
A Y
A X
A Z
C X
A X
A X
A X
A X
A Z
A X
B Z
C Z
C Y
B Z
B Z
B X
A Z
A X
B Z
A Z
A X
C Z
A Z
C Y
B Z
C Y
A X
C X
A X
A Z
C Y
A Z
A X
A Y
C Y
C Y
A Z
A Z
C Y
A Z
A Z
A X
C Y
B Y
A X
A X
A X
C Y
B X
A Z
A Y
A Z
A Z
A X
C Z
A Z
A Z
C X
A Z
B Z
B X
A X
C Z
A X
C Y
A Z
C Z
C Y
C X
C Y
A Z
A X
A Z
A Z
A Z
B Z
A Z
C Z
A Z
C Z
B Z
C Z
C Z
A Z
A Z
B X
A Z
A X
A Z
A X
A Y
A Z
B Z
C X
A Z
A Z
B X
A X
C X
C Z
A X
A Z
A X
C Y
C Z
A Z
C Y
A X
A Z
A Z
B Z
C Y
A Z
A Z
C Z
A X
C X
A Z
A Z
A Y
C Z
C Z
A Z
A X
B Z
A X
A X
A X
A X
A Z
A Z
A X
A X
C Z
C Y
C Y
C Y
A Z
C X
B Z
C Y
C Z
A X
A X
A Z
C Z
C Z
A Y
C Y
B X
C Y
A X
A X
A Y
A X
A Z
A Z
A X
A Z
A Z
A Z
A X
A Z
B Z
A X
A X
A X
C Z
C X
A Z
C Y
C Z
A X
A Z
A Z
A X
A Z
A X
B Z
A Z
A Z
A Z
C Y
C Z
B Z
A Z
B Z
A X
A X
A X
C Y
A Z
A Z
A Z
C X
A X
A X
A Z
A Z
A Z
A Z
B Z
A X
A Z
A X
C Z
A Z
A Z
A Z
B Z
A Y
A Z
C Z
B Z
C Y
A Z
A X
C Y
A X
A X
A Z
A Z
A X
B Z
B X
C Z
C Z
A Y
A Z
A X
A Z
A Z
A Z
A Z
A Z
B Z
A Z
B X
A Z
A Z
C Z
B Z
A Z
C X
B Z
C X
B Z
A Z
A Z
A Z
A Z
A X
B Z
A X
B Z
C Y
A Z
A Z
C X
A X
A Z
A Y
A Z
C Y
C X
C Z
A X
A Z
C Z
A Z
A X
A Z
B Z
A X
A Z
B Z
C Y
A Z
C Y
A Z
C Y
B X
C Y
A Z
B Z
A X
B Z
B Z
C X
A Z
C Z
A Z
C Z
A Z
C Z
B Z
C Y
C Z
A Z
C Y
A Z
C X
B Y
B Y
C X
C Y
A Z
C Z
A Z
C X
A Z
A Z
C Y
A Z
A Z
C X
A Y
A X
A X
B Z
A Z
C Z
A Z
B Z
C Y
C Z
A Z
C Y
B X
C Z
A X
C X
C Y
C Y
C Y
A Z
A Z
B Y
C Y
A Z
A X
A Z
A X
A Z
C X
C Z
A X
C Z
B X
A X
C Z
A Z
C X
A Y
A Z
A X
A Z
C Z
B Z
B Z
A Z
C Y
A Z
C Z
C Y
C Z
A Z
A Z
B X
C X
C Z
A X
A Z
A Y
C Y
A X
A Z
C Y
A Z
B X
A Z
A Z
A Z
A Z
C X
A X
A Z
A Y
A X
B Y
A X
A X
A X
A X
B Z
A X
C Y
C X
A X
A Z
A X
A Z
A Z
A Z
A Z
A X
A Z
C X
C Z
C Z
A X
B Y
A X
C Y
A X
C Z
A X
A Z
A Z
A Z
A Z
C X
A X
C Y
A Z
A Z
C Y
A Z
A X
A Z
A X
A Z
B Y
C Y
B Y
C Y
A Y
A Z
C X
B X
A X
A Y
C Z
A X
B Z
A X
A X
A Y
A X
A Z
A Z
C Z
C X
A Z
C Y
A Z
A Z
C X
A Y
A Z
A X
A Z
A Z
C X
A Z
A Z
A Z
C Y
A X
A X
A X
C Z
A Z
C Y
A X
A X
C Y
C X
C Y
A Y
C X
A Z
A Z
A Y
C X
A Z
A Z
A Z
C Y
C X
A Z
B Z
A Z
A X
C Y
A X
C Y
C Y
A X
C Y
A X
C Y
B Z
A Z
A X
A Z
A X
A X
A Z
A X
A X
C Z
A Z
B Y
C X
B X
B X
A Z
A Y
B Y
A Z
A X
A X
C Y
C X
B Z
A Z
C Y
C X
C Y
A Y
A Z
C X
A X
A X
A Z
A Z
B Z
C X
A Z
B Z
A X
B Y
C X
A X
A Y
C X
A X
C Y
A Z
A Z
A X
A X
C Z
C Z
B Z
A X
B Z
B Y
A Z
A Z
C Y
A Z
B Y
A Z
A X
C Z
C Z
A X
A X
A X
A Z
A Z
C Y
A X
A X
A Z
A X
C X
A Z
A X
B Z
B X
A X
A X
A Z
B Z
A X
A Z
C Y
C Y
B Z
C Z
A X
B X
B X
A Z
A X
A Z
C Y
A X
A X
B Y
C Y
A X
A X
C Y
A Z
A Z
C Z
A Z
B Z
A Y
A Z
C Y
B Y
A X
C X
A X
C Z
A Z
A X
A Z
A Z
A X
B Z
C Z
A Z
A X
C X
A X
C Y
B Z
B X
C Z
C X
A X
A X
A Z
A X
C Y
B Z
A Z
C Z
A Z
A Z
A X
B X
A X
A Z
A X
A X
A Z
A Z
A Y
A Z
C Z
B Z
A X
A X
A X
A Z
B Z
A Z
A Z
C Z
C Y
C Y
A Z
A X
A Z
C Z
A X
A X
A Z
A Z
C X
B Y
A X
B Z
B Z
A Z
A Z
C Z
C Z
A X
A X
A Z
C Z
A Z
C Z
B X
A X
A Z
A Z
C Z
A X
A Z
B Y
B X
A X
C Y
C X
C Y
B X
A X
C Y
A Z
B Y
A Y
A X
C Y
A Z
A Z
A Z
A Z
A Z
C Z
A Z
A Z
A Z
C X
B X
C Z
A X
B X
A X
C Y
A X
C X
A X
A Z
A Z
B X
A X
A Z
A X
C Z
A Z
A Z
A X
C Z
A Z
C X
A Z
A Z
A Z
A Z
B Z
A X
A X
C Y
A Z
A Z
B Z
A X
A Z
A Y
A Z
C X
B Z
A Z
A X
B Y
A Z
A X
C X
C X
A X
A Z
A X
A Z
B Z
B Y
A Y
A Z
C Y
B Z
A Z
C X
C Y
C Z
C X
A X
A Z
A X
A X
A Z
A X
A X
A X
A Z
C Z
C Y
A Z
A X
C X
B Z
C Z
C Y
C Y
A Z
A X
A Z
A Z
A Z
A X
A Z
A Z
A X
C Z
A X
A X
A Z
C X
B Y
A X
C Z
A X
C X
A Z
A X
A Y
C Z
A X
A Z
C X
B X
A Z
A Z
A X
A X
A Z
C Y
C Z
A X
A Z
C Z
C X
A X
A Z
A X
A X
B X
C Z
B Z
A Z
A X
A X
B Y
A Z
A X
A X
A Z
A Z
A Z
A X
C Y
A Z
C Y
A Z
A Z
A X
C Z
A Z
B X
A X
C Z
A X
A Y
B X
C X
A X
A X
A Z
A X
A X
A X
C Y
A Y
A Z
B Z
B Z
B Z
A Z
A Z
A Z
A X
B X
C Y
A X
A Z
C Z
B Y
A X
A Z
A Z
A X
A X
B Z
A X
C Y
A Z
A X
A X
A Z
A Z
A Z
A Z
A Z
A X
A Z
A X
A X
C Z
A X
C Z
A Z
C Y
A X
A Z
C Z
A Z
A Z
A X
A X
A Z
A X
A Z
B Z
A X
A X
C X
A Z
A X
A Z
C Y
C Y
C Y
C Y
C Z
A Z
B X
C Z
A X
A Z
C Y
A X
A Z
B X
A Z
C X
C Z
C X
C X
A Z
A Z
B X
A Z
A Z
A Z
B Z
A Z
C Z
A X
A X
A Z
A X
A Z
C Z
C Z
A Z
C Z
C X
A Z
A X
B Z
A X
B Y
C X
A X
A Y
A Z
A Z
A X
A X
A Z
A X
A Z
A X
A Z
A X
A Y
A X
C Z
A Z
A Z
A X
A X
A Z
A X
C Y
C Z
A Z
A Y
A Z
A X
A Z
C X
B X
A Z
C Z
B Y
A Z
A Z
C Y
A X
A Z
A Z
C Z
C Y
A Z
B Y
A X
C Y
A X
A X
A Z
C Z
A Z
A Z
A X
C Z
A X
C Y
C Y
A X
B X
C X
A Y
A X
A Z
B X
A X
B Z
A Z
A X
C X
A X
B X
A X
A Z
A X
C Y
A Z
C Z
C Y
A X
B X
C Y
C X
A X
A Z
A X
A Z
A X
A X
A Z
A Z
A X
A Z
A Y
A X
A X
A Y
A Z
A Z
C Z
A X
A Z
A X
A X
B X
A X
B Y
C Y
A Z
B Z
A Z
B Y
A Z
C Y
A Z
A X
A X
A Z
C X
C Z
B X
A X
A Z
A X
A Z
A Z
B X
A Z
A X
C Z
B Y
C Y
C X
A X
A Z
A X
C Y
A Z
C Z
A X
A X
A X
A Z
A Z
A Z
B X
C X
A Y
B Z
A X
A Z
A Z
C X
C Y
A X
A Z
C Y
A X
A Z
A X
A Z
A X
A X
B Z
A X
C X
A X
A X
A X
A Z
A Z
A X
A Z
C Y
C Z
B Z
B X
A Z
A X
C X
C Y
A X
B Z
B Z
A Z
B Z
C Y
A X
A X
A X
C Y
A Z
A X
A Z
A Z
A Z
A X
A Z
A X
A Z
C Y
A X
C X
C Y
C Y
C X
A X
C Y
A Z
C Y
B Z
B Z
C Z
A X
B X
C X
A Z
A Y
A Z
B Z
A Z
B Z
A X
A X
A Z
A Z
A Y
A Z
A X
A Y
A Z
A Z
A Z
A Z
B X
A Z
A Z
A Z
A Z
C Y
B Y
C X
A Z
B Z
B X
A X
C Z
A Z
C Z
A Z
A X
A X
A X
A X
A X
A Z
C Y
A X
C X
A Y
C X
C X
A X
A Z
A X
C Z
A X
C Y
B Z
C Y
C Z
B X
A Z
C Y
A Z
C Y
A X
A X
A X
A Z
A Z
A Z
A Z
C Z
C Y
A X
C Y
B Y
A X
A X
A Z
C Y
C Z
A X
A X
A Z
A Z
A X
A X
A Z
"###;
