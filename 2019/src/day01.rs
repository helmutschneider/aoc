use crate::util::*;

type Vec<T> = heapless::Vec<T, 128>;

pub const DAY_01: Day<i32> = Day {
    day: 1,
    parts: &[part1, part2],
    tests: &[test1, test2],
};

fn parse_input() -> Vec<i32> {
    return INPUT
        .trim()
        .lines()
        .map(|line| {
            return line.parse::<i32>().unwrap();
        })
        .collect();
}

fn get_fuel_for_mass(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;
    return max(0, fuel);
}

fn get_fuel_for_mass_rec(mass: i32) -> i32 {
    let mut total_fuel = 0;
    let mut next = mass;

    loop {
        let fuel = get_fuel_for_mass(next);
        if fuel == 0 {
            break;
        }
        total_fuel += fuel;
        next = fuel;
    }
    return total_fuel;
}

fn part1() -> i32 {
    let values = parse_input();
    let sum = values
        .iter()
        .fold(0, |carry, x| carry + get_fuel_for_mass(*x));

    return sum;
}

fn part2() -> i32 {
    let values = parse_input();
    let sum = values
        .iter()
        .fold(0, |carry, x| carry + get_fuel_for_mass_rec(*x));

    return sum;
}

fn test1() {
    assert_eq!(654, get_fuel_for_mass(1969));
    assert_eq!(33583, get_fuel_for_mass(100756));
}

fn test2() {
    assert_eq!(966, get_fuel_for_mass_rec(1969));
    assert_eq!(50346, get_fuel_for_mass_rec(100756));
}

const INPUT: &'static str = r#"
76663
111378
132647
115688
67473
85562
62955
64052
104961
128687
60344
81158
129984
106462
55967
130004
140810
71523
64891
142922
122783
123918
116246
120842
105578
122950
107512
70051
55347
54348
89301
95258
122323
136781
137756
95658
91017
79626
98414
79296
75226
143850
131334
107028
76591
75492
66400
51904
79262
68956
98957
52481
87955
118871
148734
103699
68681
55118
144120
59403
115012
147742
124218
73580
114949
65346
113104
129059
119068
72339
74984
53095
127452
133786
111439
98153
96312
139641
88907
136831
73574
67871
57641
134505
72116
134503
134387
88598
78687
61020
107234
64801
132668
60204
90001
87833
131148
61488
107938
116072
"#;
