use std::collections::HashMap;
use std::collections::HashSet;

// parse a line into a set of ingredients and allergens
fn parse(line: &str) -> (HashSet<&str>, HashSet<&str>) {
    let c_idx = line.find("(contains ").unwrap();
    let (ingredients, allergens) = line.split_at(c_idx);
    let food_ingredients: HashSet<&str> = ingredients.split_whitespace().collect();
    let food_allergens: HashSet<&str> = allergens
        .strip_prefix("(contains")
        .unwrap()
        .strip_suffix(")")
        .unwrap()
        .split(",")
        .map(|s| s.trim())
        .collect();

    (food_ingredients, food_allergens)
}

fn main() {
    let input = include_str!("../21.in").trim();

    // Map allergen to a set of ingredients
    let mut allergen_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let (food_ingredients, food_allergens) = parse(line);
        // An ingredient can only have an allergen if the ingredient appears
        // in all meals in which the allergen appears.
        for allergen in food_allergens.iter() {
            let ingredients = allergen_map
                .entry(allergen)
                .or_insert(food_ingredients.clone());
            ingredients.retain(|i| food_ingredients.contains(i));
        }
    }
    // Now we have a map of each allergen to possible ingredients.
    // Next, find an allergen with only one possible ingredient,
    // and prune that ingredent from all others. Repeat until all mappings are 1-1.
    while allergen_map.values().filter(|ingr| ingr.len() > 1).count() > 0 {
        for (allergen, ingredients) in allergen_map.clone() {
            if ingredients.len() == 1 {
                // Matched. Remove the ingredient from all other allergens.
                for (other_allergen, other_ingredients) in allergen_map.iter_mut() {
                    if other_allergen != &allergen {
                        other_ingredients.retain(|i| !ingredients.contains(i));
                    }
                }
            }
        }
    }

    println!("{:?}", allergen_map);

    // part1
    let allergen_ingredients: HashSet<&str> = allergen_map.values().flatten().cloned().collect();
    let mut count = 0;
    for line in input.lines() {
        let (ingredients, _) = parse(line);
        count += ingredients
            .into_iter()
            .filter(|ingr| !(allergen_ingredients.contains(ingr)))
            .count();
    }
    println!("part1: {}", count);

    // part2
    let mut bar: Vec<(&str, &str)> = allergen_map
        .iter()
        .map(|(&allergen, ingredients)| (allergen, *ingredients.into_iter().next().unwrap()))
        .collect();
    bar.sort_by(|a, b| (a.0).cmp(b.0));
    let part2 = bar.iter().map(|(_, i)| *i).collect::<Vec<&str>>().join(",");

    println!("part2: {}", part2);
}
