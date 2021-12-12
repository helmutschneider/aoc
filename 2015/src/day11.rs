pub fn run() {
    let mut pwd = "hxbxwxba".to_string();
    let pairs = generate_pairs();
    let pairs_as_slice: Vec<&str> = pairs.iter().map(|s| s.as_str()).collect();

    loop {
        pwd = increment_password(&pwd);
        if is_valid_password(&pwd, &pairs_as_slice) {
            println!("Day 11A: {}", pwd);
            break;
        }
    }

    loop {
        pwd = increment_password(&pwd);
        if is_valid_password(&pwd, &pairs_as_slice) {
            println!("Day 11B: {}", pwd);
            break;
        }
    }
}

fn increment_password(pwd: &str) -> String {
    let mut chars: Vec<char> = pwd.chars().collect();
    let mut did_wrap = true;
    let mut i = chars.len() - 1;

    while did_wrap {
        let mut byte = (chars[i] as u8) + 1;
        did_wrap = byte > 'z' as u8;

        if did_wrap {
            byte = 'a' as u8;
        }

        chars[i] = byte as char;

        if i == 0 {
            i = chars.len() - 1;
        } else {
            i -= 1;
        }
    }

    return String::from_iter(chars);
}

fn generate_pairs() -> Vec<String> {
    return ('a'..='z').map(|ch| format!("{}{}", ch, ch)).collect();
}

const INVALID_CHARS: [char; 3] = ['i', 'o', 'l'];

fn is_valid_password(pwd: &str, pairs: &[&str]) -> bool {
    if pwd.contains(&INVALID_CHARS[..]) {
        return false;
    }

    let contains_pair_count = pairs
        .iter()
        .fold(0, |carry, p| carry + i32::from(pwd.contains(p)));

    if contains_pair_count < 2 {
        return false;
    }

    let chars: Vec<char> = pwd.chars().collect();

    for i in 0..(chars.len() - 2) {
        let a = chars[i] as u8;
        let b = chars[i + 1] as u8;
        let c = chars[i + 2] as u8;

        if a == (b - 1) && b == (c - 1) {
            return true;
        }
    }

    return false;
}
