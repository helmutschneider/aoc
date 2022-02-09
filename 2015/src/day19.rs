use std::collections::HashMap;
use std::collections::HashSet;

type StringPair = (&'static str, &'static str);

const REPLACEMENTS: [StringPair; 43] = [
    ("Al", "ThF"),
    ("Al", "ThRnFAr"),
    ("B", "BCa"),
    ("B", "TiB"),
    ("B", "TiRnFAr"),
    ("Ca", "CaCa"),
    ("Ca", "PB"),
    ("Ca", "PRnFAr"),
    ("Ca", "SiRnFYFAr"),
    ("Ca", "SiRnMgAr"),
    ("Ca", "SiTh"),
    ("F", "CaF"),
    ("F", "PMg"),
    ("F", "SiAl"),
    ("H", "CRnAlAr"),
    ("H", "CRnFYFYFAr"),
    ("H", "CRnFYMgAr"),
    ("H", "CRnMgYFAr"),
    ("H", "HCa"),
    ("H", "NRnFYFAr"),
    ("H", "NRnMgAr"),
    ("H", "NTh"),
    ("H", "OB"),
    ("H", "ORnFAr"),
    ("Mg", "BF"),
    ("Mg", "TiMg"),
    ("N", "CRnFAr"),
    ("N", "HSi"),
    ("O", "CRnFYFAr"),
    ("O", "CRnMgAr"),
    ("O", "HP"),
    ("O", "NRnFAr"),
    ("O", "OTi"),
    ("P", "CaP"),
    ("P", "PTi"),
    ("P", "SiRnFAr"),
    ("Si", "CaSi"),
    ("Th", "ThCa"),
    ("Ti", "BP"),
    ("Ti", "TiTi"),
    ("e", "HF"),
    ("e", "NAl"),
    ("e", "OMg"),
];

const MOLECULE: &'static str = "ORnPBPMgArCaCaCaSiThCaCaSiThCaCaPBSiRnFArRnFArCaCaSiThCaCaSiThCaCaCaCaCaCaSiRnFYFArSiRnMgArCaSiRnPTiTiBFYPBFArSiRnCaSiRnTiRnFArSiAlArPTiBPTiRnCaSiAlArCaPTiTiBPMgYFArPTiRnFArSiRnCaCaFArRnCaFArCaSiRnSiRnMgArFYCaSiRnMgArCaCaSiThPRnFArPBCaSiRnMgArCaCaSiThCaSiRnTiMgArFArSiThSiThCaCaSiRnMgArCaCaSiRnFArTiBPTiRnCaSiAlArCaPTiRnFArPBPBCaCaSiThCaPBSiThPRnFArSiThCaSiThCaSiThCaPTiBSiRnFYFArCaCaPRnFArPBCaCaPBSiRnTiRnFArCaPRnFArSiRnCaCaCaSiThCaRnCaFArYCaSiRnFArBCaCaCaSiThFArPBFArCaSiRnFArRnCaCaCaFArSiRnFArTiRnPMgArF";

// NRnBSiRnCaRnFArYFArFArF

// Example input.
// const REPLACEMENTS: [Replacement; 3] = [("H", "HO"), ("H", "OH"), ("O", "HH")];
// const MOLECULE: &'static str = "HOH";

pub fn run() {
    part1();
    part2();
}

fn expand_once(input: &str, replacements: &[StringPair]) -> HashSet<String> {
    let mut result = HashSet::new();

    for i in 0..input.len() {
        for (from, to) in replacements {
            let end = std::cmp::min(i + from.len(), input.len());
            let chunk = &input[i..end];

            if chunk == *from {
                let str = format!("{}{}{}", &input[0..i], to, &input[end..]);
                result.insert(str);
            }
        }
    }

    return result;
}

fn part1() {
    let result = expand_once(MOLECULE, &REPLACEMENTS);
    println!("Day 19A: {}", result.len());
}

fn find_fewest_reductions_possible(
    molecule: &str,
    target: &str,
    replacements: &[StringPair],
) -> i64 {
    let mut seen = HashSet::<String>::new();
    let mut stack: Vec<(String, i64)> = vec![(molecule.to_string(), 0)];

    while !stack.is_empty() {
        stack.sort_by(|a, b| {
            return b.0.len().cmp(&a.0.len());
        });

        let (item, count) = stack.pop().unwrap();

        for &(from, to) in replacements {
            if !item.contains(to) {
                continue;
            }
            let res = item.replacen(to, from, 1);
            if seen.contains(&res) {
                continue;
            }
            if res == target {
                return count + 1;
            }
            seen.insert(res.clone());
            stack.push((res.clone(), count + 1));
        }
    }

    panic!("Bruh! Did not find a solution.");
}

fn part2() {
    let res = find_fewest_reductions_possible(MOLECULE, "e", &REPLACEMENTS);
    println!("Day 19B: {}", res);
}
