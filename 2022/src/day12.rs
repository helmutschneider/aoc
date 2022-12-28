use crate::println;
use crate::util::Day;
use heapless::BinaryHeap;

pub const DAY_12: Day<i16> = Day {
    year: 2022,
    day: 12,
    parts: &[do_part_1, do_part_2],
    tests: &[test_parsing, test_find_shortest_path],
};

fn do_part_1() -> i16 {
    let map = parse_map::<114, 41>(INPUT);
    let path = find_steps_of_shortest_path(map, map.start);
    return path;
}

fn do_part_2() -> i16 {
    let map = parse_map::<114, 41>(INPUT);
    let mut min = i16::MAX;
    let mut i = 0;

    for y in 0..41 {
        for x in 0..114 {
            let node = map.nodes[y][x];
            if node.height == 0 {
                let path = find_steps_of_shortest_path(map, node.position);
                min = crate::util::min(min, path);
            }

            if i % 100 == 0 {
                println!("Ok! {}", i);
            }

            i += 1;
        }
    }

    return min;
}

#[derive(Debug, Clone, Copy)]
struct HeightMap<const X: usize, const Y: usize> {
    nodes: [[Node; X]; Y],
    start: Point,
    end: Point,
}

impl<const X: usize, const Y: usize> HeightMap<X, Y> {
    fn get(&self, pt: Point) -> Node {
        return self.nodes[pt.y as usize][pt.x as usize];
    }

    fn set(&mut self, pt: Point, node: Node) {
        self.nodes[pt.y as usize][pt.x as usize] = node;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    distance: i16,
    height: i8,
    is_visited: bool,
    position: Point,
}

impl Node {
    const ZERO: Self = Self {
        distance: i16::MAX,
        height: 0,
        is_visited: false,
        position: Point::ZERO,
    };
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        return self.distance.cmp(&other.distance);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, hash32_derive::Hash32)]
struct Point {
    x: i8,
    y: i8,
}

impl Point {
    fn new(x: i8, y: i8) -> Self {
        return Self { x, y };
    }
    const ZERO: Self = Self { x: 0, y: 0 };
}

fn to_height(ch: char) -> i8 {
    return ((ch as u8) - b'a') as i8;
}

fn parse_map<const X: usize, const Y: usize>(data: &str) -> HeightMap<X, Y> {
    let lines = data.trim().lines();
    let mut nodes = [[Node::ZERO; X]; Y];
    let mut y: i8 = 0;
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    for line in lines {
        let mut x: i8 = 0;

        for ch in line.trim().chars() {
            let is_start = ch == 'S';
            let is_end = ch == 'E';
            let height: i8;

            if is_start {
                assert_eq!(None, start);

                height = to_height('a');
                start = Some(Point::new(x, y));
            } else if is_end {
                assert_eq!(None, end);

                height = to_height('z');
                end = Some(Point::new(x, y));
            } else {
                height = to_height(ch);
            }

            nodes[y as usize][x as usize] = Node {
                distance: i16::MAX,
                height,
                is_visited: false,
                position: Point::new(x, y),
            };

            x += 1;
        }

        assert_eq!(X, x as usize);

        y += 1;
    }

    assert_eq!(Y, y as usize);

    return HeightMap {
        nodes: nodes,
        start: start.unwrap(),
        end: end.unwrap(),
    };
}

fn find_steps_of_shortest_path<const X: usize, const Y: usize>(
    map: HeightMap<X, Y>,
    start: Point,
) -> i16 {
    let mut map = map;
    let mut heap: BinaryHeap<Node, heapless::binary_heap::Min, 512> = BinaryHeap::new();

    let mut start_node = map.get(start);
    start_node.distance = 0;
    map.set(start, start_node);
    heap.push(start_node).unwrap();

    while let Some(copied_node) = heap.pop() {
        let pt = copied_node.position;
        let mut node = map.get(pt);

        if node.is_visited {
            continue;
        }

        // when the distance of a node is updated we just re-add it
        // to the heap, beucase we can't mutate the data inside.
        // this means that some data in the heap might be out-of-date.
        if node.distance != copied_node.distance {
            continue;
        }

        if node.position == map.end {
            break;
        }

        node.is_visited = true;
        map.set(pt, node);

        let siblings = [
            Point::new(pt.x, pt.y - 1),
            Point::new(pt.x + 1, pt.y),
            Point::new(pt.x, pt.y + 1),
            Point::new(pt.x - 1, pt.y),
        ];

        for sibling_pt in siblings {
            if sibling_pt.x < 0 || (sibling_pt.x as usize) == X {
                continue;
            }
            if sibling_pt.y < 0 || (sibling_pt.y as usize) == Y {
                continue;
            }
            let mut sibling_node = map.get(sibling_pt);
            let can_move_to_sibling = (sibling_node.height - node.height) <= 1;

            if !can_move_to_sibling {
                continue;
            }

            let prev_distance = sibling_node.distance;
            let maybe_next_distance = node.distance + 1;
            let mut should_add_to_heap = false;

            if maybe_next_distance <= prev_distance {
                sibling_node.distance = maybe_next_distance;
                map.set(sibling_pt, sibling_node);
                should_add_to_heap = true;
            }

            if !sibling_node.is_visited {
                should_add_to_heap = true;
            }

            if should_add_to_heap {
                heap.push(sibling_node).unwrap();
            }
        }
    }

    return map.get(map.end).distance;
}

