use std::collections::HashSet;

const CONTAINERS: [i64; 20] = [
    33, 14, 18, 20, 45, 35, 16, 35, 1, 13, 18, 13, 50, 44, 48, 6, 24, 41, 30, 42,
];

// the example input...
// const CONTAINERS: [i64; 5] = [20, 15, 10, 5, 5];

#[derive(Debug, Clone, Copy, Hash)]
struct Container {
    capacity: i64,
    index: usize,
}

impl PartialEq for Container {
    fn eq(&self, other: &Self) -> bool {
        return self.index == other.index;
    }
}

impl Eq for Container {}

fn make_combos(
    target: i64,
    carry: &[Container],
    containers: &[Container],
    out: &mut Vec<HashSet<Container>>,
) {
    let sum: i64 = carry.iter().map(|c| c.capacity).sum();

    if sum > target {
        return;
    }
    if sum == target {
        let mut result = HashSet::<Container>::new();
        result.extend(carry);

        if !out.contains(&result) {
            out.push(result);
            // println!("OK {:?}", out.len());
        }
        return;
    }

    for c in containers {
        if carry.contains(c) {
            continue;
        }

        let next_carry = &[carry, &[*c]].concat();
        make_combos(target, next_carry, containers, out);
    }
}

pub fn run() {
    let containers: Vec<Container> = CONTAINERS
        .iter()
        .enumerate()
        .map(|item| {
            return Container {
                capacity: *item.1,
                index: item.0,
            };
        })
        .collect();

    println!("{:?}", containers);

    let mut out = Vec::new();
    make_combos(150, &[], &containers, &mut out);

    println!("Day 17A: {:?}", out.len());

    let min_containers = out.iter().map(|c| c.len()).min().unwrap_or(0);
    let combos_of_min_size = out.iter().filter(|c| c.len() == min_containers);

    println!("Day 17B: {:?}", combos_of_min_size.count());
}
