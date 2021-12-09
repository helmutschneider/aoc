use std::collections::HashSet;
use std::fs;

const SIZE_X: usize = 100;
const SIZE_Y: usize = 100;
type Grid = [[u8; SIZE_X]; SIZE_Y];

pub fn run() {
    let input = fs::read_to_string("./src/day9_input.txt").unwrap();
    let mut data: Grid = [[0; 100]; 100];
    let mut i = 0;

    for line in input.trim().lines() {
        let digits: Vec<u8> = line.chars().map(|chr| (chr as u8) - 48).collect();
        assert_eq!(SIZE_X, digits.len());
        data[i] = digits.try_into().unwrap();
        i += 1;
    }

    assert_eq!(SIZE_Y, i);

    part1(&data);
    part2(&data);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    value: u64,
}

fn maybe_get_point(grid: &Grid, x: i64, y: i64) -> Option<Point> {
    if x < 0 || y < 0 {
        return None;
    }

    return grid
        .get(y as usize)
        .map(|row| row.get(x as usize))
        .flatten()
        .map(|v| Point {
            x,
            y,
            value: *v as u64,
        });
}

fn find_low_points(grid: &Grid) -> Vec<Point> {
    let mut out: Vec<Point> = Vec::new();
    for x in 0i64..(SIZE_X as i64) {
        for y in 0i64..(SIZE_Y as i64) {
            let pt = maybe_get_point(grid, x, y).unwrap();
            let left = maybe_get_point(grid, x - 1, y)
                .map(|pt| pt.value)
                .unwrap_or(u64::MAX);
            let right = maybe_get_point(grid, x + 1, y)
                .map(|pt| pt.value)
                .unwrap_or(u64::MAX);
            let top = maybe_get_point(grid, x, y - 1)
                .map(|pt| pt.value)
                .unwrap_or(u64::MAX);
            let bottom = maybe_get_point(grid, x, y + 1)
                .map(|pt| pt.value)
                .unwrap_or(u64::MAX);

            if pt.value < left && pt.value < right && pt.value < top && pt.value < bottom {
                out.push(pt);
            }
        }
    }
    return out;
}

fn part1(grid: &Grid) {
    let sum = find_low_points(grid)
        .iter()
        .fold(0, |carry, pt| carry + 1 + pt.value);

    println!("Day 9A: {:?}", sum);
}

fn get_basin(grid: &Grid, pt: Point, out: &mut HashSet<Point>) {
    if out.is_empty() {
        out.insert(pt);
    }

    let left = maybe_get_point(grid, pt.x - 1, pt.y);
    let right = maybe_get_point(grid, pt.x + 1, pt.y);
    let top = maybe_get_point(grid, pt.x, pt.y - 1);
    let bottom = maybe_get_point(grid, pt.x, pt.y + 1);

    let maybe_basin_pts: Vec<Point> = [left, right, top, bottom]
        .iter()
        .filter(|pt| pt.is_some())
        .map(|pt| pt.unwrap())
        .collect();

    for maybe_basin_pt in maybe_basin_pts {
        // the puzzle description makes it seem as if the
        // next basin point must increase by exactly 1 in
        // value, which is apparently not the case.
        if maybe_basin_pt.value > pt.value && maybe_basin_pt.value != 9 {
            get_basin(grid, maybe_basin_pt, out);
            out.insert(maybe_basin_pt);
        }
    }
}

fn print_basin(grid: &Grid, basin: &HashSet<Point>) {
    for y in 0..SIZE_Y {
        for x in 0..SIZE_X {
            let pt = maybe_get_point(grid, x as i64, y as i64).unwrap();

            if basin.contains(&pt) {
                print!("\x1b[0;31m{}\x1b[0m", pt.value);
            } else {
                print!("{}", pt.value)
            }
        }
        println!();
    }
}

fn part2(grid: &Grid) {
    let pts = find_low_points(grid);
    let mut basins: Vec<HashSet<Point>> = Vec::new();

    for pt in pts {
        let mut basin = HashSet::new();
        get_basin(grid, pt, &mut basin);
        basins.push(basin);
    }

    basins.sort_by(|a, b| a.len().cmp(&b.len()));

    let a = &basins[basins.len() - 1];
    let b = &basins[basins.len() - 2];
    let c = &basins[basins.len() - 3];

    println!("Day 9B: {}", a.len() * b.len() * c.len());
}
