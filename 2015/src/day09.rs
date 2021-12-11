use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Distance {
    from: String,
    to: String,
    value: i64,
}

type DistanceMap = HashMap<String, Vec<Distance>>;

const CITY_COUNT: usize = 8;

pub fn run() {
    let input = std::fs::read_to_string("src/day09_input.txt").unwrap();
    let dists = parse_distances(&input);

    assert_eq!(CITY_COUNT, dists.len());

    let routes = get_routes_and_distances(&dists);

    part1(&routes);
    part2(&routes);
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

fn make_routes(stack: &[String], dists: &DistanceMap, out: &mut Vec<[String; CITY_COUNT]>) {
    if stack.len() == dists.len() {
        let thing: [String; CITY_COUNT] = stack.to_owned().try_into().unwrap();
        out.push(thing);
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

fn get_routes_and_distances(dists: &DistanceMap) -> Vec<Distance> {
    let mut out = Vec::new();
    make_routes(&[], dists, &mut out);

    let mut distances: Vec<Distance> = out
        .iter()
        .map(|route| {
            let mut dist = 0;

            for i in 0..(route.len() - 1) {
                let from = &route[i];
                let to = &route[i + 1];
                let found = dists[from].iter().find(|d| d.to == *to);
                dist += found.unwrap().value;
            }

            return Distance {
                from: route[0].clone(),
                to: route.last().unwrap().clone(),
                value: dist,
            };
        })
        .collect();

    distances.sort_by(|a, b| a.value.cmp(&b.value));

    return distances;
}

fn part1(dists: &[Distance]) {
    println!("Day 9A: {:?}", dists[0]);
}
fn part2(dists: &[Distance]) {
    println!("Day 9B: {:?}", dists.last().unwrap());
}
