use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Distance<'a> {
    from: &'a str,
    to: &'a str,
    value: i64,
}

type DistanceMap<'a> = HashMap<&'a str, Vec<Distance<'a>>>;
type RoutesWithDistance<'a> = Vec<(Vec<&'a str>, i64)>;

pub fn run() {
    let input = std::fs::read_to_string("src/day09_input.txt").unwrap();
    let dists = parse_distances(&input);
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

        let a = out.entry(from).or_insert(Vec::new());
        a.push(Distance {
            from: from,
            to: to,
            value,
        });

        let b = out.entry(to).or_insert(Vec::new());
        b.push(Distance {
            from: to,
            to: from,
            value,
        });
    }
    return out;
}

fn make_routes<'a>(stack: &[&'a str], dists: &DistanceMap<'a>, out: &mut Vec<Vec<&'a str>>) {
    if stack.len() == dists.len() {
        out.push(stack.to_vec());
        return;
    }

    if stack.is_empty() {
        for (k, _) in dists {
            let s = &[*k][..];
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

fn get_routes_and_distances<'a>(dists: &DistanceMap<'a>) -> RoutesWithDistance<'a> {
    let mut out = Vec::new();
    make_routes(&[], dists, &mut out);

    let mut distances: Vec<(Vec<&str>, i64)> = out
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

fn part1(dists: &RoutesWithDistance) {
    println!("Day 9A: {:?}", dists[0]);
}
fn part2(dists: &RoutesWithDistance) {
    println!("Day 9B: {:?}", dists.last().unwrap());
}
