use crate::util::Day;
use heapless::Vec;

type String = heapless::String<16>;

pub const DAY_05: Day<String> = Day {
    year: 2022,
    day: 5,
    parts: &[do_part_1, do_part_2],
    tests: &[
        test_parse_crates,
        test_parse_moves,
        test_execute_moves_part_1,
        test_execute_moves_part_2,
    ],
};

#[derive(Debug, Clone)]
struct CrateStack {
    number: usize,
    crates: Vec<u8, 128>,
}

#[derive(Debug, Clone, Copy)]
struct Move {
    num_crates: i32,
    from: i32,
    to: i32,
}

fn do_part_1() -> String {
    let stacks = parse_crates::<9>(INPUT_CRATES);
    let moves = parse_moves(INPUT_MOVES);
    let res = execute_moves_part_1(&stacks, &moves);

    return get_crates_on_top(&res);
}

fn do_part_2() -> String {
    let stacks = parse_crates::<9>(INPUT_CRATES);
    let moves = parse_moves(INPUT_MOVES);
    let res = execute_moves_part_2(&stacks, &moves);

    return get_crates_on_top(&res);
}

fn execute_moves_part_1(stack: &[CrateStack], moves: &[Move]) -> Vec<CrateStack, 16> {
    let mut out: Vec<CrateStack, 16> = stack.iter().map(|s| s.clone()).collect();

    for mv in moves {
        let from_index = (mv.from - 1) as usize;
        let to_index = (mv.to - 1) as usize;

        for _ in 0..mv.num_crates {
            if let Some(ch) = out[from_index].crates.pop() {
                out[to_index].crates.push(ch).unwrap();
            }
        }
    }

    return out;
}

fn execute_moves_part_2(stack: &[CrateStack], moves: &[Move]) -> Vec<CrateStack, 16> {
    let mut out: Vec<CrateStack, 16> = stack.iter().map(|s| s.clone()).collect();

    for mv in moves {
        let from_index = (mv.from - 1) as usize;
        let to_index = (mv.to - 1) as usize;
        let can_move_crates_count = out[from_index].crates.len();

        if can_move_crates_count == 0 {
            continue;
        }

        let num_crates_to_move = crate::util::min(can_move_crates_count, mv.num_crates as usize);
        let first_crate_index = can_move_crates_count - num_crates_to_move;
        let chunk_to_move: Vec<u8, 128> = out[from_index].crates[first_crate_index..]
            .iter()
            .map(|ch| *ch)
            .collect();

        for ch in chunk_to_move {
            out[to_index].crates.push(ch).unwrap();
        }

        for _ in 0..mv.num_crates {
            out[from_index].crates.pop();
        }
    }

    return out;
}

const EXPECTED_STACK_COUNT: usize = 9;
const EMPTY_CRATE_MARKER: u8 = b'_';

fn parse_crates<const N: usize>(data: &str) -> Vec<CrateStack, N> {
    let mut stacks: Vec<CrateStack, N> = (0..N)
        .map(|n| {
            return CrateStack {
                number: n + 1,
                crates: Vec::new(),
            };
        })
        .collect();

    for line in data.trim().lines() {
        for k in 0..N {
            let start = (k * 4) as usize;
            let end = crate::util::min(start + 3, line.len());
            let chunk = &line[start..end].trim();

            // the 0th character is a bracket '['.
            let stack_ch = chunk.as_bytes()[1];

            if stack_ch == EMPTY_CRATE_MARKER {
                continue;
            }

            stacks[k].crates.insert(0, stack_ch);
        }
    }

    return stacks;
}

fn parse_moves(data: &str) -> Vec<Move, 512> {
    let mut moves: Vec<Move, 512> = Vec::new();

    for line in data.trim().lines() {
        let parts: Vec<&str, 16> = line.split_whitespace().collect();
        let num_crates: i32 = parts[1].parse().unwrap();
        let from: i32 = parts[3].parse().unwrap();
        let to: i32 = parts[5].parse().unwrap();
        moves
            .push(Move {
                num_crates,
                from,
                to,
            })
            .unwrap();
    }

    return moves;
}

fn get_crates_on_top(stacks: &[CrateStack]) -> String {
    let mut str = String::new();

    for st in stacks {
        if let Some(ch) = st.crates.last() {
            str.push(*ch as char).unwrap();
        }
    }

    return str;
}

