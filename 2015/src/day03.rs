use std::collections::HashSet;

pub fn run() {
    let input = std::fs::read_to_string("src/day03_input.txt").unwrap();

    part1(&input);
    part2(&input);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

fn part1(input: &str) {
    let mut map: HashSet<Point> = HashSet::new();
    let mut current = Point { x: 0, y: 0 };
    map.insert(current);

    for chr in input.trim().chars() {
        current = match chr {
            '^' => Point {
                x: current.x,
                y: current.y - 1,
            },
            '>' => Point {
                x: current.x + 1,
                y: current.y,
            },
            'v' => Point {
                x: current.x,
                y: current.y + 1,
            },
            '<' => Point {
                x: current.x - 1,
                y: current.y,
            },
            _ => current,
        };
        map.insert(current);
    }

    println!("Day 3A: {:?}", map.len());
}

fn part2(input: &str) {
    let mut santas = [Point { x: 0, y: 0 }, Point { x: 0, y: 0 }];
    let mut map: HashSet<Point> = HashSet::new();

    map.insert(santas[0]);

    let chars: Vec<char> = input.trim().chars().collect();

    for i in 0..chars.len() {
        let chr = chars[i];
        let current = santas[i % 2];
        let next = match chr {
            '^' => Point {
                x: current.x,
                y: current.y - 1,
            },
            '>' => Point {
                x: current.x + 1,
                y: current.y,
            },
            'v' => Point {
                x: current.x,
                y: current.y + 1,
            },
            '<' => Point {
                x: current.x - 1,
                y: current.y,
            },
            _ => current,
        };
        map.insert(next);
        santas[i % 2] = next;
    }

    println!("Day 3B: {:?}", map.len());
}
