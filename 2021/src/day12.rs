use std::collections::HashMap;

type Caves = HashMap<String, Vec<String>>;

pub fn run() {
    let input = std::fs::read_to_string("src/day12_input.txt").unwrap();
    let caves = parse_caves(&input);
    part1(&caves);
    part2(&caves);
}

fn parse_caves(input: &str) -> Caves {
    let mut out = Caves::new();
    for line in input.trim().lines() {
        let (from, to) = line.split_once('-').unwrap();

        let a = out.entry(from.to_string()).or_insert(Vec::new());
        a.push(to.to_string());

        let b = out.entry(to.to_string()).or_insert(Vec::new());
        b.push(from.to_string());
    }
    return out;
}

fn make_paths_part1(visited: &[String], caves: &Caves, out: &mut Vec<Vec<String>>) {
    if *visited.last().unwrap() == "end" {
        out.push(visited.to_vec());
        return;
    }

    let destinations = &caves[visited.last().unwrap()];

    for d in destinations {
        let is_uppercase = d.chars().all(char::is_uppercase);
        let can_go = !visited.contains(&d) || is_uppercase;

        if can_go {
            make_paths_part1(&[visited, &[d.to_string()][..]].concat(), caves, out);
        }
    }
}

fn make_paths_part2(visited: &[String], caves: &Caves, out: &mut Vec<Vec<String>>) {
    if *visited.last().unwrap() == "end" {
        out.push(visited.to_vec());
        return;
    }

    let destinations = &caves[visited.last().unwrap()];
    let mut visit_counts_for_small_caves = HashMap::new();

    for v in visited {
        let is_lower = v.chars().all(|c| c.is_lowercase());
        if !is_lower {
            continue;
        }
        let e = visit_counts_for_small_caves.entry(v).or_insert(0);
        *e += 1;
    }

    let did_visit_small_cave_twice = visit_counts_for_small_caves.iter().any(|kv| *kv.1 > 1);

    for d in destinations {
        if d == "start" {
            continue;
        }

        let is_uppercase = d.chars().all(char::is_uppercase);
        let visit_count = visit_counts_for_small_caves.get(d).unwrap_or(&0);
        let can_go = is_uppercase || visit_count == &0 || !did_visit_small_cave_twice;

        if can_go {
            make_paths_part2(&[visited, &[d.to_string()][..]].concat(), caves, out);
        }
    }
}

fn part1(caves: &Caves) {
    let mut out = Vec::new();
    make_paths_part1(&["start".to_string()], caves, &mut out);

    println!("Day 12A: {}", out.len());
}

fn part2(caves: &Caves) {
    let mut out = Vec::new();
    make_paths_part2(&["start".to_string()], caves, &mut out);

    println!("Day 12B: {}", out.len());
}
