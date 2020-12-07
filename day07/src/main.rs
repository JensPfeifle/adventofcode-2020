use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

fn parse(rule: &str) -> (&str, Vec<(usize, &str)>) {
    let (outer, contents) = rule.split("contain").next_tuple().unwrap();
    let mut outer = outer.trim().rsplitn(2, " "); // split into "bags" + style
    outer.next(); // throw out 'bags'
    let parsed_outer = outer.next().unwrap();

    let mut parsed_contents = Vec::new();
    if contents.trim() != "no other bags." {
        for inner_bags in contents.split(",") {
            // split each item into "bags" + number and style
            let mut splitted = inner_bags.trim().rsplitn(2, " ");
            splitted.next(); // throw out 'bags'
            let number_and_style = splitted.next().unwrap();
            let (number, style) = number_and_style.splitn(2, " ").next_tuple().unwrap();
            parsed_contents.push((number.parse::<usize>().unwrap(), style));
        }
    }
    (parsed_outer, parsed_contents)
}
fn reverse_rules<'a>(
    rules: &'a HashMap<&str, Vec<(usize, &str)>>,
) -> HashMap<&'a str, Vec<&'a str>> {
    // get map of bag style -> possible parents
    let mut parents: HashMap<&str, Vec<&str>> = HashMap::new();
    for (outer, contents) in rules.iter() {
        for (_num, style) in contents.iter() {
            if let Some(pars) = parents.get_mut(style) {
                pars.push(outer.clone());
            } else {
                parents.insert(style, vec![outer]);
            }
        }
    }
    parents
}

fn part1(rules: &HashMap<&str, Vec<(usize, &str)>>) -> usize {
    let reversed_rules = reverse_rules(rules);
    let mut queue: Vec<&str> = reversed_rules.get("shiny gold").unwrap().to_vec();
    let mut results: HashSet<&str> = HashSet::new();
    while queue.len() > 0 {
        let style = queue.pop().unwrap();
        if let Some(parents) = reversed_rules.get(&style) {
            queue.extend(parents.clone());
        }
        results.insert(style);
    }

    results.len()
}

fn part2(rules: &HashMap<&str, Vec<(usize, &str)>>) -> usize {
    let mut queue: Vec<(usize, &str)> = rules.get("shiny gold").unwrap().to_vec();
    let mut total: usize = 0;
    while queue.len() > 0 {
        let (num, style) = queue.pop().unwrap();
        if let Some(parents) = rules.get(&style) {
            queue.extend(parents.repeat(num));
        }
        total += num;
    }

    total
}

fn main() -> std::io::Result<()> {
    let input = include_str!("../7.in").trim();
    let mut rules: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();
    for rule in input.lines().map(parse) {
        let (outer, contents) = rule;
        rules.insert(outer, contents);
    }
    println!("part1: {}", part1(&rules));
    println!("part2: {}", part2(&rules));
    Ok(())
}
