use std::collections::HashSet;

fn find_row(seat: &str) -> u64 {
    let mut range_start: f64 = 0.0;
    let mut range_end: f64 = 127.0;
    for c in seat.chars().take(7) {
        let half_range = ((range_end - range_start) / 2.0).ceil();
        match c {
            'F' => range_end -= half_range,
            'B' => range_start += half_range,
            _ => panic!("Unkown command, expected 'F' or 'B'"),
        }
    }
    if range_start != range_end {
        panic!("Rows do not reduce to single row!")
    }
    range_start as u64
}

fn find_col(seat: &str) -> u64 {
    let mut range_start: f64 = 0.0;
    let mut range_end: f64 = 7.0;
    for c in seat.chars().skip(7) {
        let half_range = ((range_end - range_start) / 2.0).ceil();
        match c {
            'L' => range_end -= half_range,
            'R' => range_start += half_range,
            _ => panic!("Unkown command, expected 'L' or 'R'"),
        }
    }
    if range_start != range_end {
        panic!("Cols do not reduce to single column!")
    }
    range_start as u64
}

fn find_id(loc: (u64, u64)) -> u64 {
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
    let rows = input.lines().map(find_row);
    let cols = input.lines().map(find_col);
    let locations = rows.zip(cols);
    let ids = locations.map(find_id).collect::<Vec<u64>>();
    println!("Part1: largest ID is {}", ids.iter().max().unwrap());
    println!("Part2: seat ID is {:?}", find_seat(&ids));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_row() {
        assert_eq!(find_row("FBFBBFFRLR"), 44);
        assert_eq!(find_row("BFFFBBFRRR"), 70);
        assert_eq!(find_row("FFFBBBFRRR"), 14);
        assert_eq!(find_row("BBFFBBFRLL"), 102);
    }

    #[test]
    fn test_find_col() {
        assert_eq!(find_col("FBFBBFFRLR"), 5);
        assert_eq!(find_col("BFFFBBFRRR"), 7);
        assert_eq!(find_col("FFFBBBFRRR"), 7);
        assert_eq!(find_col("BBFFBBFRLL"), 4);
    }
}
