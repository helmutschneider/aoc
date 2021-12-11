pub fn run() {
    let input = std::fs::read_to_string("src/day02_input.txt").unwrap();
    let lines: Vec<&str> = input.trim().lines().collect();
    let sizes: Vec<Size> = lines
        .iter()
        .map(|line| {
            let parts: Vec<u64> = line
                .split('x')
                .map(|num| num.parse::<u64>().unwrap())
                .collect();

            assert_eq!(3, parts.len());

            return Size {
                length: parts[0],
                width: parts[1],
                height: parts[2],
            };
        })
        .collect();

    part1(&sizes);
    part2(&sizes);
}

#[derive(Debug)]
struct Size {
    length: u64,
    width: u64,
    height: u64,
}

impl Size {
    fn get_paper_area(&self) -> u64 {
        let mut parts = [self.length, self.width, self.height];
        parts.sort();

        return (2 * self.length * self.width)
            + (2 * self.width * self.height)
            + (2 * self.height * self.length)
            + (parts[0] * parts[1]);
    }

    fn get_ribbon_length(&self) -> u64 {
        let mut parts = [
            self.length + self.height,
            self.height + self.width,
            self.length + self.width,
        ];
        parts.sort();

        return parts[0] * 2 + (self.length * self.width * self.height);
    }
}

fn part1(sizes: &[Size]) {
    let area = sizes.iter().fold(0, |carry, s| carry + s.get_paper_area());

    println!("Day 2A: {:?}", area);
}
fn part2(sizes: &[Size]) {
    let ribbon = sizes
        .iter()
        .fold(0, |carry, s| carry + s.get_ribbon_length());

    println!("Day 2B: {:?}", ribbon);
}
