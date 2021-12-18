pub fn run() {
    let input = std::fs::read_to_string("src/day15_input.txt").unwrap();
    let ingredients = parse(&input);

    println!("Day 15A: {}", solve_the_thing(&ingredients, None));
    println!("Day 15B: {}", solve_the_thing(&ingredients, Some(500)));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn parse(input: &str) -> Vec<Ingredient> {
    let mut out = Vec::new();

    for line in input.trim().lines() {
        let chunks: Vec<&str> = line
            .trim()
            .split(&[':', ' ', ','][..])
            .filter(|s| !s.is_empty())
            .collect();
        let ingr = Ingredient {
            name: chunks[0].to_string(),
            capacity: chunks[2].parse().unwrap(),
            durability: chunks[4].parse().unwrap(),
            flavor: chunks[6].parse().unwrap(),
            texture: chunks[8].parse().unwrap(),
            calories: chunks[10].parse().unwrap(),
        };
        out.push(ingr);
    }
    return out;
}

fn solve_the_thing(ingredients: &[Ingredient], with_calories: Option<i64>) -> i64 {
    let mut best_cookie = 0;

    for a in 0..=100 {
        for b in 0..=100 {
            for c in 0..=100 {
                for d in 0..=100 {
                    if a + b + c + d != 100 {
                        continue;
                    }
                    let teaspoons = [a, b, c, d];
                    let capacity: i64 = ingredients
                        .iter()
                        .enumerate()
                        .map(|(i, ingr)| ingr.capacity * teaspoons[i])
                        .sum();
                    let durability: i64 = ingredients
                        .iter()
                        .enumerate()
                        .map(|(i, ingr)| ingr.durability * teaspoons[i])
                        .sum();
                    let flavor: i64 = ingredients
                        .iter()
                        .enumerate()
                        .map(|(i, ingr)| ingr.flavor * teaspoons[i])
                        .sum();
                    let texture: i64 = ingredients
                        .iter()
                        .enumerate()
                        .map(|(i, ingr)| ingr.texture * teaspoons[i])
                        .sum();
                    let calories: i64 = ingredients
                        .iter()
                        .enumerate()
                        .map(|(i, ingr)| ingr.calories * teaspoons[i])
                        .sum();

                    let score = [capacity, durability, flavor, texture]
                        .map(|s| std::cmp::max(0, s))
                        .iter()
                        .product();

                    let has_correct_calorie_count =
                        with_calories == None || with_calories == Some(calories);

                    if score > best_cookie && has_correct_calorie_count {
                        best_cookie = score;
                    }
                }
            }
        }
    }

    return best_cookie;
}

mod tests {
    use crate::day15::*;

    const TEST_INPUT: &'static str = r#"
    Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3        
"#;

    #[test]
    fn parse_the_thing() {
        let parsed = parse(TEST_INPUT);
        assert_eq!(2, parsed.len());

        assert_eq!(-1, parsed[0].capacity);
        assert_eq!(-2, parsed[0].durability);
        assert_eq!(6, parsed[0].flavor);
        assert_eq!(3, parsed[0].texture);
        assert_eq!(8, parsed[0].calories);
    }

    #[test]
    fn solves_correctly() {
        let parsed = parse(TEST_INPUT);
        let best = solve_the_thing(&parsed, None);
        assert_eq!(62_842_880, best);
    }
}