fn path_to_string<const X: usize, const Y: usize>(path: &[Point]) -> heapless::String<512> {
    let mut rows = [['.'; X]; Y];

    for i in 0..path.len() {
        let pt = path[i];
        let maybe_next_pt = path.get(i + 1);
        let ch: char;

        if let Some(next_pt) = maybe_next_pt {
            if next_pt.x > pt.x {
                ch = '>';
            } else if next_pt.x < pt.x {
                ch = '<';
            } else if next_pt.y > pt.y {
                ch = 'v';
            } else if next_pt.y < pt.y {
                ch = '^';
            } else {
                panic!("Very bad!");
            }
        } else {
            ch = 'E';
        }
        rows[pt.y as usize][pt.x as usize] = ch;
    }

    let mut s: heapless::String<512> = heapless::String::new();

    for row in rows {
        for ch in row {
            s.push(ch).unwrap();
        }
        s.push_str("\n").unwrap();
    }

    return s;
}

fn test_parsing() {
    let map = parse_map::<8, 5>(TEST_INPUT);

    assert_eq!(Point::new(0, 0), map.start);
    assert_eq!(0, map.nodes[0][0].height);
    assert_eq!(2, map.nodes[1][2].height);
}

fn test_find_shortest_path() {
    let map = parse_map::<8, 5>(TEST_INPUT);
    let path = find_steps_of_shortest_path(map, map.start);

    assert_eq!(31, path);
}

const TEST_INPUT: &'static str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

