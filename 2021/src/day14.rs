use std::{collections::HashMap};

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

type KnownExpansion = HashMap<char, i64>;

fn polymerize_and_count_chars(input: &str, rules: &Rules, iters: i64) -> HashMap<char, i64> {
    fn rec(pair: &str, rules: &Rules, expand_to_depth: i64, cache: &mut HashMap<(String, i64), KnownExpansion>, out: &mut HashMap<char, i64>) {
        assert_eq!(2, pair.len());

        let cache_key = (pair.to_string(), expand_to_depth);

        if let Some(known) = cache.get(&cache_key) {
            for (ch, count) in known {
                let e = out.entry(*ch).or_insert(0);
                *e += count;
            }
            return;
        }
        
        // cache the entire subtree.
        let mut inner_out = HashMap::new();
        let ch = rules[pair];
        inner_out.insert(ch, 1);

        if expand_to_depth > 1 {
            let next_expand_to = expand_to_depth - 1;
            let left = format!("{}{}", &pair[0..1], ch);
            let right = format!("{}{}", ch, &pair[1..2]);

            rec(&left, rules, next_expand_to, cache, &mut inner_out);
            rec(&right, rules, next_expand_to, cache, &mut inner_out);
        }

        for (ch, count) in &inner_out {
            let e = out.entry(*ch).or_insert(0);
            *e += count;
        }

        cache.insert(cache_key, inner_out);
    }

    let mut out = HashMap::new();
    let mut cache = HashMap::new();

    for ch in input.chars() {
        let e = out.entry(ch).or_insert(0);
        *e += 1;
    }

    for i in 0..(input.len() - 1) {
        let pair = &input[i..(i + 2)];
        rec(pair, rules, iters, &mut cache, &mut out);
    }
    
    return out;
}

fn polymerize_and_get_the_diff(input: &str, iterations: i64) -> i64 {
    let (template, rules) = parse(input);
    let chars = polymerize_and_count_chars(template, &rules, iterations);

    let max = chars.values().max().unwrap();
    let min = chars.values().min().unwrap();

    return max - min;
}

fn part1(input: &str) {
    println!("Day 14A: {}", polymerize_and_get_the_diff(input, 10));
}

fn part2(input: &str) {
    println!("Day 14B: {}", polymerize_and_get_the_diff(input, 40));
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

    fn count_chars(value: &str) -> HashMap<char, i64> {
        let mut out = HashMap::new();
        for ch in value.chars() {
            let e = out.entry(ch).or_insert(0);
            *e += 1;
        }
        return out;
    }

    #[test]
    fn parse_the_example() {        
        let (template, rules) = parse(EXAMPLE_INPUT);

        assert_eq!("NNCB", template);
        assert_eq!(16, rules.len());
    }

    #[test]
    fn polymerize_the_example() {
        let (template, rules) = parse(EXAMPLE_INPUT);
        assert_eq!(count_chars("NCNBCHB"), polymerize_and_count_chars(template, &rules, 1));
        assert_eq!(count_chars("NBCCNBBBCBHCB"), polymerize_and_count_chars(template, &rules, 2));
        assert_eq!(count_chars("NBBBCNCCNBBNBNBBCHBHHBCHB"), polymerize_and_count_chars(template, &rules, 3));
        assert_eq!(count_chars("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"), polymerize_and_count_chars(template, &rules, 4));
    }

    #[test]
    fn calculate_diff_of_example() {
        assert_eq!(1588, polymerize_and_get_the_diff(EXAMPLE_INPUT, 10));
    }
}
