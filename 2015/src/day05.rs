pub fn run() {
    let input = std::fs::read_to_string("src/day05_input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    part1(&lines);
    part2(&lines);
}

fn is_nice_string_part1(value: &str) -> bool {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    const BAD_STRINGS: [&'static str; 4] = ["ab", "cd", "pq", "xy"];

    let chars: Vec<char> = value.chars().collect();
    let mut has_repeated_letter = false;
    let mut vowels: u64 = 0;

    let has_bad_string = BAD_STRINGS.iter().any(|s| value.contains(s));

    if has_bad_string {
        return false;
    }

    for i in 0..chars.len() {
        let chr = chars[i];
        let is_vowel = VOWELS.contains(&chr);

        if is_vowel {
            vowels += 1;
        }

        if !has_repeated_letter && Some(&chr) == chars.get(i + 1) {
            has_repeated_letter = true;
        }
    }

    return vowels >= 3 && has_repeated_letter;
}

fn part1(lines: &[&str]) {
    let count = lines
        .iter()
        .fold(0, |carry, s| carry + u64::from(is_nice_string_part1(s)));

    println!("Day 5A: {}", count);
}

fn is_nice_string_part2(value: &str) -> bool {
    let chars: Vec<char> = value.chars().collect();
    let mut has_repeated_char = false;
    let mut has_reappearing_chunk = false;

    for i in 0..chars.len() {
        let chr = chars[i];
        let end_1 = std::cmp::min(chars.len(), i + 2);
        let maybe_reappearing_chunk = &value[i..end_1];

        if !has_reappearing_chunk {
            for k in (i + 2)..chars.len() {
                let end_2 = std::cmp::min(chars.len(), k + 2);
                let other = &value[k..end_2];

                if maybe_reappearing_chunk == other {
                    has_reappearing_chunk = true;
                    break;
                }
            }
        }

        if !has_repeated_char && Some(&chr) == chars.get(i + 2) {
            has_repeated_char = true;
        }
    }

    return has_repeated_char && has_reappearing_chunk;
}

fn part2(lines: &[&str]) {
    let count = lines
        .iter()
        .fold(0, |carry, s| carry + u64::from(is_nice_string_part2(s)));

    println!("Day 5B: {}", count);
}