const INPUT: &'static str = r#"
abccccccccccccccccaaccccccccccccccccccccaaaaaaaaaaaaacccccccccccccccccccccccccccccccccccccccccccccccccccccccaaaaaa
abcccccccccccccaaaaaccccccccccccccccccccaaaaaaaaaaaaaccccccccccccccccccccccccccccccccccccccccccccccccccccccccaaaaa
abccccccccccccccaaaaaccccccccccccccaaaaacccaaaaaacccccaaccccccccccccccccccccccccccccccccccccccccccccccccccccccaaaa
abccccccccccccccaaaaacccccccccaacccaaaaacccaaaaaaaccccaaaacaacaaccccccccccccccccccccccccaaaccccaaaccccccccccccaaaa
abcccccccccccccaaaaacccccccaaaaaccaaaaaacccaaaaaaaacaaaaaacaaaaaccccccccccccccccccccccccaaacccaaaaccccccccccccaaac
abccccccaacaaaccccaaccccccccaaaaacaaaaaaccaaaacaaaacaaaaaccaaaaaaccccccccccccccccccccccccaaaaaaaacccccccccccccaacc
abccccccaaaaaaccccccccccccccaaaaacaaaaaaccaaaaccaaaacaaaaacaaaaaacccccccccccccccccccccccaaaaaaaaaccccccccccccccccc
abccccccaaaaaacccccccccccccaaaaaccccaaccccaacccccaaccaacaacaaaaaccccccccccccccccccccccccccaaakkkkllllcccaaaccccccc
abccccccaaaaaaacccccccccccccccaaccccaacccccccccccccccccccccccaaaccccccaaaacccccccccjjjjkkkkkkkkkkllllccccaacaccccc
abcccccaaaaaaaacccccaaccccccccccccccaaaaaaccccccccccccccccccaaccccccccaaaaccccccccjjjjjkkkkkkkkkppllllcccaaaaacccc
abcccccaaaaaaaaccaaaacccccccccccccccaaaaaccccccccccccccccaacaaccccccccaaaacccccccjjjjjjjkkkkkppppppplllccaaaaacccc
abccccccccaaaccccaaaaaacccccccccccaaaaaaaccccccccccccccccaaaaacccccccccaacccccccjjjjoooooooppppppppplllcccaaaccccc
abccccccccaaccccccaaaaaccccaacccccaaaaaaaaccccaaacccccccccaaaaaaacccccccccccccccjjjooooooooppppuuppppllcccaaaccccc
abccccccaacccccccaaaaacccccaaaccaaaaaaaaaaccaaaaaaccccccaaaaaaaaaacaaaccccccccccjjjoooouuuoopuuuuupppllcccaaaccccc
abacccccaaccccccccccaacccccaaaaaaaccaaaaaaccaaaaaaccccccaaaaaccaaaaaaaccccaaccccjjoootuuuuuuuuuuuuvpqlllcccccccccc
abaccaaaaaaaacccccccccccccccaaaaaaccaacccccccaaaaacccccccacaaaccaaaaaaccaaaacaccjjooottuuuuuuuxyuvvqqljjccddcccccc
abcccaaaaaaaaccccccccccccaaaaaaaaacaacaaccccaaaaaccccccccccaaaaaaaaaacccaaaaaacciijootttxxxuuxyyyvvqqjjjjdddcccccc
abcccccaaaaccccaaacccccccaaaaaaaaacaaaaaccccaaaaaccccccccccccaaaaaaaaacccaaaaccciiinntttxxxxxxyyvvqqqqjjjddddccccc
abccccaaaaaccccaaaaacccccaaaaaaaaaaaaaaaaccccccccccccccccccccaaaaaaaaaaccaaaaccciiinntttxxxxxxyyvvvqqqqjjjdddccccc
abccccaaaaaaccaaaaaccccccccaaaaaaaaaaaaaacccccccccccccccccccccccaaacaaacaacaaccciiinnnttxxxxxyyyvvvvqqqqjjjdddcccc
SbccccaaccaaccaaaaacccccccccaaaaaaaaaaaaacccccccccccccccccccccccaaacccccccccccciiinnntttxxxEzzyyyyvvvqqqjjjdddcccc
abcccccccccccccaaaaacccccccaaaaaaaaacaaaccccccccccccccccccccccccaaccccccccccccciiinnnttxxxxyyyyyyyyvvvqqqjjjdddccc
abcccccccccccccaaccccccccccaaaaaaaaccccccccccccccccccccccccccccccccccccccccccciiinnntttxxyyyyyyyyyvvvvqqqjjjdddccc
abccccccccccccccccccccccccaaaaaaaacccccccccccccccccccccccccccccccccccccccccccciiinntttxxxwwwyyywwvvvvrqqjjjjdddccc
abcccccccccccccccccccccccccccaaaaaaccccccccccccccccccccccccccccccccccccccccccciinnntttxwwwwwyyywwvvvrrrqkkkeddcccc
abcccccccccccccccccccccccccccaaaaaaccccccccccccccccccccccccccccccccccccccccccchhnnntttsswwswwyywwrrrrrrkkkkeeecccc
abcccccccccccccccccccccccccccaaaaaacccccccccccccccccccaccccccccccccaaacccccccchhhnmmssssssswwwwwwrrrkkkkkeeeeecccc
abcccccccccccccccccccccccccccccaaacccccccccccccccccccaaccccccccccaaaaaacccccaahhhmmmmmsssssswwwwrrrkkkkkeeeeeccccc
abaacccccccccccccaccccccccccccccccccccccccccccccccaaaaacaacccccccaaaaaacaaaaaahhhhmmmmmmmmssswwwrrkkkkeeeeeacccccc
abacccccccccccccaaaaaaaaccccccccaaacccccccaaccccccaaaaaaaacccccccaaaaaacaaaaaaahhhhmmmmmmmmsssrrrrkkkeeeeeaacccccc
abaaaccccaaccccccaaaaaacccccccccaaacccaacaaaccccccccaaaacccccccccaaaaacccaaaaaaahhhhhhhmmmmlsssrrllkfeeeeaaaaacccc
abaaaccaaaaccccccaaaaaacccccccccaaaaaaaaaaaaaacccccaaaaacccccccccaaaaacccaaaaaaachhhhhgggmllsssrrllkffeaaaaaaacccc
abaacccaaaaaacccaaaaaaaacccccaaaaaaaaaaaaaaaaacccccaacaaacccccccccccccccaaaaaacccccchggggglllllllllfffaaaaaaaacccc
abaaccccaaaacccaaaaaaaaaaccccaaaaaaaaacaaaaaaaccaccaccaaacccccccccccccccaaaaaacccccccccgggglllllllffffaaaaaacccccc
abcccccaaaaacccaaaaaaaaaacccccaaaaaaaccaaaaacccaaaccccccccccccccccccccccccccaacccccccccagggglllllffffccccaaacccccc
abcccccaacaaccccccaaaaacaccaacccaaaaaaaaaaaaaccaaacccccccccccccccccccccccccccccccccccccaagggggffffffcccccccccccccc
abcccccccccccaaaaaaaaacccccaaccaaaaaaaccaaaaacaaaaccccccccccccccccccccccccccccccccccccaaaacgggfffffccccccccccccccc
abcccccccccccaaaaacaacccaaaaaaaaaaccaacccaaaaaaaacccaaccccccccccccccccccccccccccccccccccccccggfffccccccccccccaaaca
abccccccccccaaaaaaccccccaaaaaaaaacccccccccaaaaaaaaaaaacccccccccccccaaaccccccccccccccccccccccaaaccccccccccccccaaaaa
abccccccccccaaaaaaccccccccaaaacccccccccccccaaaaaaaaaaaaccccccccccccaaaaccccccccccccccccccccccaaaccccccccccccccaaaa
abcccccccccccaaaaacccccccaaaaaaccccccccccaaaaaaaaaaaaaaccccccccccccaaaaccccccccccccccccccccccccccccccccccccccaaaaa
"#;
