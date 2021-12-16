use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;

pub fn run() {
    let input = std::fs::read_to_string("src/day15_input.txt").unwrap();

    part1(&input);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    risk: i64,
    x: i64,
    y: i64,
}

impl Node {
    const EMPTY: Self = Self {
        risk: 0,
        x: 0,
        y: 0,
    };
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hash: i64 = (self.y << 8) | self.x;
        state.write_i64(hash);
    }
}

struct Grid<const N: usize> {
    data: [[Node; N]; N],
}

impl<const N: usize> Grid<N> {
    fn get(&self, x: i64, y: i64) -> Option<Node> {
        return self
            .data
            .get(y as usize)
            .map(|row| row.get(x as usize))
            .flatten()
            .map(|n| *n);
    }
}

fn parse<const N: usize>(input: &str) -> Grid<N> {
    let mut out = [[Node::EMPTY; N]; N];
    let mut y = 0;

    for line in input.trim().lines() {
        let chars: Vec<char> = line.trim().chars().collect();
        assert_eq!(N, chars.len());

        for x in 0..N {
            out[y][x] = Node {
                risk: chars[x] as i64 - 48,
                x: x as i64,
                y: y as i64,
            }
        }

        y += 1;
    }

    assert_eq!(N, y);

    return Grid { data: out };
}

// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Algorithm
fn get_safest_path<const N: usize>(grid: &Grid<N>) -> i64 {
    fn sort_desc(data: &mut Vec<Node>, risks: &HashMap<Node, i64>) {
        data.sort_by(|a, b| {
            let value_a = risks[a];
            let value_b = risks[b];

            return value_b.cmp(&value_a);
        })
    }

    const NOT_VISITED_RISK: i64 = i64::MAX;

    let mut least_risky_paths: HashMap<Node, i64> = HashMap::new();

    for node in grid.data.iter().flatten() {
        least_risky_paths.insert(*node, NOT_VISITED_RISK);
    }

    least_risky_paths.insert(grid.get(0, 0).unwrap(), 0);

    let mut stack: Vec<Node> = grid.data.iter().flat_map(|n| *n).collect();

    sort_desc(&mut stack, &least_risky_paths);

    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        let neighbors = [
            grid.get(current.x - 1, current.y),
            grid.get(current.x + 1, current.y),
            grid.get(current.x, current.y + 1),
        ];
        let least_risky = *least_risky_paths.get(&current).unwrap();

        // we should not be visiting nodes that have not yet been
        // assigned a distance. this means that we got here without
        // visiting a neighbour first.
        assert_ne!(NOT_VISITED_RISK, least_risky);

        for n in neighbors {
            if n.is_none() {
                continue;
            }
            let n = n.unwrap();
            if !stack.contains(&n) {
                continue;
            }
            let current_distance = *least_risky_paths.get(&n).unwrap();
            let maybe_shorter = least_risky + n.risk;

            least_risky_paths.insert(n, std::cmp::min(current_distance, maybe_shorter));
        }

        sort_desc(&mut stack, &least_risky_paths);
    }

    let last = grid.get(N as i64 - 1, N as i64 - 1).unwrap();

    return *least_risky_paths.get(&last).unwrap();
}

fn part1(input: &str) {
    let grid = parse::<100>(input);

    let t = std::time::Instant::now();

    let dist = get_safest_path(&grid);

    println!("Day 15A: {:?}", dist);
    // println!("{:?}", t.elapsed())
}

mod tests {
    use crate::day15::*;

    const TEST_INPUT: &'static str = r#"
    1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581
    "#;

    #[test]
    fn find_the_example_path() {
        let grid = parse::<10>(TEST_INPUT);
        let safest = get_safest_path(&grid);

        assert_eq!(40, safest);
    }
}
