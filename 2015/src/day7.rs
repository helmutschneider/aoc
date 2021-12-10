use std::collections::HashMap;

pub fn run() {
    let input = std::fs::read_to_string("src/day7_input.txt").unwrap();
    let lines: Vec<&str> = input.trim().lines().collect();
    let commands = parse_commands(&lines);
    part1(&commands);
    part2(&commands);
}

type WireCache = HashMap<String, u16>;

#[derive(Debug)]
enum Argument {
    Constant(u16),
    Wire(String),
}

impl Argument {
    fn parse(arg: &str) -> Self {
        let chars: Vec<char> = arg.chars().collect();
        if chars[0].is_numeric() {
            return Argument::Constant(arg.parse().unwrap());
        }
        return Argument::Wire(arg.to_string());
    }

    fn resolve(&self, commands: &[Command], cache: &mut WireCache) -> u16 {
        return match self {
            Argument::Constant(v) => *v,
            Argument::Wire(w) => resolve_wire(commands, &w, cache),
        };
    }
}

#[derive(Debug)]
enum Expression {
    Equal(Argument),
    And(Argument, Argument),
    Or(Argument, Argument),
    LeftShift(Argument, Argument),
    RightShift(Argument, Argument),
    Not(Argument),
}

impl Expression {
    fn evaluate(&self, commands: &[Command], cache: &mut WireCache) -> u16 {
        return match self {
            Expression::Equal(arg) => arg.resolve(commands, cache),
            Expression::Not(arg) => !arg.resolve(commands, cache),
            Expression::And(a, b) => a.resolve(commands, cache) & b.resolve(commands, cache),
            Expression::RightShift(a, b) => {
                a.resolve(commands, cache) >> b.resolve(commands, cache)
            }
            Expression::Or(a, b) => a.resolve(commands, cache) | b.resolve(commands, cache),
            Expression::LeftShift(a, b) => a.resolve(commands, cache) << b.resolve(commands, cache),
        };
    }
}

#[derive(Debug)]
struct Command {
    input: Expression,
    output: String,
}

fn parse_commands(lines: &[&str]) -> Vec<Command> {
    let mut out: Vec<Command> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let chars: Vec<char> = parts[0].chars().collect();

        let expr = if parts[0] == "NOT" {
            Expression::Not(Argument::parse(parts[1]))
        } else if parts[1] == "AND" {
            Expression::And(Argument::parse(parts[0]), Argument::parse(parts[2]))
        } else if parts[1] == "RSHIFT" {
            Expression::RightShift(Argument::parse(parts[0]), Argument::parse(parts[2]))
        } else if parts[1] == "LSHIFT" {
            Expression::LeftShift(Argument::parse(parts[0]), Argument::parse(parts[2]))
        } else if parts[1] == "OR" {
            Expression::Or(Argument::parse(parts[0]), Argument::parse(parts[2]))
        } else if chars[0].is_numeric() {
            Expression::Equal(Argument::parse(parts[0]))
        } else if !chars[0].is_numeric() && parts[1] == "->" {
            Expression::Equal(Argument::parse(parts[0]))
        } else {
            panic!("Unknown expression: {:?}.", parts)
        };
        out.push(Command {
            input: expr,
            output: parts.last().unwrap().to_string(),
        })
    }

    return out;
}

fn resolve_wire(commands: &[Command], wire: &str, cache: &mut HashMap<String, u16>) -> u16 {
    let maybe_cached = cache.get(wire);

    if let Some(v) = maybe_cached {
        return *v;
    }

    let found = commands.iter().find(|cmd| cmd.output == wire);
    let result = found.unwrap().input.evaluate(commands, cache);

    cache.insert(wire.to_string(), result);

    return result;
}

fn part1(commands: &[Command]) {
    let mut cache: HashMap<String, u16> = HashMap::new();
    let value = resolve_wire(commands, "a", &mut cache);

    println!("Day 7A: {:?}", value);
}

fn part2(commands: &[Command]) {
    let mut cache: HashMap<String, u16> = HashMap::new();
    cache.insert("b".to_string(), 16076);

    let value = resolve_wire(commands, "a", &mut cache);

    println!("Day 7B: {:?}", value);
}
