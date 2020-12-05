use std::collections::HashSet;

fn find_location(seat: &str) -> (u64, u64) {
    let mut min_r: f64 = 0.0;
    let mut max_r: f64 = 127.0;
    let mut min_c: f64 = 0.0;
    let mut max_c: f64 = 7.0;
    for c in seat.chars() {
        let half_r = ((max_r - min_r) / 2.0).ceil();
        let half_c = ((max_c - min_c) / 2.0).ceil();
        match c {
            'F' => max_r -= half_r,
            'B' => min_r += half_r,
            'L' => max_c -= half_c,
            'R' => min_c += half_c,
            _ => panic!("Unkown command!"),
        }
    }
    if min_r != max_r {
        panic!("Rows do not reduce to single row!")
    }
    if min_c != max_c {
        panic!("Cols do not reduce to single column!")
    }

    (min_r as u64, min_c as u64)
}

fn calculate_id(loc: (u64, u64)) -> u64 {
    // loc = (row, col)
    loc.0 * 8 + loc.1
}

fn find_seat(ids: &Vec<u64>) -> u64 {
    let smallest = *ids.iter().min().unwrap();
    let largest = *ids.iter().max().unwrap();
    let found_ids: HashSet<u64> = ids.iter().cloned().collect();
    let all_ids: HashSet<u64> = (smallest..=largest).collect();
    *all_ids.difference(&found_ids).next().unwrap()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("../5.in").trim();
    let locations = input.lines().map(find_location);
    let ids = locations.map(calculate_id).collect::<Vec<u64>>();
    println!("Part1: largest ID is {}", ids.iter().max().unwrap());
    println!("Part2: seat ID is {:?}", find_seat(&ids));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_location() {
        assert_eq!(find_location("FBFBBFFRLR"), (44, 5));
        assert_eq!(find_location("BFFFBBFRRR"), (70, 7));
        assert_eq!(find_location("FFFBBBFRRR"), (14, 7));
        assert_eq!(find_location("BBFFBBFRLL"), (102, 4));
    }
}
