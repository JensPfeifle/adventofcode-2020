use std::collections::HashMap;

fn play(input: &[usize], end: usize) -> usize {
    let mut pp: HashMap<usize, usize> = HashMap::new();
    let mut p: HashMap<usize, usize> = HashMap::new();

    let mut turn: usize = 1;
    let mut num: usize = 0;
    for n in input.iter() {
        p.insert(*n, turn);
        num = *n;
        turn += 1;
    }

    while turn <= end {
        if p.contains_key(&num) && pp.contains_key(&num) {
            num = p.get(&num).unwrap() - pp.get(&num).unwrap();
        } else {
            num = 0;
        }
        if let Some(last_said) = p.get(&num) {
            pp.insert(num, *last_said);
        }
        p.insert(num, turn);
        turn += 1;
    }

    num
}

fn main() {
    let input = [0, 6, 1, 7, 2, 19, 20];
    println!("part1: {}", play(&input, 2020));
    println!("part2: {}", play(&input, 30_000_000));
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(play(&[0, 3, 6], 2020), 436);
        assert_eq!(play(&[1, 3, 2], 2020), 1);
        assert_eq!(play(&[2, 1, 3], 2020), 10);
        assert_eq!(play(&[1, 2, 3], 2020), 27);
        assert_eq!(play(&[2, 3, 1], 2020), 78);
        assert_eq!(play(&[3, 2, 1], 2020), 438);
        assert_eq!(play(&[3, 1, 2], 2020), 1836);
    }

    #[test]
    fn test_part2() {
        // you don't really want to run these...
        assert_eq!(play(&[0, 3, 6], 30_000_000), 175594);
        assert_eq!(play(&[1, 3, 2], 30_000_000), 2578);
        assert_eq!(play(&[2, 1, 3], 30_000_000), 3544142);
        assert_eq!(play(&[1, 2, 3], 30_000_000), 261213);
        assert_eq!(play(&[2, 3, 1], 30_000_000), 6895259);
        assert_eq!(play(&[3, 2, 1], 30_000_000), 18);
        assert_eq!(play(&[3, 1, 2], 30_000_000), 362);
    }
}
