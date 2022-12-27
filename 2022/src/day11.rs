use crate::{println, util::Day};
use heapless::{FnvIndexSet, Vec};

pub const DAY_11: Day<i64> = Day {
    year: 2022,
    day: 11,
    parts: &[do_part_1, do_part_2],
    tests: &[test_parsing, test_simluate, test_simluate_part_2],
};

fn do_part_1() -> i64 {
    let mut monkeys = parse_monkeys(INPUT);
    do_monkey_simulation(&mut monkeys, 20, &mut |w| w / 3);

    monkeys.sort_unstable_by(|a, b| b.inspected_items.cmp(&a.inspected_items));

    return (monkeys[0].inspected_items as i64) * (monkeys[1].inspected_items as i64);
}

fn do_part_2() -> i64 {
    let mut monkeys = parse_monkeys(INPUT);
    let factor: i64 = monkeys.iter().map(|m| m.test_divisible_by).product();
    do_monkey_simulation(&mut monkeys, 10_000, &mut |w| w % factor);

    monkeys.sort_unstable_by(|a, b| b.inspected_items.cmp(&a.inspected_items));

    return (monkeys[0].inspected_items as i64) * (monkeys[1].inspected_items as i64);
}

fn do_monkey_simulation<F: FnMut(i64) -> i64>(
    monkeys: &mut Vec<Monkey, 16>,
    rounds: i32,
    transform_worry_level: &mut F,
) {
    for round in 0..rounds {
        for monkey_index in 0..monkeys.len() {
            let monkey = monkeys[monkey_index].clone();

            for item in monkey.items {
                let mut next_item = match monkey.operation {
                    Operation::Add(Argument::Old, Argument::Old) => item + item,
                    Operation::Add(Argument::Old, Argument::Integer(value)) => item + value,
                    Operation::Multiply(Argument::Old, Argument::Old) => item * item,
                    Operation::Multiply(Argument::Old, Argument::Integer(value)) => item * value,
                    _ => panic!("Bad operation! {:?}", monkey.operation),
                };
                next_item = transform_worry_level(next_item);

                let to_index: usize = if next_item % monkey.test_divisible_by == 0 {
                    monkey.throw_to_if_true as usize
                } else {
                    monkey.throw_to_if_false as usize
                };

                monkeys[monkey_index].items.clear();
                monkeys[monkey_index].inspected_items += 1;
                monkeys[to_index].items.push(next_item).unwrap();
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    number: i32,
    items: Vec<i64, 64>,
    operation: Operation,
    test_divisible_by: i64,
    throw_to_if_true: i32,
    throw_to_if_false: i32,
    inspected_items: i32,
}

impl Monkey {
    fn new() -> Self {
        return Self {
            number: -1,
            items: heapless::Vec::new(),
            operation: Operation::None,
            test_divisible_by: -1,
            throw_to_if_true: -1,
            throw_to_if_false: -1,
            inspected_items: 0,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    None,
    Multiply(Argument, Argument),
    Add(Argument, Argument),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Argument {
    Old,
    Integer(i64),
}

const PATTERN_MONKEY: &'static str = "Monkey ";
const PATTERN_STARTING_ITEMS: &'static str = "Starting items: ";
const PATTERN_OPERATION: &'static str = "Operation: new = ";
const PATTERN_TEST_DIVISIBLE_BY: &'static str = "Test: divisible by ";
const PATTERN_TEST_IF_TRUE: &'static str = "If true: throw to monkey ";
const PATTERN_TEST_IF_FALSE: &'static str = "If false: throw to monkey ";

fn parse_monkeys(data: &str) -> heapless::Vec<Monkey, 16> {
    let lines = data.trim().lines();
    let mut monkeys: Vec<Monkey, 16> = Vec::new();
    let mut current_monkey: Option<Monkey> = None;

    for line in lines {
        let line = line.trim();

        if line.starts_with(PATTERN_MONKEY) {
            if let Some(m) = current_monkey {
                monkeys.push(m).unwrap();
            }

            let mut m = Monkey::new();
            let len = PATTERN_MONKEY.len();
            let num: i32 = line[len..(len + 1)].parse().unwrap();
            m.number = num;
            current_monkey = Some(m);
        } else if line.starts_with(PATTERN_STARTING_ITEMS) {
            let items: Vec<i64, 64> = line[PATTERN_STARTING_ITEMS.len()..]
                .split(',')
                .map(|n| n.trim().parse().unwrap())
                .collect();

            match current_monkey.as_mut() {
                Some(m) => m.items = items,
                None => panic!("No monkey!"),
            };
        } else if line.starts_with(PATTERN_OPERATION) {
            let len = PATTERN_OPERATION.len();
            let parts: Vec<&str, 4> = line[len..].split_whitespace().map(|pt| pt.trim()).collect();
            let arg_1: Argument = match parts[0] {
                "old" => Argument::Old,
                _ => {
                    let parsed: i64 = parts[0].parse().unwrap();
                    Argument::Integer(parsed)
                }
            };
            let arg_2: Argument = match parts[2] {
                "old" => Argument::Old,
                _ => {
                    let parsed: i64 = parts[2].parse().unwrap();
                    Argument::Integer(parsed)
                }
            };
            let op: Operation = match parts[1] {
                "+" => Operation::Add(arg_1, arg_2),
                "*" => Operation::Multiply(arg_1, arg_2),
                _ => panic!("Bad operation! {}", parts[1]),
            };

            match current_monkey.as_mut() {
                Some(m) => m.operation = op,
                None => panic!("No monkey!"),
            };
        } else if line.starts_with(PATTERN_TEST_DIVISIBLE_BY) {
            let len = PATTERN_TEST_DIVISIBLE_BY.len();
            let num = line[len..].parse().unwrap();

            match current_monkey.as_mut() {
                Some(m) => m.test_divisible_by = num,
                None => panic!("No monkey!"),
            };
        } else if line.starts_with(PATTERN_TEST_IF_TRUE) {
            let len = PATTERN_TEST_IF_TRUE.len();
            let num = line[len..].parse().unwrap();

            match current_monkey.as_mut() {
                Some(m) => m.throw_to_if_true = num,
                None => panic!("No monkey!"),
            };
        } else if line.starts_with(PATTERN_TEST_IF_FALSE) {
            let len = PATTERN_TEST_IF_FALSE.len();
            let num = line[len..].parse().unwrap();

            match current_monkey.as_mut() {
                Some(m) => m.throw_to_if_false = num,
                None => panic!("No monkey!"),
            };
        } else if line == "" {
            // do nothing.
        } else {
            panic!("Unrecognized line:\n{}", line);
        }
    }

    if let Some(m) = current_monkey {
        monkeys.push(m).unwrap();
    }

    return monkeys;
}

fn test_parsing() {
    let monkeys = parse_monkeys(INPUT);

    assert_eq!(8, monkeys.len());
    assert_eq!([85, 77, 77], monkeys[0].items);
    assert_eq!(
        Operation::Multiply(Argument::Old, Argument::Integer(7)),
        monkeys[0].operation
    );
    assert_eq!(19, monkeys[0].test_divisible_by);
    assert_eq!(6, monkeys[0].throw_to_if_true);
    assert_eq!(7, monkeys[0].throw_to_if_false);

    assert_eq!([50, 66, 61, 92, 64, 78], monkeys[7].items);
}

fn test_simluate() {
    let mut monkeys = parse_monkeys(TEST_INPUT_BRUH);

    do_monkey_simulation(&mut monkeys, 1, &mut |w| w / 3);

    assert_eq!([20, 23, 27, 26], monkeys[0].items);
    assert_eq!([2080, 25, 167, 207, 401, 1046], monkeys[1].items);
    assert_eq!([], monkeys[2].items);
    assert_eq!([], monkeys[3].items);
}

fn test_simluate_part_2() {
    let mut monkeys = parse_monkeys(TEST_INPUT_BRUH);
    let factor: i64 = monkeys.iter().map(|m| m.test_divisible_by).product();

    do_monkey_simulation(&mut monkeys, 20, &mut |w| w % factor);

    assert_eq!(99, monkeys[0].inspected_items);
    assert_eq!(97, monkeys[1].inspected_items);
    assert_eq!(8, monkeys[2].inspected_items);
    assert_eq!(103, monkeys[3].inspected_items);
}

const TEST_INPUT_BRUH: &'static str = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;

const INPUT: &'static str = r#"
Monkey 0:
  Starting items: 85, 77, 77
  Operation: new = old * 7
  Test: divisible by 19
    If true: throw to monkey 6
    If false: throw to monkey 7

Monkey 1:
  Starting items: 80, 99
  Operation: new = old * 11
  Test: divisible by 3
    If true: throw to monkey 3
    If false: throw to monkey 5

Monkey 2:
  Starting items: 74, 60, 74, 63, 86, 92, 80
  Operation: new = old + 8
  Test: divisible by 13
    If true: throw to monkey 0
    If false: throw to monkey 6

Monkey 3:
  Starting items: 71, 58, 93, 65, 80, 68, 54, 71
  Operation: new = old + 7
  Test: divisible by 7
    If true: throw to monkey 2
    If false: throw to monkey 4

Monkey 4:
  Starting items: 97, 56, 79, 65, 58
  Operation: new = old + 5
  Test: divisible by 5
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 5:
  Starting items: 77
  Operation: new = old + 4
  Test: divisible by 11
    If true: throw to monkey 4
    If false: throw to monkey 3

Monkey 6:
  Starting items: 99, 90, 84, 50
  Operation: new = old * old
  Test: divisible by 17
    If true: throw to monkey 7
    If false: throw to monkey 1

Monkey 7:
  Starting items: 50, 66, 61, 92, 64, 78
  Operation: new = old + 3
  Test: divisible by 2
    If true: throw to monkey 5
    If false: throw to monkey 1
"#;
