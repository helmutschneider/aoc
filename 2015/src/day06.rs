pub fn run() {
    let input = std::fs::read_to_string("src/day06_input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let commands = parse_commands(&lines);
    part1(&commands);
    part2(&commands);
}

struct Point {
    x: usize,
    y: usize,
}

struct Command {
    start: Point,
    end: Point,
    kind: CommandKind,
}

enum CommandKind {
    TurnOn,
    TurnOff,
    Toggle,
}

fn parse_commands(lines: &[&str]) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();
    const SPLIT_CHARS: [char; 4] = [' ', '\n', '\t', ','];

    for line in lines {
        let parts: Vec<&str> = line.trim().split(&SPLIT_CHARS[..]).collect();

        if parts[0] == "turn" && parts[1] == "on" {
            commands.push(Command {
                start: Point {
                    x: parts[2].parse().unwrap(),
                    y: parts[3].parse().unwrap(),
                },
                end: Point {
                    x: parts[5].parse().unwrap(),
                    y: parts[6].parse().unwrap(),
                },
                kind: CommandKind::TurnOn,
            });
        } else if parts[0] == "turn" && parts[1] == "off" {
            commands.push(Command {
                start: Point {
                    x: parts[2].parse().unwrap(),
                    y: parts[3].parse().unwrap(),
                },
                end: Point {
                    x: parts[5].parse().unwrap(),
                    y: parts[6].parse().unwrap(),
                },
                kind: CommandKind::TurnOff,
            });
        } else if parts[0] == "toggle" {
            commands.push(Command {
                start: Point {
                    x: parts[1].parse().unwrap(),
                    y: parts[2].parse().unwrap(),
                },
                end: Point {
                    x: parts[4].parse().unwrap(),
                    y: parts[5].parse().unwrap(),
                },
                kind: CommandKind::Toggle,
            });
        } else {
            panic!("Unknown command: {:?}", parts);
        }
    }

    return commands;
}

fn part1(commands: &[Command]) {
    let mut lights = [[false; 1000]; 1000];

    for cmd in commands {
        for y in cmd.start.y..=cmd.end.y {
            for x in cmd.start.x..=cmd.end.x {
                lights[y][x] = match cmd.kind {
                    CommandKind::TurnOn => true,
                    CommandKind::TurnOff => false,
                    CommandKind::Toggle => !lights[y][x],
                };
            }
        }
    }

    let lights_on = lights
        .iter()
        .flatten()
        .fold(0, |carry, state| carry + u64::from(*state == true));

    println!("Day 6A: {:?}", lights_on);
}

fn part2(commands: &[Command]) {
    let mut brightness = [[0; 1000]; 1000];

    for cmd in commands {
        for y in cmd.start.y..=cmd.end.y {
            for x in cmd.start.x..=cmd.end.x {
                brightness[y][x] += match cmd.kind {
                    CommandKind::TurnOn => 1,
                    CommandKind::TurnOff => -1,
                    CommandKind::Toggle => 2,
                };
                brightness[y][x] = std::cmp::max(0, brightness[y][x]);
            }
        }
    }

    let total_brightness = brightness.iter().flatten().fold(0, |carry, b| carry + b);

    println!("Day 6B: {}", total_brightness);
}
