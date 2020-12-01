use itertools::Itertools;
use std::io::Error;

struct Entry {
    pos1: usize,
    pos2: usize,
    letter: char,
    pw: String,
}

impl Entry {
    fn validate(&self) -> bool {
        let mut is_valid: bool = false;
        let chars: Vec<char> = self.pw.chars().collect();
        if chars[self.pos1 - 1] == self.letter || chars[self.pos2 - 1] == self.letter {
            is_valid = true
        }
        if chars[self.pos1 - 1] == self.letter && chars[self.pos2 - 1] == self.letter {
            is_valid = false
        }
        is_valid
    }
}

fn parse(input: &str) -> Result<Entry, Error> {
    let (policy, pw) = input.split(": ").next_tuple().unwrap();
    let (positions, letter) = policy.split_whitespace().next_tuple().unwrap();
    let (pos1, pos2) = positions.split('-').next_tuple().unwrap();
    let policy = Entry {
        pos1: pos1.parse().unwrap(),
        pos2: pos2.parse().unwrap(),
        letter: letter.chars().next().unwrap(),
        pw: pw.to_owned(),
    };
    Ok(policy)
}

fn main() -> std::io::Result<()> {
    let input = include_str!("../2.in");
    let mut total_valid: u32 = 0;
    for line in input.lines() {
        let entry = parse(line)?;
        if entry.validate() {
            total_valid += 1
        }
    }
    println!("Part2: {} passwords are valid", total_valid);
    Ok(())
}