fn test_parse_crates() {
    let stacks = parse_crates::<9>(INPUT_CRATES);

    assert_eq!(9, stacks.len());
    assert_eq!(b'W', stacks[0].crates[0]);
    assert_eq!(b'R', stacks[0].crates[1]);
    assert_eq!(b'F', stacks[0].crates[2]);

    assert_eq!(b'J', stacks[8].crates[0]);
    assert_eq!(b'V', stacks[8].crates[6]);
}

fn test_parse_moves() {
    let stuff = r#"
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;
    let moves = parse_moves(stuff);

    assert_eq!(4, moves.len());
    assert_eq!(1, moves[0].num_crates);
    assert_eq!(2, moves[0].from);
    assert_eq!(1, moves[0].to);
}

fn test_execute_moves_part_1() {
    let in_crates = r#"
[_] [D] [_]
[N] [C] [_]
[Z] [M] [P]
"#;
    let in_moves = r#"
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;
    let stacks = parse_crates::<3>(in_crates);

    assert_eq!(3, stacks.len());

    let moves = parse_moves(in_moves);
    let res = execute_moves_part_1(&stacks, &moves);

    assert_eq!(&[b'C'], res[0].crates);
    assert_eq!(&[b'M'], res[1].crates);
    assert_eq!(&[b'P', b'D', b'N', b'Z'], res[2].crates);

    let top = get_crates_on_top(&res);

    assert_eq!("CMZ", top);
}

fn test_execute_moves_part_2() {
    let in_crates = r#"
[_] [D] [_]
[N] [C] [_]
[Z] [M] [P]
"#;
    let in_moves = r#"
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;
    let stacks = parse_crates::<3>(in_crates);

    assert_eq!(3, stacks.len());

    let moves = parse_moves(in_moves);
    let res = execute_moves_part_2(&stacks, &moves);

    assert_eq!(&[b'M'], res[0].crates);
    assert_eq!(&[b'C'], res[1].crates);
    assert_eq!(&[b'P', b'Z', b'N', b'D'], res[2].crates);

    let top = get_crates_on_top(&res);

    assert_eq!("MCD", top);
}

const INPUT_CRATES: &'static str = r#"
[_] [P] [_] [_] [_] [_] [C] [C] [_]
[_] [W] [_] [_] [B] [_] [G] [V] [V]
[_] [V] [_] [_] [T] [Z] [J] [T] [S]
[_] [D] [L] [_] [Q] [F] [Z] [W] [R]
[_] [C] [N] [R] [H] [L] [Q] [F] [G]
[F] [M] [Z] [H] [G] [W] [L] [R] [H]
[R] [H] [M] [C] [P] [C] [V] [N] [W]
[W] [T] [P] [J] [C] [G] [W] [P] [J]
"#;

