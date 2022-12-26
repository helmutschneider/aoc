use crate::{
    println,
    util::{Day, FnIterator},
};
use heapless::Vec;

pub const DAY_10: Day<heapless::String<512>> = Day {
    year: 2022,
    day: 10,
    parts: &[do_part_1, do_part_2],
    tests: &[
        test_parsing,
        test_execute_cycles_one_at_a_time,
        test_execute_cycles_batched,
        test_get_signal_strength,
    ],
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn needs_cycles(&self) -> i32 {
        return match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        };
    }
}

fn parse_instructions<'a>(data: &'a str) -> impl Iterator<Item = Instruction> + 'a {
    let lines = data.trim().lines();
    let iter = FnIterator::new(lines, |line| {
        let parts: Vec<&str, 8> = line.split_whitespace().collect();
        let to_match: &[&str] = &parts;

        return match to_match {
            ["noop", ..] => Instruction::Noop,
            ["addx", arg, ..] => {
                let value: i32 = arg.parse().unwrap();
                Instruction::Addx(value)
            }
            _ => panic!("Unknown instruction: {:?}", parts),
        };
    });

    return iter;
}

fn do_part_1() -> heapless::String<512> {
    let instructions: Vec<Instruction, 256> = parse_instructions(INPUT).collect();
    let sum = get_signal_strength_after_220_cycles(instructions);

    return heapless::String::from(sum);
}

fn do_part_2() -> heapless::String<512> {
    let instructions: Vec<Instruction, 256> = parse_instructions(INPUT).collect();
    let mut machine = Machine::new(instructions);
    let mut pixels = ['.'; 240];

    while !machine.instructions.is_empty() || machine.current_instruction.is_some() {
        let sprite_left = machine.X - 1;
        let sprite_right = machine.X + 1;
        let position_in_row = machine.executed_cycles % 40;
        let should_draw_pixel = position_in_row >= sprite_left && position_in_row <= sprite_right;

        if should_draw_pixel {
            let row_offset = (machine.executed_cycles / 40) * 40;
            pixels[(row_offset + position_in_row) as usize] = '#';
        }

        machine.execute_cycles(1);
    }

    let mut s: heapless::String<512> = heapless::String::from("\n");

    for k in 0..pixels.len() {
        let pixel = pixels[k];
        s.push(pixel).unwrap();

        if (k + 1) % 40 == 0 {
            s.push('\n').unwrap();
        }
    }

    return s;
}

fn test_parsing() {
    let instructions: heapless::Vec<Instruction, 16> = parse_instructions(TEST_INPUT_1).collect();

    assert_eq!(
        [
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5)
        ],
        instructions
    );
}

struct Machine<const N: usize> {
    current_instruction: Option<Instruction>,
    executed_cycles: i32,
    instructions: heapless::Vec<Instruction, N>,
    remaining_cycles_for_current_instruction: i32,
    X: i32,
}

impl<const N: usize> Machine<N> {
    fn new(instructions: heapless::Vec<Instruction, N>) -> Self {
        return Self {
            current_instruction: None,
            executed_cycles: 0,
            instructions: instructions,
            remaining_cycles_for_current_instruction: 0,
            X: 1,
        };
    }

    fn execute_cycles(&mut self, cycles: i32) {
        for _ in 0..cycles {
            let instr: Instruction = match self.current_instruction {
                Some(x) => x,
                None => {
                    let x = self.instructions.remove(0);
                    self.current_instruction = Some(x);
                    self.remaining_cycles_for_current_instruction = x.needs_cycles();
                    x
                }
            };

            assert_ne!(0, self.remaining_cycles_for_current_instruction);

            self.executed_cycles += 1;
            self.remaining_cycles_for_current_instruction -= 1;

            if self.remaining_cycles_for_current_instruction == 0 {
                self.X = match instr {
                    Instruction::Addx(value) => self.X + value,
                    _ => self.X,
                };
                self.current_instruction = None;
            }
        }
    }
}

fn get_signal_strength_after_220_cycles<const N: usize>(instructions: Vec<Instruction, N>) -> i32 {
    let mut machine = Machine::new(instructions);

    let cycles = [
        // we want the X value at the start of the cycle, so just execute 19 full cycles.
        19, 40, 40, 40, 40, 40,
    ];
    let mut sum: i32 = 0;

    for c in cycles {
        machine.execute_cycles(c);

        let product = (machine.executed_cycles + 1) * machine.X;

        sum += product;
    }

    return sum;
}

fn test_execute_cycles_one_at_a_time() {
    let instructions: heapless::Vec<Instruction, 16> = parse_instructions(TEST_INPUT_1).collect();
    let mut machine = Machine::new(instructions);

    machine.execute_cycles(1);
    assert_eq!(1, machine.X);

    machine.execute_cycles(1);
    assert_eq!(1, machine.X);

    machine.execute_cycles(1);
    assert_eq!(4, machine.X);

    machine.execute_cycles(1);
    assert_eq!(4, machine.X);

    machine.execute_cycles(1);
    assert_eq!(-1, machine.X);
}

fn test_execute_cycles_batched() {
    let instructions: heapless::Vec<Instruction, 16> = parse_instructions(TEST_INPUT_1).collect();
    let mut machine = Machine::new(instructions);

    machine.execute_cycles(5);
    assert_eq!(-1, machine.X);
}

fn test_get_signal_strength() {
    let instructions: heapless::Vec<Instruction, 256> = parse_instructions(TEST_INPUT_2).collect();
    let sum = get_signal_strength_after_220_cycles(instructions);

    assert_eq!(13_140, sum);
}

const TEST_INPUT_1: &'static str = r#"
noop
addx 3
addx -5
"#;

const TEST_INPUT_2: &'static str = r#"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;

const INPUT: &'static str = r#"
noop
addx 10
addx -4
addx -1
noop
noop
addx 5
addx -12
addx 17
noop
addx 1
addx 2
noop
addx 3
addx 2
noop
noop
addx 7
addx 3
noop
addx 2
noop
noop
addx 1
addx -38
addx 5
addx 2
addx 3
addx -2
addx 2
addx 5
addx 2
addx -4
addx 26
addx -19
addx 2
addx 5
addx -2
addx 7
addx -2
addx 5
addx 2
addx 4
addx -17
addx -23
addx 1
addx 5
addx 3
noop
addx 2
addx 24
addx 4
addx -23
noop
addx 5
addx -1
addx 6
noop
addx -2
noop
noop
noop
addx 7
addx 1
addx 4
noop
noop
noop
noop
addx -37
addx 5
addx 2
addx 1
noop
addx 4
addx -2
addx -4
addx 9
addx 7
noop
noop
addx 2
addx 3
addx -2
noop
addx -12
addx 17
noop
addx 3
addx 2
addx -3
addx -30
addx 3
noop
addx 2
addx 3
addx -2
addx 2
addx 5
addx 2
addx 11
addx -6
noop
addx 2
addx -19
addx 20
addx -7
addx 14
addx 8
addx -7
addx 2
addx -26
addx -7
noop
noop
addx 5
addx -2
addx 5
addx 15
addx -13
addx 5
noop
noop
addx 1
addx 4
addx 3
addx -2
addx 4
addx 1
noop
addx 2
noop
addx 3
addx 2
noop
noop
noop
noop
noop
"#;
