use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("./src/day8_input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &[&str]) {
    let mut count: usize = 0;

    for line in lines {
        let (_, output_digits) = line.split_once('|').unwrap();
        for code in output_digits.trim().split_whitespace() {
            count += match code.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            };
        }
    }

    println!("Day 8A: {:?}", count);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Segment {
    Top,
    TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    BottomRight,
    Bottom,
}

const SEGMENTS: [Segment; 7] = [
    Segment::Top,
    Segment::TopLeft,
    Segment::TopRight,
    Segment::Middle,
    Segment::BottomLeft,
    Segment::BottomRight,
    Segment::Bottom,
];

const DIGITS_AS_SEGMENTS: [&[Segment]; 10] = [
    &[
        Segment::Top,
        Segment::TopRight,
        Segment::TopLeft,
        Segment::BottomLeft,
        Segment::BottomRight,
        Segment::Bottom,
    ],
    &[Segment::TopRight, Segment::BottomRight],
    &[
        Segment::Top,
        Segment::TopRight,
        Segment::Middle,
        Segment::BottomLeft,
        Segment::Bottom,
    ],
    &[
        Segment::Top,
        Segment::TopRight,
        Segment::Middle,
        Segment::BottomRight,
        Segment::Bottom,
    ],
    &[
        Segment::TopLeft,
        Segment::TopRight,
        Segment::Middle,
        Segment::BottomRight,
    ],
    &[
        Segment::Top,
        Segment::TopLeft,
        Segment::Middle,
        Segment::BottomRight,
        Segment::Bottom,
    ],
    &[
        Segment::Top,
        Segment::TopLeft,
        Segment::Middle,
        Segment::BottomLeft,
        Segment::BottomRight,
        Segment::Bottom,
    ],
    &[Segment::Top, Segment::TopRight, Segment::BottomRight],
    &[
        Segment::Top,
        Segment::TopLeft,
        Segment::TopRight,
        Segment::Middle,
        Segment::BottomLeft,
        Segment::BottomRight,
        Segment::Bottom,
    ],
    &[
        Segment::Top,
        Segment::TopLeft,
        Segment::TopRight,
        Segment::Middle,
        Segment::BottomRight,
        Segment::Bottom,
    ],
];

type Orientation = HashMap<char, Segment>;

fn make_orientations(stack: &[Segment], out: &mut Vec<Orientation>) {
    if stack.len() == 7 {
        let mut ori = Orientation::new();
        let chars: Vec<char> = ('a'..='g').collect();
        for i in 0..chars.len() {
            ori.insert(chars[i], stack[i]);
        }
        out.push(ori);
        return;
    }

    for seg in SEGMENTS {
        if stack.contains(&seg) {
            continue;
        }
        let mut next = stack.to_vec();
        next.push(seg);
        make_orientations(&next, out);
    }
}

fn attempt_orientation<'a>(
    line: &'a str,
    ori: &Orientation,
) -> Option<HashMap<usize, HashSet<char>>> {
    assert_eq!(7, ori.len());

    let mut matches: HashMap<usize, HashSet<char>> = HashMap::new();

    for code in line.split_whitespace() {
        if code == "|" {
            continue;
        }
        let segments: HashSet<Segment> = code.chars().map(|chr| *ori.get(&chr).unwrap()).collect();
        for i in 0..DIGITS_AS_SEGMENTS.len() {
            let maybe_match = DIGITS_AS_SEGMENTS[i];
            let did_match = segments.len() == maybe_match.len()
                && maybe_match.iter().all(|seg| segments.contains(seg));

            if did_match {
                let chars: HashSet<char> = code.chars().collect();
                matches.insert(i, chars);
            }
        }
    }

    return match matches.len() {
        // there are 10 digits to match.
        10 => Some(matches),
        _ => None,
    };
}

fn part2(lines: &[&str]) {
    let mut orientations = Vec::new();

    // create factorial(7) orientations = 5040.
    make_orientations(&[], &mut orientations);

    let mut sum = 0;

    for line in lines {
        let mut found = None;

        for ori in &orientations {
            found = attempt_orientation(line, ori);
            if found.is_some() {
                break;
            }
        }

        assert!(found.is_some());

        let stuff = found.unwrap();
        let (_, output_digits) = line.split_once('|').unwrap();
        let mut digit_str = String::new();

        for chunk in output_digits.trim().split_whitespace() {
            let output_chars: HashSet<char> = chunk.chars().collect();
            let (found_digit, _) = stuff.iter().find(|kv| *kv.1 == output_chars).unwrap();

            digit_str.push_str(&found_digit.to_string());
        }

        sum += digit_str.parse::<u64>().unwrap();
    }

    println!("{:?}", sum);
}
