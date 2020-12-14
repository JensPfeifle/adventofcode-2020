use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../14.in").trim();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let re_assignment = Regex::new(r"^mem\[(\d*)\] = (\d*)$").unwrap();
    let re_mask = Regex::new(r"^mask = ([01X]*)$").unwrap();
    let mut mask: Vec<char> = Vec::new();
    let mut memory: HashMap<usize, usize> = HashMap::new();
    for line in input.lines() {
        if re_assignment.is_match(line) {
            let caps = re_assignment.captures(line).unwrap();
            let loc: usize = caps[1].parse().unwrap();
            let val: usize = caps[2].parse().unwrap();
            let newval = apply_mask_to_value(&mask, val);
            memory.insert(loc, newval);
        } else if re_mask.is_match(line) {
            let caps = re_mask.captures(line).unwrap();
            mask = caps[1].chars().collect()
        } else {
            panic!("Could not parse input!");
        }
    }
    println!("part1: {}", memory.values().sum::<usize>());
}

fn apply_mask_to_value(mask: &Vec<char>, val: usize) -> usize {
    let mut bval: Vec<char> = format!("{:0>36b}", val).chars().collect();
    for n in 0..mask.len() {
        if mask[n] != 'X' {
            bval[n] = mask[n].clone();
        }
    }
    let bstr: String = bval.iter().collect();
    let new_val = usize::from_str_radix(&bstr, 2).unwrap();
    new_val
}

fn part2(input: &str) {
    let re_assignment = Regex::new(r"^mem\[(\d*)\] = (\d*)$").unwrap();
    let re_mask = Regex::new(r"^mask = ([01X]*)$").unwrap();
    let mut mask: Vec<char> = Vec::new();
    let mut memory: HashMap<usize, usize> = HashMap::new();
    for line in input.lines() {
        if re_assignment.is_match(line) {
            let caps = re_assignment.captures(line).unwrap();
            let loc: usize = caps[1].parse().unwrap();
            let val: usize = caps[2].parse().unwrap();
            write_part2(&mut memory, &mask, loc, val);
        } else if re_mask.is_match(line) {
            let caps = re_mask.captures(line).unwrap();
            mask = caps[1].chars().collect()
        } else {
            panic!("Could not parse input!");
        }
    }
    println!("part2: {}", memory.values().sum::<usize>());
}

fn write_part2(memory: &mut HashMap<usize, usize>, mask: &Vec<char>, loc: usize, val: usize) {
    let mut bval: Vec<char> = format!("{:0>36b}", loc).chars().collect();
    let mut floating: Vec<usize> = Vec::new(); // indices of floating values
    for n in 0..mask.len() {
        match mask[n] {
            '0' => (), // unchanged
            '1' => bval[n] = '1',
            'X' => {
                bval[n] = 'X';
                floating.push(n);
            }
            _ => panic!(),
        }
    }

    // replace 'X' bits with all possible combinations of 0/1
    let num_combinations = 1 << floating.len();
    for c in 0..num_combinations {
        let bits: Vec<char> = format!("{:0>36b}", c).chars().rev().collect();
        for (n, idx) in floating.iter().enumerate() {
            bval[*idx] = bits[n]
        }
        let bstr: String = bval.iter().collect();
        let loc = usize::from_str_radix(&bstr, 2).unwrap();
        // aaand insert into each location
        memory.insert(loc, val);
    }
}
