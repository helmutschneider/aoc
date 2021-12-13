use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

pub fn run() {
    let input = std::fs::read_to_string("src/day13_input.txt").unwrap();
    part1(&input);
    part2(&input);
}

#[derive(Debug)]
struct Paper {
    points: HashSet<Point>,
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

enum ParserState {
    Point,
    Fold,
}

fn parse(input: &str) -> (Paper, Vec<Fold>) {
    let mut parsed_pts: HashSet<Point> = HashSet::new();
    let mut folds: Vec<Fold> = Vec::new();
    let mut parse_state = ParserState::Point;

    for line in input.trim().lines() {
        if line.is_empty() {
            parse_state = ParserState::Fold;
            continue;
        }

        match parse_state {
            ParserState::Point => {
                let (x_str, y_str) = line.split_once(',').unwrap();
                parsed_pts.insert(Point {
                    x: x_str.parse().unwrap(),
                    y: y_str.parse().unwrap(),
                });
            }
            ParserState::Fold => {
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

    let paper = Paper {
        points: parsed_pts,
        height: size.y,
        width: size.x,
    };

    return (paper, folds);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MaybeFoldedPoint {
    Unchanged,
    Folded(Point),
    Disappears,
}

fn get_folded_pt(pt: Point, fold: Fold) -> MaybeFoldedPoint {
    if matches!(fold, Fold::Left { x } if x == pt.x) {
        return MaybeFoldedPoint::Disappears;
    }
    if matches!(fold, Fold::Up { y } if y == pt.y) {
        return MaybeFoldedPoint::Disappears;
    }

    let is_folded = match fold {
        Fold::Left { x } => pt.x > x,
        Fold::Up { y } => pt.y > y,
    };

    if !is_folded {
        return MaybeFoldedPoint::Unchanged;
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

    return MaybeFoldedPoint::Folded(next_pt);
}

fn do_fold(paper: &Paper, fold: Fold) -> Paper {
    let (next_width, next_height) = match fold {
        Fold::Left { x } => (x, paper.height),
        Fold::Up { y } => (paper.width, y),
    };

    let mut next = Paper {
        points: HashSet::new(),
        height: next_height,
        width: next_width,
    };

    for pt in &paper.points {
        let next_pt = get_folded_pt(*pt, fold);

        match next_pt {
            MaybeFoldedPoint::Unchanged => {
                next.points.insert(*pt);
            }
            MaybeFoldedPoint::Folded(p) => {
                next.points.insert(p);
            }
            MaybeFoldedPoint::Disappears => {}
        };
    }

    return next;
}

fn print_paper(paper: &Paper) {
    for y in 0..paper.height {
        for x in 0..paper.width {
            let pt = Point { x, y };
            let is_set = paper.points.contains(&pt);
            print!("{}", if is_set { '#' } else { '.' });
        }
        println!()
    }
    println!()
}

fn part1(input: &str) {
    let (mut paper, folds) = parse(input);

    paper = do_fold(&paper, folds[0]);
    let visible_dots = paper.points.len();

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

        assert_eq!(MaybeFoldedPoint::Unchanged, pt);
    }

    #[test]
    fn get_folded_pt_with_y_fold() {
        let pt = get_folded_pt(Point { x: 5, y: 14 }, Fold::Up { y: 7 });

        assert_eq!(MaybeFoldedPoint::Folded(Point { x: 5, y: 0 }), pt);
    }

    #[test]
    fn get_folded_pt_with_point_close_to_fold() {
        let pt = get_folded_pt(Point { x: 5, y: 8 }, Fold::Up { y: 7 });

        assert_eq!(MaybeFoldedPoint::Folded(Point { x: 5, y: 6 }), pt);
    }

    #[test]
    fn get_folded_pt_with_point_close_to_fold_x() {
        let pt = get_folded_pt(Point { x: 5, y: 8 }, Fold::Left { x: 4 });

        assert_eq!(MaybeFoldedPoint::Folded(Point { x: 3, y: 8 }), pt);
    }

    #[test]
    fn get_folded_pt_with_non_symmetric_fold() {
        let pt = get_folded_pt(Point { x: 5, y: 14 }, Fold::Up { y: 12 });

        assert_eq!(MaybeFoldedPoint::Folded(Point { x: 5, y: 10 }), pt);
    }
}
