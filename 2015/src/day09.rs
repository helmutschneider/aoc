use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Distance {
    from: String,
    to: String,
    value: i64,
}

type DistanceMap = HashMap<String, Vec<Distance>>;

pub fn run() {
    let input = std::fs::read_to_string("src/day09_input.txt").unwrap();
    let dists = parse_distances(&input);

    part1(&dists);
    part2(&dists);
}

fn parse_distances(input: &str) -> DistanceMap {
    let mut out = DistanceMap::new();
    for line in input.trim().lines() {
        let chunks: Vec<&str> = line.split_whitespace().collect();
        let from = chunks[0];
        let to = chunks[2];
        let value: i64 = chunks[4].parse().unwrap();

        let a = out.entry(from.to_string()).or_insert(Vec::new());
        a.push(Distance {
            from: from.to_string(),
            to: to.to_string(),
            value,
        });

        let b = out.entry(to.to_string()).or_insert(Vec::new());
        b.push(Distance {
            from: to.to_string(),
            to: from.to_string(),
            value,
        });
    }
    return out;
}

fn make_routes(stack: &[String], dists: &DistanceMap, out: &mut Vec<Vec<String>>) {
    if stack.len() == dists.len() {
        out.push(stack.to_vec());
        return;
    }

    if stack.is_empty() {
        for (k, _) in dists {
            let s = &[k.clone()][..];
            make_routes(s, dists, out)
        }
        return;
    }

    let last = &stack[stack.len() - 1];
    let next = &dists[last];

    for n in next {
        if stack.contains(&n.to) {
            continue;
        }

        let mut s = stack.to_vec();
        s.push(n.to.clone());
        make_routes(&s, dists, out);
    }
}

fn get_routes_and_distances(dists: &DistanceMap) -> Vec<(Vec<String>, i64)> {
    let mut out = Vec::new();
    make_routes(&[], dists, &mut out);

    let mut distances: Vec<(Vec<String>, i64)> = out
        .iter()
        .map(|route| {
            let mut dist = 0;

            for i in 0..(route.len() - 1) {
                let from = &route[i];
                let to = &route[i + 1];
                let found = dists[from].iter().find(|d| d.to == *to);
                dist += found.unwrap().value;
            }

            return (route.clone(), dist);
        })
        .collect();

    distances.sort_by(|a, b| a.1.cmp(&b.1));

    return distances;
}

fn part1(dists: &DistanceMap) {
    let distances = get_routes_and_distances(dists);
    println!("Day 9A: {:?}", distances[0]);
}
fn part2(dists: &DistanceMap) {
    let distances = get_routes_and_distances(dists);
    println!("Day 9B: {:?}", distances.last().unwrap());
}
