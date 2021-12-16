use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;

pub fn run() {
    let input = std::fs::read_to_string("src/day15_input.txt").unwrap();

    let first = parse::<100>(&input);
    println!("Day 15A: {}", get_risk_of_safest_path(first));

    // this may overflow the stack (especially on debug builds).
    let second = grow::<100, 500>(&first);
    println!("Day 15B: {:?}", get_risk_of_safest_path(second));
}

const NOT_VISITED_RISK: i16 = i16::MAX;

#[derive(Debug, Clone, Copy)]
struct Node {
    least_risky_path: i16,
    risk: i16,
    x: i16,
    y: i16,
}

impl Node {
    const EMPTY: Self = Self {
        least_risky_path: NOT_VISITED_RISK,
        risk: 0,
        x: 0,
        y: 0,
    };
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hash: i64 = ((self.y as i64) << 32) | self.x as i64;
        state.write_i64(hash);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return other.least_risky_path.partial_cmp(&self.least_risky_path);
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.least_risky_path.cmp(&self.least_risky_path);
    }
}

#[derive(Debug, Clone, Copy)]
struct Grid<const N: usize> {
    data: [[Node; N]; N],
}

fn parse<const N: usize>(input: &str) -> Grid<N> {
    let mut out = [[Node::EMPTY; N]; N];
    let mut y = 0;

    for line in input.trim().lines() {
        let chars: Vec<char> = line.trim().chars().collect();
        assert_eq!(N, chars.len());

        for x in 0..N {
            out[y][x] = Node {
                least_risky_path: NOT_VISITED_RISK,
                risk: chars[x] as i16 - 48,
                x: x as i16,
                y: y as i16,
            }
        }

        y += 1;
    }

    assert_eq!(N, y);

    return Grid { data: out };
}

// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Algorithm
fn get_risk_of_safest_path<const N: usize>(grid: Grid<N>) -> i16 {
    let mut data = grid.data;
    let mut visited = HashSet::<Node>::new();
    let mut keyed_stack = HashSet::<Node>::new();
    let mut sorted_stack = BinaryHeap::<Node>::new();

    data[0][0].least_risky_path = 0;

    sorted_stack.push(data[0][0]);

    while !sorted_stack.is_empty() {
        let current = sorted_stack.pop().unwrap();
        keyed_stack.remove(&current);

        let maybe_neighbours = [
            (current.x - 1, current.y),
            (current.x + 1, current.y),
            (current.x, current.y + 1),
        ];

        // we should not be visiting nodes that have not yet been
        // assigned a distance. this means that we got here without
        // visiting a neighbour first.
        assert_ne!(NOT_VISITED_RISK, current.least_risky_path);
        assert_eq!(false, visited.contains(&current));

        for (x, y) in maybe_neighbours {
            // point is outside the grid...
            if x < 0 || y < 0 || x >= (N as i16) || y >= (N as i16) {
                continue;
            }

            let neigbour = &mut data[y as usize][x as usize];
            if visited.contains(&neigbour) {
                continue;
            }
            let maybe_less_risky = current.least_risky_path + neigbour.risk as i16;
            neigbour.least_risky_path = std::cmp::min(maybe_less_risky, neigbour.least_risky_path);

            if !keyed_stack.contains(&neigbour) {
                keyed_stack.insert(*neigbour);
                sorted_stack.push(*neigbour);
            }
        }

        visited.insert(current);
    }

    return data[N - 1][N - 1].least_risky_path;
}

fn grow<const N: usize, const R: usize>(grid: &Grid<N>) -> Grid<R> {
    assert_eq!(0, R % N);

    let repeat_times = R / N;
    let mut second = Grid {
        data: [[Node::EMPTY; R]; R],
    };

    for y in 0..N {
        for x in 0..N {
            let actual = grid.data[y][x];
            for tile_y in 0..repeat_times {
                for tile_x in 0..repeat_times {
                    let to_add = tile_x + tile_y;
                    let x2 = x + (N * tile_x);
                    let y2 = y + (N * tile_y);

                    // overflow 9 into 1.
                    let risk = std::cmp::max(1, (actual.risk + to_add as i16) % 10);

                    second.data[y2][x2] = Node {
                        least_risky_path: NOT_VISITED_RISK,
                        risk: risk,
                        x: x2 as i16,
                        y: y2 as i16,
                    };
                }
            }
        }
    }

    return second;
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
    fn sort_nodes() {
        let mut nodes = vec![
            Node {
                x: 0,
                y: 0,
                least_risky_path: 5,
                risk: 0,
            },
            Node {
                x: 0,
                y: 0,
                least_risky_path: 8,
                risk: 0,
            },
        ];

        nodes.sort();

        assert_eq!(5, nodes[1].least_risky_path);
    }

    #[test]
    fn find_the_example_path() {
        let grid = parse::<10>(TEST_INPUT);
        let safest = get_risk_of_safest_path(grid);

        assert_eq!(40, safest);
    }

    #[test]
    fn grow_times_5() {
        let grid = parse::<10>(TEST_INPUT);
        let second = grow::<10, 50>(&grid);

        assert_eq!(2, second.data[0][10].risk);
        assert_eq!(3, second.data[0][20].risk);

        assert_eq!(2, second.data[10][0].risk);
        assert_eq!(3, second.data[20][0].risk);

        assert_eq!(3, second.data[10][10].risk);
    }
}
