use std::collections::{hash_map::Entry, HashMap};

pub fn do_part_1() -> i32 {
    let valves = parse_input(TEST_INPUT);
    println!("{:?}", valves);

    return 0;
}

const MAX_ELAPSED_MINUTES: i32 = 30;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valve {
    name: String,
    leads_to: Vec<String>,
    flow_rate: i32,
}

fn parse_input(data: &str) -> Vec<Valve> {
    let mut out: Vec<Valve> = Vec::new();
    for line in data.trim().lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let name = parts[1];
        let (_, flow_rate_str) = parts[4].split_once('=').unwrap();
        let leads_to: Vec<String> = parts[9..]
            .iter()
            .map(|p| p.trim_matches(',').to_string())
            .collect();

        let valve = Valve {
            name: name.to_string(),
            leads_to: leads_to,
            flow_rate: flow_rate_str.trim_matches(';').parse().unwrap(),
        };

        out.push(valve);
    }
    return out;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let stuff = parse_input(TEST_INPUT);

        assert_eq!(10, stuff.len());

        let valve = Valve {
            name: "AA".to_string(),
            leads_to: vec!["DD".to_string(), "II".to_string(), "BB".to_string()],
            flow_rate: 0,
        };

        assert_eq!(valve, stuff[0]);
    }
}

const TEST_INPUT: &'static str = r#"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;