const INPUT_MOVES: &'static str = r#"
move 2 from 4 to 9
move 5 from 2 to 9
move 1 from 5 to 1
move 3 from 1 to 4
move 2 from 4 to 6
move 7 from 6 to 9
move 5 from 3 to 9
move 1 from 8 to 6
move 3 from 2 to 3
move 12 from 9 to 3
move 4 from 9 to 7
move 15 from 3 to 9
move 1 from 1 to 5
move 2 from 6 to 5
move 18 from 9 to 1
move 6 from 8 to 1
move 1 from 8 to 7
move 5 from 7 to 2
move 6 from 1 to 2
move 7 from 9 to 6
move 6 from 1 to 3
move 5 from 3 to 9
move 3 from 9 to 1
move 1 from 7 to 9
move 4 from 2 to 1
move 5 from 6 to 3
move 1 from 3 to 4
move 1 from 5 to 9
move 2 from 9 to 6
move 5 from 5 to 9
move 10 from 1 to 8
move 4 from 3 to 8
move 3 from 4 to 9
move 4 from 6 to 9
move 14 from 8 to 6
move 1 from 3 to 8
move 14 from 9 to 4
move 6 from 1 to 6
move 1 from 8 to 2
move 3 from 5 to 8
move 1 from 8 to 9
move 1 from 8 to 1
move 5 from 4 to 9
move 1 from 8 to 4
move 3 from 9 to 4
move 3 from 7 to 5
move 7 from 6 to 3
move 7 from 4 to 1
move 3 from 9 to 1
move 7 from 2 to 3
move 1 from 4 to 8
move 8 from 6 to 2
move 2 from 7 to 4
move 1 from 7 to 4
move 1 from 7 to 9
move 1 from 5 to 9
move 1 from 9 to 4
move 1 from 4 to 2
move 8 from 4 to 9
move 1 from 4 to 2
move 5 from 9 to 4
move 2 from 6 to 9
move 1 from 6 to 9
move 1 from 8 to 1
move 13 from 3 to 2
move 1 from 3 to 9
move 2 from 6 to 8
move 1 from 8 to 1
move 14 from 1 to 7
move 4 from 2 to 1
move 2 from 9 to 5
move 3 from 9 to 7
move 1 from 8 to 2
move 4 from 1 to 5
move 1 from 4 to 7
move 3 from 9 to 1
move 7 from 7 to 4
move 14 from 2 to 8
move 3 from 1 to 7
move 3 from 5 to 4
move 2 from 1 to 9
move 11 from 8 to 9
move 3 from 7 to 8
move 3 from 8 to 6
move 6 from 4 to 3
move 2 from 6 to 8
move 8 from 4 to 3
move 3 from 8 to 7
move 2 from 8 to 2
move 2 from 3 to 9
move 1 from 6 to 8
move 5 from 2 to 7
move 10 from 9 to 7
move 1 from 8 to 5
move 3 from 5 to 2
move 6 from 7 to 5
move 19 from 7 to 3
move 9 from 5 to 9
move 6 from 2 to 6
move 2 from 7 to 3
move 29 from 3 to 8
move 2 from 7 to 9
move 5 from 8 to 1
move 12 from 9 to 6
move 1 from 3 to 8
move 1 from 2 to 7
move 1 from 3 to 1
move 10 from 6 to 1
move 1 from 6 to 7
move 9 from 1 to 9
move 2 from 1 to 2
move 12 from 9 to 4
move 7 from 6 to 3
move 8 from 3 to 7
move 5 from 7 to 6
move 19 from 8 to 3
move 10 from 4 to 6
move 1 from 4 to 6
move 6 from 8 to 6
move 1 from 4 to 2
move 6 from 6 to 3
move 3 from 2 to 7
move 13 from 6 to 3
move 1 from 9 to 1
move 6 from 1 to 8
move 1 from 6 to 5
move 1 from 5 to 4
move 3 from 7 to 1
move 2 from 1 to 3
move 11 from 3 to 8
move 1 from 4 to 3
move 3 from 8 to 4
move 1 from 7 to 5
move 3 from 8 to 9
move 2 from 9 to 2
move 7 from 8 to 3
move 1 from 7 to 9
move 1 from 1 to 4
move 32 from 3 to 4
move 1 from 5 to 9
move 2 from 8 to 3
move 2 from 6 to 4
move 1 from 9 to 4
move 1 from 9 to 2
move 3 from 3 to 1
move 1 from 8 to 6
move 1 from 6 to 2
move 1 from 9 to 3
move 1 from 1 to 7
move 1 from 8 to 7
move 2 from 3 to 8
move 1 from 8 to 4
move 1 from 1 to 2
move 2 from 4 to 8
move 1 from 1 to 8
move 26 from 4 to 6
move 3 from 8 to 5
move 3 from 7 to 6
move 7 from 6 to 3
move 18 from 6 to 8
move 16 from 8 to 9
move 1 from 5 to 1
move 2 from 8 to 3
move 3 from 9 to 8
move 3 from 6 to 4
move 2 from 5 to 4
move 1 from 6 to 4
move 2 from 7 to 2
move 2 from 3 to 9
move 4 from 8 to 3
move 1 from 1 to 2
move 6 from 9 to 7
move 2 from 2 to 5
move 12 from 3 to 1
move 9 from 9 to 2
move 10 from 1 to 3
move 2 from 5 to 9
move 8 from 4 to 7
move 13 from 7 to 6
move 6 from 6 to 5
move 4 from 5 to 3
move 2 from 5 to 4
move 8 from 4 to 3
move 1 from 7 to 2
move 15 from 2 to 7
move 8 from 3 to 7
move 1 from 1 to 6
move 7 from 7 to 1
move 5 from 1 to 6
move 7 from 3 to 2
move 3 from 1 to 6
move 12 from 7 to 9
move 12 from 9 to 8
move 1 from 7 to 1
move 2 from 9 to 5
move 1 from 1 to 9
move 4 from 4 to 2
move 4 from 8 to 4
move 2 from 7 to 2
move 4 from 6 to 5
move 4 from 8 to 9
move 1 from 8 to 4
move 5 from 5 to 3
move 5 from 2 to 4
move 5 from 9 to 5
move 1 from 3 to 6
move 1 from 7 to 8
move 12 from 3 to 9
move 4 from 2 to 6
move 7 from 4 to 9
move 13 from 6 to 4
move 3 from 6 to 9
move 4 from 4 to 2
move 1 from 3 to 4
move 21 from 9 to 7
move 4 from 2 to 1
move 3 from 5 to 4
move 8 from 7 to 6
move 2 from 7 to 2
move 11 from 4 to 2
move 1 from 9 to 7
move 1 from 5 to 7
move 1 from 1 to 8
move 5 from 2 to 5
move 1 from 3 to 5
move 2 from 4 to 9
move 3 from 4 to 8
move 3 from 1 to 8
move 1 from 9 to 6
move 8 from 7 to 8
move 9 from 6 to 5
move 1 from 9 to 6
move 1 from 6 to 4
move 3 from 7 to 5
move 1 from 6 to 9
move 12 from 5 to 1
move 2 from 5 to 8
move 1 from 9 to 6
move 2 from 7 to 6
move 9 from 1 to 8
move 1 from 6 to 9
move 1 from 9 to 2
move 1 from 4 to 2
move 2 from 6 to 7
move 5 from 8 to 3
move 2 from 7 to 4
move 16 from 8 to 5
move 2 from 3 to 8
move 7 from 5 to 1
move 3 from 3 to 8
move 7 from 5 to 7
move 4 from 5 to 2
move 6 from 7 to 9
move 2 from 9 to 6
move 2 from 9 to 2
move 1 from 6 to 8
move 12 from 2 to 6
move 2 from 9 to 6
move 1 from 5 to 2
move 3 from 5 to 4
move 9 from 2 to 6
move 6 from 8 to 3
move 1 from 7 to 5
move 1 from 6 to 7
move 1 from 7 to 8
move 1 from 5 to 8
move 5 from 1 to 2
move 3 from 4 to 5
move 4 from 6 to 8
move 5 from 2 to 9
move 5 from 8 to 4
move 1 from 1 to 4
move 9 from 8 to 4
move 1 from 2 to 3
move 3 from 6 to 8
move 4 from 9 to 2
move 2 from 6 to 4
move 2 from 3 to 1
move 4 from 4 to 7
move 6 from 4 to 5
move 10 from 6 to 8
move 4 from 1 to 9
move 4 from 7 to 5
move 3 from 3 to 9
move 6 from 9 to 8
move 2 from 2 to 9
move 8 from 4 to 3
move 2 from 2 to 7
move 1 from 4 to 9
move 6 from 3 to 8
move 2 from 7 to 8
move 6 from 5 to 9
move 5 from 5 to 6
move 2 from 5 to 9
move 7 from 9 to 5
move 2 from 1 to 9
move 6 from 5 to 8
move 1 from 5 to 1
move 2 from 3 to 6
move 1 from 3 to 6
move 4 from 9 to 5
move 1 from 3 to 4
move 1 from 1 to 2
move 1 from 2 to 1
move 1 from 6 to 8
move 14 from 8 to 5
move 6 from 5 to 1
move 16 from 8 to 3
move 2 from 8 to 2
move 10 from 6 to 7
move 1 from 6 to 9
move 2 from 2 to 9
move 2 from 7 to 3
move 1 from 8 to 5
move 3 from 9 to 1
move 4 from 9 to 5
move 9 from 3 to 8
move 2 from 3 to 6
move 5 from 3 to 8
move 1 from 4 to 2
move 12 from 8 to 4
move 1 from 8 to 9
move 4 from 5 to 9
move 7 from 7 to 1
move 10 from 5 to 2
move 2 from 5 to 2
move 1 from 6 to 5
move 2 from 5 to 2
move 5 from 2 to 6
move 4 from 9 to 6
move 6 from 4 to 9
move 2 from 3 to 4
move 6 from 4 to 7
move 6 from 7 to 5
move 10 from 1 to 5
move 4 from 1 to 2
move 4 from 6 to 3
move 6 from 9 to 7
move 2 from 4 to 9
move 7 from 7 to 6
move 1 from 9 to 7
move 2 from 9 to 8
move 2 from 8 to 2
move 1 from 2 to 5
move 3 from 8 to 4
move 4 from 2 to 7
move 3 from 4 to 7
move 2 from 3 to 5
move 2 from 3 to 2
move 18 from 5 to 3
move 6 from 3 to 1
move 8 from 3 to 1
move 8 from 7 to 9
move 9 from 2 to 5
move 3 from 2 to 3
move 7 from 3 to 7
move 3 from 6 to 4
move 1 from 7 to 1
move 7 from 6 to 7
move 1 from 2 to 9
move 1 from 4 to 2
move 13 from 7 to 2
move 10 from 5 to 3
move 1 from 2 to 9
move 7 from 1 to 5
move 8 from 9 to 5
move 1 from 9 to 5
move 1 from 9 to 8
move 1 from 8 to 2
move 8 from 5 to 3
move 18 from 3 to 5
move 2 from 4 to 1
move 3 from 2 to 5
move 27 from 5 to 1
move 17 from 1 to 5
move 2 from 2 to 3
move 1 from 6 to 5
move 2 from 2 to 5
move 1 from 6 to 4
move 1 from 6 to 9
move 2 from 3 to 5
move 17 from 5 to 6
move 1 from 9 to 3
move 6 from 2 to 4
move 1 from 3 to 2
move 3 from 4 to 9
move 1 from 2 to 9
move 1 from 4 to 7
move 3 from 5 to 2
move 2 from 5 to 1
move 1 from 5 to 2
move 1 from 7 to 3
move 18 from 1 to 4
move 1 from 3 to 1
move 5 from 4 to 2
move 1 from 5 to 1
move 9 from 2 to 7
move 1 from 4 to 5
move 1 from 2 to 9
move 8 from 6 to 2
move 13 from 4 to 2
move 2 from 4 to 9
move 1 from 5 to 2
move 1 from 6 to 8
move 6 from 7 to 5
move 1 from 8 to 4
move 1 from 7 to 6
move 1 from 6 to 1
move 7 from 6 to 5
move 1 from 7 to 9
move 6 from 9 to 3
move 2 from 9 to 7
move 2 from 5 to 7
move 4 from 7 to 8
move 4 from 5 to 4
move 1 from 6 to 7
move 3 from 3 to 8
move 6 from 5 to 9
move 2 from 3 to 5
move 4 from 4 to 7
move 1 from 3 to 1
move 2 from 2 to 3
move 6 from 9 to 6
move 1 from 7 to 1
move 19 from 2 to 4
move 2 from 5 to 6
move 2 from 8 to 9
move 2 from 1 to 2
move 2 from 2 to 5
move 2 from 4 to 3
move 4 from 6 to 2
move 1 from 7 to 8
move 6 from 1 to 8
move 3 from 5 to 1
move 5 from 2 to 5
move 1 from 6 to 7
move 9 from 8 to 1
move 2 from 3 to 6
move 4 from 6 to 5
move 1 from 6 to 2
move 9 from 5 to 2
move 3 from 4 to 6
move 12 from 4 to 6
move 1 from 9 to 4
move 1 from 3 to 1
move 3 from 4 to 8
move 1 from 3 to 6
move 6 from 6 to 2
move 1 from 4 to 5
move 3 from 6 to 2
move 4 from 1 to 5
move 1 from 5 to 1
move 2 from 8 to 9
move 7 from 6 to 3
move 1 from 3 to 1
move 1 from 8 to 1
move 3 from 8 to 9
move 4 from 3 to 5
move 3 from 7 to 3
move 5 from 3 to 7
move 1 from 9 to 1
move 4 from 9 to 2
move 15 from 2 to 7
move 14 from 1 to 7
move 5 from 5 to 1
move 9 from 7 to 2
move 1 from 9 to 6
move 1 from 7 to 4
move 1 from 4 to 6
move 2 from 6 to 2
move 9 from 2 to 5
move 4 from 2 to 4
move 4 from 7 to 5
move 6 from 5 to 9
move 7 from 1 to 8
move 6 from 2 to 8
move 1 from 1 to 2
move 3 from 9 to 5
move 18 from 7 to 8
move 2 from 4 to 6
move 2 from 4 to 6
move 3 from 7 to 6
move 3 from 5 to 3
move 1 from 2 to 6
move 5 from 6 to 8
move 29 from 8 to 1
move 2 from 3 to 5
move 25 from 1 to 6
move 2 from 9 to 5
move 1 from 7 to 8
move 6 from 8 to 2
move 1 from 9 to 1
move 15 from 6 to 8
move 1 from 3 to 8
move 14 from 8 to 7
move 5 from 1 to 3
move 1 from 6 to 2
move 2 from 5 to 7
move 10 from 6 to 2
move 4 from 5 to 7
move 6 from 5 to 1
move 2 from 1 to 4
move 19 from 7 to 9
"#;
