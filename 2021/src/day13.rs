use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

pub fn run() {
    let input = std::fs::read_to_string("src/day13_input.txt").unwrap();
    /*
        let input = r#"
    6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0

    fold along y=7
    fold along x=5
    "#;
    */
    part1(&input);
    part2(&input);
}

#[derive(Debug)]
struct Paper {
    points: HashMap<Point, bool>,
    height: i64,
    width: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
enum Fold {
    Up { y: i64 },
    Left { x: i64 },
}

enum ParseState {
    Point,
    Fold,
}

fn parse(input: &str) -> (Paper, Vec<Fold>) {
    let mut parsed_pts: HashSet<Point> = HashSet::new();
    let mut folds: Vec<Fold> = Vec::new();
    let mut parse_state = ParseState::Point;

    for line in input.trim().lines() {
        if line.is_empty() {
            parse_state = ParseState::Fold;
            continue;
        }

        match parse_state {
            ParseState::Point => {
                let (x_str, y_str) = line.split_once(',').unwrap();
                parsed_pts.insert(Point {
                    x: x_str.parse().unwrap(),
                    y: y_str.parse().unwrap(),
                });
            }
            ParseState::Fold => {
                let parts: Vec<&str> = line.split(&[' ', '='][..]).collect();

                match parts[2] {
                    "x" => folds.push(Fold::Left {
                        x: parts[3].parse().unwrap(),
                    }),
                    "y" => folds.push(Fold::Up {
                        y: parts[3].parse().unwrap(),
                    }),
                    _ => panic!("Unexpected parts[2]: {:?}", parts[2]),
                }
            }
        }
    }

    let size = parsed_pts.iter().fold(Point { x: 0, y: 0 }, |carry, pt| {
        return Point {
            x: std::cmp::max(carry.x, pt.x + 1),
            y: std::cmp::max(carry.y, pt.y + 1),
        };
    });
    let mut pts: HashMap<Point, bool> = HashMap::new();

    for y in 0..size.y {
        for x in 0..size.x {
            let pt = Point { x, y };
            pts.insert(pt, parsed_pts.contains(&pt));
        }
    }

    let paper = Paper {
        points: pts,
        height: size.y,
        width: size.x,
    };

    return (paper, folds);
}

fn get_folded_pt(pt: Point, fold: Fold) -> Option<Point> {
    let is_folded = match fold {
        Fold::Left { x } => pt.x > x,
        Fold::Up { y } => pt.y > y,
    };

    if !is_folded {
        return None;
    }

    let next_pt = match fold {
        Fold::Left { x } => Point {
            x: pt.x - (pt.x - x) * 2,
            y: pt.y,
        },
        Fold::Up { y } => Point {
            x: pt.x,
            y: pt.y - (pt.y - y) * 2,
        },
    };

    return Some(next_pt);
}

fn do_fold(paper: &Paper, fold: Fold) -> Paper {
    let (next_width, next_height) = match fold {
        Fold::Left { x } => (x, paper.height),
        Fold::Up { y } => (paper.width, y),
    };

    let mut next = Paper {
        points: HashMap::new(),
        height: next_height,
        width: next_width,
    };

    for y in 0..paper.height {
        for x in 0..paper.width {
            let pt = Point { x, y };

            // point disappears when folding.
            if pt.x == next_width || pt.y == next_height {
                continue;
            }

            let is_folded = pt.x > next_width || pt.y > next_height;
            let is_currently_set = paper.points[&pt];
            if is_folded {
                let next_pt = get_folded_pt(pt, fold).unwrap();
                let is_folded_set = *paper.points.get(&next_pt).unwrap_or(&false);
                next.points
                    .insert(next_pt, is_currently_set || is_folded_set);
            } else {
                next.points.insert(pt, is_currently_set);
            };
        }
    }

    return next;
}

fn print_paper(paper: &Paper) {
    for y in 0..paper.height {
        for x in 0..paper.width {
            let pt = Point { x, y };
            let is_set = paper.points[&pt];
            print!("{}", if is_set { '#' } else { '.' });
        }
        println!()
    }
    println!()
}

fn part1(input: &str) {
    let (mut paper, folds) = parse(input);

    paper = do_fold(&paper, folds[0]);

    let visible_dots = paper
        .points
        .iter()
        .fold(0, |carry, pt| carry + i64::from(*pt.1));

    println!("Day 13A: {:?}", visible_dots);
}

fn part2(input: &str) {
    let (mut paper, folds) = parse(input);

    for fold in folds {
        paper = do_fold(&paper, fold);
    }

    println!("Day 13B:");
    print_paper(&paper);
}

mod tests {
    use crate::day13::*;

    #[test]
    fn get_folded_pt_with_non_folded_pt() {
        let pt = get_folded_pt(Point { x: 5, y: 7 }, Fold::Up { y: 8 });

        assert_eq!(None, pt);
    }

    #[test]
    fn get_folded_pt_with_y_fold() {
        let pt = get_folded_pt(Point { x: 5, y: 14 }, Fold::Up { y: 7 }).unwrap();

        assert_eq!(5, pt.x);
        assert_eq!(0, pt.y);
    }

    #[test]
    fn get_folded_pt_with_point_close_to_fold() {
        let pt = get_folded_pt(Point { x: 5, y: 8 }, Fold::Up { y: 7 }).unwrap();

        assert_eq!(5, pt.x);
        assert_eq!(6, pt.y);
    }

    #[test]
    fn get_folded_pt_with_point_close_to_fold_x() {
        let pt = get_folded_pt(Point { x: 5, y: 8 }, Fold::Left { x: 4 }).unwrap();

        assert_eq!(3, pt.x);
        assert_eq!(8, pt.y);
    }

    #[test]
    fn get_folded_pt_with_non_symmetric_fold() {
        let pt = get_folded_pt(Point { x: 5, y: 14 }, Fold::Up { y: 12 }).unwrap();

        assert_eq!(5, pt.x);
        assert_eq!(10, pt.y);
    }
}
