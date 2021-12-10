use std::fs;

type Grid<T> = [[T; 5]; 5];

#[derive(Debug, Clone, Copy)]
struct Number {
    is_marked: bool,
    value: i64,
}

impl Number {
    fn new(value: i64) -> Self {
        return Self {
            is_marked: false,
            value,
        };
    }

    fn mark_drawn(&mut self) {
        self.is_marked = true;
    }
}

#[derive(Debug, Clone, Copy)]
struct Board {
    data: Grid<Number>,
}

impl Board {
    fn from_array(data: Grid<i64>) -> Self {
        let nums = data.map(|row| {
            return row.map(|num| Number::new(num));
        });

        return Self { data: nums };
    }

    fn maybe_mark_number(&mut self, value: i64) {
        for i in 0..5 {
            let mut did_mark_number = false;
            for j in 0..5 {
                let num = &mut self.data[i][j];
                if num.value == value {
                    num.mark_drawn();
                    did_mark_number = true;
                    break;
                }
            }
            if did_mark_number {
                break;
            }
        }
    }

    fn is_winner(&self) -> bool {
        let mut did_win = false;

        for i in 0..5 {
            // by row
            did_win = did_win || self.data[i].iter().all(|num| num.is_marked);

            // by column
            did_win = did_win || self.data.iter().all(|row| row[i].is_marked);
        }

        return did_win;
    }
}

pub fn run() {
    let input = fs::read_to_string("./src/day04_input.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn parse_the_thing(input: &str) -> (Vec<i64>, Vec<Board>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let numbers_drawn = lines[0]
        .split(',')
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut boards: Vec<Board> = Vec::new();
    let mut i = 2;

    while i < lines.len() {
        let mut data: Grid<i64> = [[0; 5]; 5];

        for j in 0..5 {
            let line = lines[i + j];
            let numbers = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            assert_eq!(5, numbers.len());

            data[j] = numbers.try_into().unwrap();
        }

        boards.push(Board::from_array(data));

        // there is one blank line after every board.
        i += 6;
    }

    return (numbers_drawn, boards);
}

fn get_board_score(board: &Board, drawn_number: i64) -> i64 {
    let unmarked_sum = board.data.iter().flatten().fold(0, |carry, num| {
        carry + (if num.is_marked { 0 } else { num.value })
    });
    return unmarked_sum * drawn_number;
}

fn part1(input: &str) {
    let (numbers_drawn, mut boards) = parse_the_thing(input);

    for num in numbers_drawn {
        for board in &mut boards {
            board.maybe_mark_number(num);

            if board.is_winner() {
                println!("Day 4A: {:?}", get_board_score(board, num));

                return;
            }
        }
    }
}

fn part2(input: &str) {
    let (numbers_drawn, mut boards) = parse_the_thing(input);
    let mut last_winner: Option<(Board, i64)> = None;

    for num in numbers_drawn {
        for board in &mut boards {
            // skip boards that already won.
            if board.is_winner() {
                continue;
            }

            board.maybe_mark_number(num);

            if board.is_winner() {
                last_winner = Some((*board, num));
            }
        }
    }

    let (board, drawn) = last_winner.unwrap();

    println!("Day 4B: {:?}", get_board_score(&board, drawn));
}
