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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

            let neighbour = &mut data[y as usize][x as usize];
            if visited.contains(&neighbour) {
                continue;
            }
            let maybe_less_risky = current.least_risky_path + neighbour.risk as i16;
            neighbour.least_risky_path =
                std::cmp::min(maybe_less_risky, neighbour.least_risky_path);

            if !keyed_stack.contains(&neighbour) {
                keyed_stack.insert(*neighbour);
                sorted_stack.push(*neighbour);
            }
        }

        visited.insert(current);
    }

    return data[N - 1][N - 1].least_risky_path;
}

fn grow<const N: usize, const R: usize>(grid: &Grid<N>) -> Grid<R> {
    const RISK_VALUES: [i16; 9] = [9, 1, 2, 3, 4, 5, 6, 7, 8];

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

                    let risk_idx = ((actual.risk + to_add as i16) % 9) as usize;

                    second.data[y2][x2] = Node {
                        least_risky_path: NOT_VISITED_RISK,
                        risk: RISK_VALUES[risk_idx],
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

    const TEST_INPUT_TIMES_FIVE: &'static str = r#"
    11637517422274862853338597396444961841755517295286
    13813736722492484783351359589446246169155735727126
    21365113283247622439435873354154698446526571955763
    36949315694715142671582625378269373648937148475914
    74634171118574528222968563933317967414442817852555
    13191281372421239248353234135946434524615754563572
    13599124212461123532357223464346833457545794456865
    31254216394236532741534764385264587549637569865174
    12931385212314249632342535174345364628545647573965
    23119445813422155692453326671356443778246755488935
    22748628533385973964449618417555172952866628316397
    24924847833513595894462461691557357271266846838237
    32476224394358733541546984465265719557637682166874
    47151426715826253782693736489371484759148259586125
    85745282229685639333179674144428178525553928963666
    24212392483532341359464345246157545635726865674683
    24611235323572234643468334575457944568656815567976
    42365327415347643852645875496375698651748671976285
    23142496323425351743453646285456475739656758684176
    34221556924533266713564437782467554889357866599146
    33859739644496184175551729528666283163977739427418
    35135958944624616915573572712668468382377957949348
    43587335415469844652657195576376821668748793277985
    58262537826937364893714847591482595861259361697236
    96856393331796741444281785255539289636664139174777
    35323413594643452461575456357268656746837976785794
    35722346434683345754579445686568155679767926678187
    53476438526458754963756986517486719762859782187396
    34253517434536462854564757396567586841767869795287
    45332667135644377824675548893578665991468977611257
    44961841755517295286662831639777394274188841538529
    46246169155735727126684683823779579493488168151459
    54698446526571955763768216687487932779859814388196
    69373648937148475914825958612593616972361472718347
    17967414442817852555392896366641391747775241285888
    46434524615754563572686567468379767857948187896815
    46833457545794456865681556797679266781878137789298
    64587549637569865174867197628597821873961893298417
    45364628545647573965675868417678697952878971816398
    56443778246755488935786659914689776112579188722368
    55172952866628316397773942741888415385299952649631
    57357271266846838237795794934881681514599279262561
    65719557637682166874879327798598143881961925499217
    71484759148259586125936169723614727183472583829458
    28178525553928963666413917477752412858886352396999
    57545635726865674683797678579481878968159298917926
    57944568656815567976792667818781377892989248891319
    75698651748671976285978218739618932984172914319528
    56475739656758684176786979528789718163989182927419
    67554889357866599146897761125791887223681299833479    
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
        let expected = parse::<50>(TEST_INPUT_TIMES_FIVE);

        for y in 0..50 {
            for x in 0..50 {
                assert_eq!(expected.data[y][x], second.data[y][x]);
            }
        }
    }

    #[test]
    fn find_path_of_enlarged_example() {
        let grid = parse::<50>(TEST_INPUT_TIMES_FIVE);

        assert_eq!(315, get_risk_of_safest_path(grid));

        let grid = parse::<10>(TEST_INPUT);
        let second = grow::<10, 50>(&grid);

        assert_eq!(315, get_risk_of_safest_path(second));
    }
}
