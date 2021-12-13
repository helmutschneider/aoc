#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Moving(i64),
    Resting(i64),
}

#[derive(Debug, Clone, Copy)]
struct Reindeer<'a> {
    name: &'a str,
    speed: i64,
    can_move_seconds: i64,
    must_rest_seconds: i64,
    state: State,
    distance_traveled: i64,
    score: i64,
}

pub fn run() {
    let input = std::fs::read_to_string("src/day14_input.txt").unwrap();
    let deer = parse_deer(&input);
    part1(&deer);
}

fn parse_deer(input: &str) -> Vec<Reindeer> {
    let mut deer = Vec::new();

    for line in input.trim().lines() {
        let chunks: Vec<&str> = line.trim().split_whitespace().collect();
        let d = Reindeer {
            name: chunks[0],
            speed: chunks[3].parse().unwrap(),
            can_move_seconds: chunks[6].parse().unwrap(),
            must_rest_seconds: chunks[chunks.len() - 2].parse().unwrap(),
            state: State::Moving(0),
            distance_traveled: 0,
            score: 0,
        };
        deer.push(d);
    }

    return deer;
}

fn part1(deer: &[Reindeer]) {
    let mut deer: Vec<Reindeer> = deer.to_vec();

    for _ in 1..=2503 {
        for d in deer.iter_mut() {
            match d.state {
                State::Moving(s) => {
                    if s == d.can_move_seconds {
                        d.state = State::Resting(1);
                    } else {
                        d.state = State::Moving(s + 1);
                        d.distance_traveled += d.speed;
                    }
                }
                State::Resting(s) => {
                    if s == d.must_rest_seconds {
                        d.state = State::Moving(1);
                        d.distance_traveled += d.speed;
                    } else {
                        d.state = State::Resting(s + 1);
                    }
                }
            }
        }

        let winner_distance = deer.iter().map(|d| d.distance_traveled).max().unwrap();
        deer.iter_mut()
            .filter(|d| d.distance_traveled == winner_distance)
            .for_each(|d| d.score += 1);
    }

    let winner_by_distance = deer
        .iter()
        .max_by(|a, b| a.distance_traveled.cmp(&b.distance_traveled))
        .unwrap();

    let winner_by_score = deer.iter().max_by(|a, b| a.score.cmp(&b.score)).unwrap();

    println!(
        "Day 14A: {} = {}",
        winner_by_distance.name, winner_by_distance.distance_traveled
    );
    println!(
        "Day 14B: {} = {}",
        winner_by_score.name, winner_by_score.score
    );
}
