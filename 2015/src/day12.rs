use std::collections::HashMap;

pub fn run() {
    let input = std::fs::read_to_string("src/day12_input.txt").unwrap();
    part1(&input);
    part2(&input);
}

#[derive(Debug)]
enum JsonNode {
    // we do not need booleans or null.
    Number(f64),
    String(String),
    Object(HashMap<String, JsonNode>),
    Array(Vec<JsonNode>),
}

fn parse(input: &str) -> JsonNode {
    let chars: Vec<char> = input.chars().collect();
    return read_node(&chars, 0).0;
}

fn read_node(chars: &[char], index: usize) -> (JsonNode, usize) {
    let mut i = skip_whitespace(chars, index);

    let node = match chars[i] {
        '"' => {
            // we do not care about strings that are now
            let (str, next) = read_string(chars, i);
            i = next;
            JsonNode::String(str)
        }
        '{' => {
            let (map, next) = read_object(chars, i);
            i = next;
            JsonNode::Object(map)
        }
        '[' => {
            let (arr, next) = read_array(chars, i);
            i = next;
            JsonNode::Array(arr)
        }
        '-' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            let (num, next) = read_number(chars, i);
            i = next;
            JsonNode::Number(num)
        }
        _ => panic!("Unexpected token: {}", chars[i]),
    };

    return (node, i);
}

fn read_string(chars: &[char], index: usize) -> (String, usize) {
    assert_eq!('"', chars[index]);

    let mut i = index + 1;
    let mut s = String::new();

    while chars[i] != '"' {
        s.push(chars[i]);
        i += 1;
    }

    assert_eq!('"', chars[i]);

    return (s, i + 1);
}

fn read_object(chars: &[char], index: usize) -> (HashMap<String, JsonNode>, usize) {
    assert_eq!('{', chars[index]);

    let mut i = skip_whitespace(chars, index + 1);
    let mut map = HashMap::new();

    while chars[i] != '}' {
        i = skip_whitespace(chars, i);
        let (key, next) = read_string(chars, i);
        i = skip_whitespace(chars, next);
        assert_eq!(':', chars[i]);
        i = skip_whitespace(chars, i + 1);
        let (node, next) = read_node(chars, i);
        i = skip_whitespace(chars, next);

        map.insert(key, node);

        if chars[i] == ',' {
            i += 1;
        }
    }

    return (map, i + 1);
}

fn read_array(chars: &[char], index: usize) -> (Vec<JsonNode>, usize) {
    assert_eq!('[', chars[index]);

    let mut i = skip_whitespace(chars, index + 1);
    let mut arr = Vec::new();

    while chars[i] != ']' {
        i = skip_whitespace(chars, i);
        let (node, next) = read_node(chars, i);
        arr.push(node);
        i = skip_whitespace(chars, next);
        if chars[i] == ',' {
            i += 1;
        }
    }
    return (arr, i + 1);
}

fn read_number(chars: &[char], index: usize) -> (f64, usize) {
    let mut i = skip_whitespace(chars, index);
    let mut s = String::new();

    while is_kind_of_numeric(chars, i) {
        s.push(chars[i]);
        i += 1;
    }

    let num: f64 = s.parse().unwrap();

    return (num, i);
}

fn skip_whitespace(chars: &[char], index: usize) -> usize {
    let mut i = index;
    while chars[i].is_whitespace() {
        i += 1;
    }
    return i;
}

fn is_kind_of_numeric(chars: &[char], index: usize) -> bool {
    return chars[index].is_numeric()
        || (chars[index] == '-' && chars[index + 1].is_numeric())
        || chars[index] == '.';
}

fn count_part1(node: &JsonNode) -> f64 {
    return match node {
        JsonNode::Number(n) => *n,
        JsonNode::Array(arr) => arr.iter().fold(0.0, |carry, n| carry + count_part1(n)),
        JsonNode::Object(map) => map.iter().fold(0.0, |carry, kv| carry + count_part1(kv.1)),
        _ => 0.0,
    };
}

fn part1(input: &str) {
    let stuff = parse(input);
    let num = count_part1(&stuff);

    println!("Day 12A: {:?}", num);
}

fn count_part2(node: &JsonNode) -> f64 {
    return match node {
        JsonNode::Number(n) => *n,
        JsonNode::Array(arr) => arr.iter().fold(0.0, |carry, n| carry + count_part2(n)),
        JsonNode::Object(map) => {
            let has_red_value = map.iter().any(|kv| {
                if let JsonNode::String(s) = kv.1 {
                    return s == "red";
                }
                return false;
            });

            if has_red_value {
                0.0
            } else {
                map.iter().fold(0.0, |carry, kv| carry + count_part2(kv.1))
            }
        }
        _ => 0.0,
    };
}

fn part2(input: &str) {
    let stuff = parse(input);
    let num = count_part2(&stuff);

    println!("Day 12B: {:?}", num);
}
