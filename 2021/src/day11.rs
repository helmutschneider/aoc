#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Octopus {
    did_flash_at_tick: bool,
    energy: i64,
    location: Point,
}

impl Octopus {
    const EMPTY: Octopus = Octopus {
        did_flash_at_tick: false,
        energy: 0,
        location: Point { x: 0, y: 0 },
    };
}

const SIZE_X: usize = 10;
const SIZE_Y: usize = 10;

type Grid = [[Octopus; SIZE_X]; SIZE_Y];

pub fn run() {
    let input = std::fs::read_to_string("src/day11_input.txt").unwrap();
    let mut grid = [[Octopus::EMPTY; SIZE_X]; SIZE_Y];

    let mut x = 0;
    let mut y = 0;
    for line in input.trim().lines() {
        for chr in line.chars() {
            grid[y][x].energy = (chr as i64) - 48;
            grid[y][x].location = Point {
                x: x as i64,
                y: y as i64,
            };
            x += 1;
        }
        assert_eq!(SIZE_X, x);

        y += 1;
        x = 0;
    }
    assert_eq!(SIZE_Y, y);

    part1(grid);
    part2(grid);
}

fn maybe_get_point(grid: &Grid, x: i64, y: i64) -> Option<Point> {
    if x < 0 || y < 0 {
        return None;
    }

    return grid
        .get(y as usize)
        .map(|row| row.get(x as usize))
        .flatten()
        .map(|o| o.location);
}

fn tick(grid: &mut Grid) -> i64 {
    for y in 0..SIZE_Y {
        for x in 0..SIZE_X {
            grid[y][x].did_flash_at_tick = false;
            grid[y][x].energy += 1;
        }
    }
    let mut to_flash: Vec<Point> = grid
        .iter()
        .flatten()
        .filter(|o| o.energy > 9)
        .map(|o| o.location)
        .collect();

    let mut flashes = 0;

    while !to_flash.is_empty() {
        let pt = to_flash.pop().unwrap();
        let mut flash_me = &mut grid[pt.y as usize][pt.x as usize];

        if flash_me.did_flash_at_tick {
            continue;
        }

        flash_me.energy = 0;
        flash_me.did_flash_at_tick = true;
        flashes += 1;
        let neighbours = [
            // top
            maybe_get_point(grid, pt.x, pt.y - 1),
            // top right
            maybe_get_point(grid, pt.x + 1, pt.y - 1),
            // right
            maybe_get_point(grid, pt.x + 1, pt.y),
            // bottom right
            maybe_get_point(grid, pt.x + 1, pt.y + 1),
            // bottom
            maybe_get_point(grid, pt.x, pt.y + 1),
            // bottom left
            maybe_get_point(grid, pt.x - 1, pt.y + 1),
            // left
            maybe_get_point(grid, pt.x - 1, pt.y),
            // top left
            maybe_get_point(grid, pt.x - 1, pt.y - 1),
        ];

        for n in neighbours {
            if n.is_none() {
                continue;
            }
            let pt = n.unwrap();
            let mut maybe_flash_me = &mut grid[pt.y as usize][pt.x as usize];

            if maybe_flash_me.did_flash_at_tick {
                continue;
            }

            maybe_flash_me.energy += 1;
            if maybe_flash_me.energy > 9 {
                to_flash.push(maybe_flash_me.location);
            }
        }
    }

    return flashes;
}

fn print_grid(grid: &Grid) {
    for y in 0..SIZE_Y {
        for x in 0..SIZE_X {
            print!("{}", grid[y][x].energy);
        }
        println!()
    }
}

fn part1(grid: Grid) {
    let mut flashes = 0;
    let mut g = grid;

    for _ in 0..100 {
        flashes += tick(&mut g);
    }

    println!("Day 11A: {:?}", flashes);
}

fn part2(grid: Grid) {
    let mut g = grid;
    let mut i = 1;
    let expected_flashes = (SIZE_X as i64) * (SIZE_Y as i64);
    loop {
        let flashes = tick(&mut g);
        if flashes == expected_flashes {
            println!("Day 11B: {}", i);
            return;
        }
        i += 1;
    }
}
