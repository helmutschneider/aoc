use std::collections::HashMap;

type Seatings = HashMap<String, Vec<Seating>>;

#[derive(Debug)]
struct Seating {
    name_a: String,
    name_b: String,
    value: i64,
}

pub fn run() {
    let input = std::fs::read_to_string("src/day13_input.txt").unwrap();
    let lines: Vec<&str> = input.trim().lines().collect();
    let s = parse_seatings(&lines);
    part1(&s);

    let mut s = parse_seatings(&lines);
    s.insert("Helmut".to_string(), Vec::new());
    part2(&s);
}

fn parse_seatings(lines: &[&str]) -> Seatings {
    let mut map = Seatings::new();
    for line in lines {
        let parts: Vec<&str> = line.trim_matches(&['.'][..]).split_whitespace().collect();
        let name_a = parts[0];
        let name_b = parts[10];
        let value = if parts[2] == "gain" {
            parts[3].parse().unwrap()
        } else {
            -parts[3].parse::<i64>().unwrap()
        };

        let e = map.entry(name_a.to_string()).or_insert(Vec::new());
        e.push(Seating {
            name_a: name_a.to_string(),
            name_b: name_b.to_string(),
            value,
        })
    }
    return map;
}

fn make_permutations(stack: &[String], seatings: &Seatings, out: &mut Vec<Vec<String>>) {
    if stack.len() == seatings.len() {
        out.push(stack.to_vec());
    }

    for (name, _) in seatings {
        if stack.contains(name) {
            continue;
        }
        let mut s = stack.to_vec();
        s.push(name.to_string());
        make_permutations(&s, seatings, out);
    }
}

fn get_score(stack: &[String], seatings: &Seatings) -> i64 {
    let mut score = 0;

    for i in 0..stack.len() {
        let left_index = if i == 0 { stack.len() - 1 } else { i - 1 };
        let right_index = if i == stack.len() - 1 { 0 } else { i + 1 };
        let name = &stack[i];

        score += seatings[name]
            .iter()
            .find(|s| s.name_b == stack[left_index])
            .map(|s| s.value)
            .unwrap_or(0);

        score += seatings[name]
            .iter()
            .find(|s| s.name_b == stack[right_index])
            .map(|s| s.value)
            .unwrap_or(0);
    }
    return score;
}

fn part1(seatings: &Seatings) {
    let mut out = Vec::new();
    make_permutations(&[], &seatings, &mut out);

    let mut scores: Vec<i64> = out.iter().map(|s| get_score(s, &seatings)).collect();
    scores.sort();

    println!("Day 13A: {:?}", scores.last().unwrap());
}

fn part2(seatings: &Seatings) {
    let mut out = Vec::new();
    make_permutations(&[], &seatings, &mut out);

    let mut scores: Vec<i64> = out.iter().map(|s| get_score(s, &seatings)).collect();
    scores.sort();

    println!("Day 13B: {:?}", scores.last().unwrap());
}
