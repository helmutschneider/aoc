use core::panic;

pub fn run() {
    let input = std::fs::read_to_string("src/day10_input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    part1(&lines);
    part2(&lines);
}

fn get_closing_char(chr: char) -> char {
    return match chr {
        '[' => ']',
        '(' => ')',
        '{' => '}',
        '<' => '>',
        _ => panic!("Invalid character: {}.", chr),
    };
}

fn get_corrup_score(chr: char) -> i64 {
    return match chr {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Cannot determine score of {}.", chr),
    };
}

fn get_incomplete_score(chr: char) -> i64 {
    return match chr {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Cannot determine score of {}.", chr),
    };
}

enum ParseResult {
    Valid,
    Corrupt { expected: char, found: char },
    Incomplete { tokens: Vec<char> },
}

fn parse(line: &str) -> ParseResult {
    let chars: Vec<char> = line.chars().collect();
    let mut stack: Vec<char> = Vec::new();

    for chr in chars {
        match chr {
            '[' | '(' | '{' | '<' => {
                stack.push(chr);
            }
            ']' | ')' | '}' | '>' => {
                let popped = stack.pop();
                let expected = get_closing_char(popped.unwrap());

                if chr != expected {
                    return ParseResult::Corrupt {
                        expected: expected,
                        found: chr,
                    };
                }
            }
            _ => panic!("Unexpected character: {}.", chr),
        };
    }

    if stack.is_empty() {
        return ParseResult::Valid;
    }

    return ParseResult::Incomplete { tokens: stack };
}

fn part1(lines: &[&str]) {
    let mut score = 0;

    for line in lines {
        let parsed = parse(line);
        score += match parsed {
            ParseResult::Corrupt { found, expected: _ } => get_corrup_score(found),
            _ => 0,
        };
    }

    println!("Day 10A: {:?}", score);
}

fn part2(lines: &[&str]) {
    let mut scores: Vec<i64> = Vec::new();

    for line in lines {
        let parsed = parse(line);
        let inner_score: Option<i64> = match parsed {
            ParseResult::Corrupt {
                expected: _,
                found: _,
            } => None,
            ParseResult::Incomplete { mut tokens } => {
                let mut score = 0;

                while !tokens.is_empty() {
                    let token = tokens.pop().unwrap();
                    score *= 5;
                    score += get_incomplete_score(get_closing_char(token));
                }

                Some(score)
            }
            ParseResult::Valid => panic!("Line must not be valid, yet."),
        };

        if let Some(s) = inner_score {
            scores.push(s);
        }
    }

    scores.sort();

    println!("Day 10B: {:?}", scores[(scores.len() - 1) / 2]);
}
