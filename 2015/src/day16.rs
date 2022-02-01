use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Compound {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

#[derive(Debug)]
struct Gift {
    sue_number: i64,
    compounds: HashMap<Compound, i64>,
}

fn parse_gift(line: &str) -> Gift {
    let mut index_of_first_colon = 0;
    let mut i = 0;

    for ch in line.chars() {
        if ch == ':' {
            index_of_first_colon = i;
            break;
        }
        i += 1;
    }

    let (_, sue_num_str) = &line[0..(index_of_first_colon)].split_once(' ').unwrap();
    let sue_num = sue_num_str.parse::<i64>().unwrap();
    let actual_line = &line[(index_of_first_colon + 1)..];
    let mut compounds = HashMap::new();

    for chunk in actual_line.split(',') {
        let (key, value) = chunk.split_once(':').unwrap();

        let compound = match key.trim() {
            "children" => Compound::Children,
            "cats" => Compound::Cats,
            "samoyeds" => Compound::Samoyeds,
            "pomeranians" => Compound::Pomeranians,
            "akitas" => Compound::Akitas,
            "vizslas" => Compound::Vizslas,
            "goldfish" => Compound::Goldfish,
            "trees" => Compound::Trees,
            "cars" => Compound::Cars,
            "perfumes" => Compound::Perfumes,
            _ => panic!(),
        };

        compounds.insert(compound, value.trim().parse::<i64>().unwrap());
    }

    return Gift {
        sue_number: sue_num,
        compounds,
    };
}

fn get_gift_likeness(gift: &Gift) -> i64 {
    let cmp = &gift.compounds;

    let cats = cmp.get(&Compound::Cats);
    let trees = cmp.get(&Compound::Trees);
    let pomer = cmp.get(&Compound::Pomeranians);
    let fish = cmp.get(&Compound::Goldfish);
    let mut score = 0;

    if let Some(&num) = cats {
        if num <= 7 {
            return 0;
        }
        score += 1;
    }

    if let Some(&num) = trees {
        if num <= 3 {
            return 0;
        }
        score += 1;
    }

    if let Some(&num) = pomer {
        if num >= 3 {
            return 0;
        }
        score += 1;
    }

    if let Some(&num) = fish {
        if num >= 5 {
            return 0;
        }
        score += 1;
    }

    let exact_compounds = [
        (Compound::Children, 3),
        (Compound::Samoyeds, 2),
        (Compound::Cars, 2),
        (Compound::Perfumes, 1),
    ];

    for (c, expected_num) in exact_compounds {
        if let Some(&actual_num) = cmp.get(&c) {
            if actual_num != expected_num {
                return 0;
            }
            score += 1;
        }
    }

    return score;
}

pub fn run() {
    let input = fs::read_to_string("./src/day16_input.txt").unwrap();
    let mut gifts = input.lines().map(parse_gift).collect::<Vec<Gift>>();

    gifts.sort_by(|a, b| {
        let al = get_gift_likeness(&a);
        let bl = get_gift_likeness(&b);

        return bl.cmp(&al);
    });

    println!("Day 16B: {:?}", gifts[0]);
}
