pub fn run() {
    let input = std::fs::read_to_string("src/day8_input.txt").unwrap();
    let lines: Vec<&str> = input.trim().lines().collect();

    part1(&lines);
    part2(&lines);
}

fn count_bytes_in_encoded_str(code: &str) -> usize {
    assert_eq!("\"", &code[0..1]);
    assert_eq!('"', code.chars().last().unwrap());

    let mut byte_count = 0;
    let mut i = 1;

    while i < (code.len() - 1) {
        let chr = &code[i..(i + 1)];

        if chr == "\\" {
            let next = &code[(i + 1)..(i + 2)];

            if next == "\"" || next == "\\" {
                byte_count += 1;
                i += 2;
            } else if next == "x" {
                byte_count += 1;
                i += 4;
            } else {
                panic!("Unexpected escaped character: {} in {}", chr, code);
            }
        } else {
            byte_count += 1;
            i += 1;
        }
    }

    return byte_count;
}

fn encode_str_and_count_bytes(value: &str) -> usize {
    // two mandatory double quotes.
    let mut count = 2;

    for chr in value.chars() {
        count += match chr {
            '\\' | '"' => 2,
            _ => 1,
        };
    }

    return count;
}

fn part1(lines: &[&str]) {
    let mut char_len: usize = 0;
    let mut memory_len: usize = 0;

    for line in lines {
        char_len += line.trim().len();
        memory_len += count_bytes_in_encoded_str(line);
    }
    println!("Day 8A: {:?}", char_len - memory_len);
}

fn part2(lines: &[&str]) {
    let mut char_len: usize = 0;
    let mut encoded_len: usize = 0;

    for line in lines {
        char_len += line.trim().len();
        encoded_len += encode_str_and_count_bytes(line);
    }
    println!("Day 8B: {:?}", encoded_len - char_len);
}
