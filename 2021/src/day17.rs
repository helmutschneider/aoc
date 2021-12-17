struct Rect {
    x0: i64,
    y0: i64,
    x1: i64,
    y1: i64,
}

const PUZZLE_INPUT: Rect = Rect {
    x0: 179,
    y0: -109,
    x1: 201,
    y1: -63,
};

#[derive(Debug)]
struct Solution {
    vx0: i64,
    vy0: i64,
    t: i64,
    x_end: i64,
    y_end: i64,
    max_y: i64,
}

fn get_solutions(target: Rect) -> Vec<Solution> {
    let mut out = Vec::new();

    // 1000 appears to be a large enough range for my input.
    for vx0 in 1..=1000 {
        for vy0 in -1000..1000 {
            let mut t = 0;
            let mut x = 0;
            let mut y = 0;
            let mut vx = vx0;
            let mut vy = vy0;
            let mut max_y = y;

            loop {
                x += vx;
                y += vy;
                vx = std::cmp::max(0, vx - 1);
                vy -= 1;
                t += 1;
                max_y = std::cmp::max(y, max_y);

                if x >= target.x0 && x <= target.x1 && y >= target.y0 && y <= target.y1 {
                    out.push(Solution {
                        vx0,
                        vy0,
                        t,
                        x_end: x,
                        y_end: y,
                        max_y: max_y,
                    });
                    break;
                }

                if x > target.x1 || y < target.y0 {
                    break;
                }
            }
        }
    }

    return out;
}

pub fn run() {
    let solutions = get_solutions(PUZZLE_INPUT);
    let maxx = solutions.iter().map(|s| s.max_y).max();

    println!("Day 17A: {:?}", maxx);
    println!("Day 17B: {:?}", solutions.len());
}
