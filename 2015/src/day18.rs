use std::fs;

pub fn run() {
    let input = fs::read_to_string("./src/day18_input.txt").unwrap();

    do_the_thing(&input, Part::One);
    do_the_thing(&input, Part::Two);
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum LightState {
    Off,
    On,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Part {
    One,
    Two,
}

type Lights<const N: usize> = [[LightState; N]; N];

fn parse_lights<const N: usize>(input: &str) -> Lights<N> {
    let mut lights = [[LightState::Off; N]; N];
    let mut i = 0;

    for line in input.lines() {
        let mut k = 0;

        for ch in line.chars() {
            lights[i][k] = if ch == '#' {
                LightState::On
            } else {
                LightState::Off
            };
            k += 1;
        }
        assert_eq!(N, k);
        i += 1;
    }
    assert_eq!(N, i);

    return lights;
}

fn is_corner(coord: (i64, i64), size: usize) -> bool {
    let edge_value = (size - 1) as i64;
    return (coord.0 == 0 && coord.1 == 0)
        || (coord.0 == 0 && coord.1 == edge_value)
        || (coord.0 == edge_value && coord.1 == 0)
        || (coord.0 == edge_value && coord.1 == edge_value);
}

fn step<const N: usize>(prev: &Lights<N>, part: Part) -> Lights<N> {
    let mut next = [[LightState::Off; N]; N];

    for i in 0..(N as i64) {
        for k in 0..(N as i64) {
            let neighbours: [(i64, i64); 8] = [
                (i - 1, k - 1),
                (i - 1, k),
                (i - 1, k + 1),
                (i, k - 1),
                (i, k + 1),
                (i + 1, k - 1),
                (i + 1, k),
                (i + 1, k + 1),
            ];
            let enabled_neighbours = neighbours
                .map(|coord| {
                    let state = prev
                        .get(coord.0 as usize)
                        .map(|arr| arr.get(coord.1 as usize))
                        .map(|b| b.map(|c| *c).unwrap_or(LightState::Off))
                        .unwrap_or(LightState::Off);

                    return state;
                })
                .into_iter()
                .filter(|s| *s == LightState::On)
                .count();
            let state = prev[i as usize][k as usize];
            let next_state = match state {
                LightState::On => {
                    if enabled_neighbours == 2
                        || enabled_neighbours == 3
                        || (part == Part::Two && is_corner((i, k), N))
                    {
                        LightState::On
                    } else {
                        LightState::Off
                    }
                }
                LightState::Off => {
                    if enabled_neighbours == 3 || (part == Part::Two && is_corner((i, k), N)) {
                        LightState::On
                    } else {
                        LightState::Off
                    }
                }
            };
            next[i as usize][k as usize] = next_state;
        }
    }

    return next;
}

fn do_the_thing(input: &str, part: Part) {
    let mut lights = parse_lights::<100>(input);

    if part == Part::Two {
        lights[0][0] = LightState::On;
        lights[0][99] = LightState::On;
        lights[99][0] = LightState::On;
        lights[99][99] = LightState::On;
    }

    for _ in 0..100 {
        let next = step(&lights, part);
        lights = next;
    }

    let enabled_count = lights
        .iter()
        .flatten()
        .filter(|l| **l == LightState::On)
        .count();

    println!(
        "Day 18{}: {}",
        if part == Part::One { "A" } else { "B" },
        enabled_count
    );
}
