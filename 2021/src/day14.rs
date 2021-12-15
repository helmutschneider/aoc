use std::collections::HashMap;

pub fn run() {
    let input = std::fs::read_to_string("src/day14_input.txt").unwrap();
    part1(&input);
    part2(&input);
}

type Rules<'a> = HashMap<&'a str, char>;

fn parse(input: &str) -> (&str, Rules) {
    let lines: Vec<&str> = input.trim().lines().collect();
    let template = lines[0].trim();
    let mut rules = Rules::new();

    for i in 2..lines.len() {
        let (pair, insert_element) = lines[i].trim().split_once(" -> ").unwrap();
        rules.insert(pair, insert_element.chars().nth(0).unwrap());
    }

    return (template, rules);
}

fn polymerize(template: &str, rules: &Rules, iterations: i64) -> String {
    let mut result: Vec<char> = template.chars().collect();

    for n in 0..iterations {
        let mut parts = Vec::<char>::new();

        for i in 0..(result.len() - 1) {
            let pair = format!("{}{}", result[i], result[i + 1]);
    
            if let Some(to_insert) = rules.get(pair.as_str()) {
                if i == 0 {
                    parts.push(result[i]);
                }
                
                parts.push(*to_insert);
                parts.push(result[i + 1]);
            } else {
                parts.push(result[i]);
                parts.push(result[i + 1]);
            }
        }

        result = parts;
    }

    return String::from_iter(result);
}

fn polymerize_and_get_the_diff(input: &str, iterations: i64) -> i64 {
    let (template, rules) = parse(&input);
    let result = polymerize(template, &rules, iterations);
    let mut map = HashMap::<char, i64>::new();

    for ch in result.chars() {
        let e = map.entry(ch).or_insert(0);
        *e += 1;
    }

    let min = map.values().min().unwrap();
    let max = map.values().max().unwrap();

    return max - min;
}

fn part1(input: &str) {
    println!("Day 14A: {}", polymerize_and_get_the_diff(input, 10));
}

fn part2(input: &str) {
    // ⚠️ YOU'RE TOO SLOW ⚠️
    // println!("Day 14B: {}", polymerize_and_get_the_diff(input, 40));
}

mod tests {
    use crate::day14::*;

    const EXAMPLE_INPUT: &'static str = r#"
    NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
    "#;

    #[test]
    fn parse_the_example() {        
        let (template, rules) = parse(EXAMPLE_INPUT);

        assert_eq!("NNCB", template);
        assert_eq!(16, rules.len());
    }

    #[test]
    fn polymerize_the_example() {
        let (template, rules) = parse(EXAMPLE_INPUT);
        let mut result = template.to_string();

        result = polymerize(&result, &rules, 1);
        assert_eq!("NCNBCHB", result);

        result = polymerize(&result, &rules, 1);
        assert_eq!("NBCCNBBBCBHCB", result);

        result = polymerize(&result, &rules, 1);
        assert_eq!("NBBBCNCCNBBNBNBBCHBHHBCHB", result);

        result = polymerize(&result, &rules, 1);
        assert_eq!("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB", result);
    }

    #[test]
    fn calculate_diff_of_example() {
        assert_eq!(1588, polymerize_and_get_the_diff(EXAMPLE_INPUT, 10));
    }
}
