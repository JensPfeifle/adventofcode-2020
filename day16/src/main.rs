use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../16.in").trim();
    let (rules, my_ticket, nearby_tickets) = input.split("\n\n").next_tuple().unwrap();
    let rules = rules.lines().collect::<Vec<&str>>();
    let nearby_tickets = nearby_tickets.lines().skip(1).collect::<Vec<&str>>();
    let my_ticket = my_ticket.lines().skip(1).next().unwrap();

    part1(&rules, &nearby_tickets);
    part2(&rules, &nearby_tickets, &my_ticket);
}

fn part1(rules: &Vec<&str>, nearby_tickets: &Vec<&str>) {
    let rules: Vec<(&str, Vec<(usize, usize)>)> = rules.iter().map(|r| parse_rule(r)).collect();
    let mut total: usize = 0;
    for ticket in nearby_tickets.iter() {
        for value in ticket.split(",") {
            // FIXME: move this out one loop?
            let val: usize = value.parse().unwrap();
            let mut ticket_is_valid = false;
            // ticket is invalid if any field does not match any of the rules
            for rule in rules.iter() {
                if is_valid(&val, &rule.1) {
                    ticket_is_valid = true;
                }
            }
            if !ticket_is_valid {
                total += val;
            }
        }
    }
    println!("part1: {}", total);
}

/// Validate a value against some ranges
fn is_valid(value: &usize, ranges: &Vec<(usize, usize)>) -> bool {
    let mut is_valid = false;
    for (a, b) in ranges.iter() {
        if a <= value && value <= b {
            is_valid = true;
        }
    }
    is_valid
}

fn parse_rule(rule: &str) -> (&str, Vec<(usize, usize)>) {
    let (field_name, raw_ranges) = rule.split(": ").next_tuple().unwrap();

    let mut ranges: Vec<(usize, usize)> = Vec::new();

    for range in raw_ranges.split(" or ") {
        let (a, b) = range.split("-").next_tuple().unwrap();
        let a: usize = a.parse().unwrap();
        let b: usize = b.parse().unwrap();
        ranges.push((a, b));
    }

    (field_name, ranges)
}

fn part2(rules: &Vec<&str>, nearby_tickets: &Vec<&str>, my_ticket: &str) {
    println!("{} tickets scanned", nearby_tickets.len());
    let valid_tickets = filter_invalid_tickets(&nearby_tickets, &rules);
    println!("{} tickets are valid", valid_tickets.len());

    let number_of_fields = my_ticket.split(",").count();

    let mut valid_rules: Vec<HashSet<&str>> = (0..number_of_fields)
        .into_iter()
        .map(|n| get_valid_rules(n, &valid_tickets, rules))
        .collect();

    // map rules to field indexes:
    // begin where only one rule matches,
    // thus we have 1-to-1 mapping,
    // then remove the rule from all the others,
    // at which point another field will only have one rule
    while valid_rules.iter().filter(|rules| rules.len() > 1).count() > 0 {
        for field_idx in 0..valid_rules.len() {
            if valid_rules[field_idx].len() == 1 {
                for inner in 0..valid_rules.len() {
                    if inner != field_idx {
                        let rule_name = valid_rules[field_idx].clone().into_iter().next().unwrap();
                        valid_rules[inner].remove(rule_name);
                    }
                }
            }
        }
    }

    //
    // functional hell incoming...
    //

    // unpack final rule from each HashSet
    let field_rules: Vec<&str> = valid_rules
        .iter()
        .map(|set| *set.into_iter().next().unwrap())
        .collect();

    // find field indexes which correspond to rules that start with 'departure'
    let departure_values_indexes: Vec<usize> = field_rules
        .iter()
        .enumerate()
        .filter(|(_, &rule_name)| rule_name.starts_with("departure"))
        .map(|(idx, _)| idx)
        .collect();

    // parse ticket str
    let my_ticket_vals: Vec<usize> = my_ticket
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    // calculate solution
    let solution: usize = departure_values_indexes
        .iter()
        .map(|&idx| my_ticket_vals[idx])
        .product();

    println!("part2: {:?}", solution);

    //
    // ... or just do like this -> same answer ;)
    //
    let mut solution: usize = 1;
    for (index, &rule_name) in field_rules.iter().enumerate() {
        if rule_name.starts_with("departure") {
            solution *= my_ticket
                .split(",")
                .nth(index)
                .unwrap()
                .parse::<usize>()
                .unwrap()
        }
    }

    println!("part2: {:?}", solution)
}

// take a Vec of tickets and filter out those which are invalid according to the given rules
fn filter_invalid_tickets<'a>(tickets: &Vec<&'a str>, rules: &Vec<&str>) -> Vec<&'a str> {
    let rules: Vec<(&str, Vec<(usize, usize)>)> = rules.iter().map(|r| parse_rule(r)).collect();
    let mut valid_tickets: Vec<&str> = Vec::new();
    {
        for ticket in tickets.iter() {
            let mut ticket_values = ticket.split(",").map(|v| v.parse::<usize>().unwrap());
            // ticket is invalid if any field value does not match any of the rules
            let is_invalid =
                ticket_values.any(|val| !rules.iter().any(|rule| is_valid(&val, &rule.1)));
            if !is_invalid {
                valid_tickets.push(ticket);
            }
        }
    }
    valid_tickets
}

// return a set of names of rules which are valid for the nth field of all of the given tickets
fn get_valid_rules<'a>(
    field_idx: usize,
    tickets: &Vec<&str>,
    rules: &Vec<&'a str>,
) -> HashSet<&'a str> {
    let rules: Vec<(&str, Vec<(usize, usize)>)> = rules.iter().map(|r| parse_rule(r)).collect();
    let mut matching_rules: HashSet<&str> = HashSet::new();
    for rule in rules.iter() {
        // check if rule matches field n for all valid tickets
        let mut all_tickets_match = true;
        for ticket in tickets.iter() {
            let v = ticket.split(",").nth(field_idx).unwrap();
            if !is_valid(&(v.parse::<usize>().unwrap()), &rule.1) {
                all_tickets_match = false;
            }
        }
        if all_tickets_match {
            matching_rules.insert(rule.0);
        }
    }
    matching_rules
}
