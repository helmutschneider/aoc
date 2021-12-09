use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("./src/day5_input.txt").unwrap();
    part1(&input);
    part2(&input);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Part {
    One,
    Two,
}

fn parse_the_thing(input: &str, part: Part) -> HashMap<Point, usize> {
    let lines = input.trim().lines().collect::<Vec<&str>>();
    let mut map: HashMap<Point, usize> = HashMap::new();

    for line in lines {
        let parts = line
            .split("->")
            .collect::<Vec<&str>>()
            .iter()
            .map(|chunk| chunk.split(',').collect::<Vec<&str>>())
            .flatten()
            .map(|num| num.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        assert_eq!(4, parts.len());

        // the lines can go left-to-right, right-to-left, up-to-down and down-to-up.
        let x1 = parts[0];
        let y1 = parts[1];
        let x2 = parts[2];
        let y2 = parts[3];

        let x_addend = if x1 == x2 {
            0
        } else if x2 > x1 {
            1
        } else {
            -1
        };
        let y_addend = if y1 == y2 {
            0
        } else if y2 > y1 {
            1
        } else {
            -1
        };

        if (part == Part::One && (x1 == x2 || y1 == y2)) || part == Part::Two {
            let mut x = x1;
            let mut y = y1;

            loop {
                let pt = Point { x, y };
                let entry = map.entry(pt).or_insert(0);
                *entry += 1;

                if pt.x == x2 && pt.y == y2 {
                    break;
                }

                x += x_addend;
                y += y_addend;
            }
        }
    }
    return map;
}

fn part1(input: &str) {
    let pts = parse_the_thing(input, Part::One);
    let mut num_overlaps: usize = 0;

    for (_, qty) in pts {
        if qty >= 2 {
            num_overlaps += 1;
        }
    }

    println!("Day 5A: {:?}", num_overlaps);
}

fn part2(input: &str) {
    let pts = parse_the_thing(input, Part::Two);
    let mut num_overlaps: usize = 0;

    for (_, qty) in pts {
        if qty >= 2 {
            num_overlaps += 1;
        }
    }

    println!("Day 5B: {:?}", num_overlaps);
}
