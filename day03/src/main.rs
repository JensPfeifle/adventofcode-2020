fn count_trees(map: &str, slope: &(usize, usize)) -> u64 {
    let mut trees_seen: u64 = 0;
    let mut x: usize = 0;
    let mut y: usize = 0;
    let (dx, dy) = slope;
    let rows: Vec<&str> = map.trim().lines().collect::<Vec<&str>>();
    let width = rows[0].len();
    while y < rows.len() {
        let tile: char = rows[y].chars().collect::<Vec<char>>()[x];
        if tile == '#' {
            trees_seen += 1;
        }
        x = (x + dx) % width;
        y = y + dy;
    }

    trees_seen
}

fn main() -> std::io::Result<()> {
    let input = include_str!("../3.in");
    let closure_ftw = |slope| count_trees(input, slope);
    // Part 1
    let slopes: Vec<(usize, usize)> = vec![(3, 1)];
    let total = slopes.iter().map(closure_ftw).product::<u64>();
    println!("Part1: {}", total);
    // Part 2
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let total = slopes.iter().map(closure_ftw).product::<u64>();
    println!("Part2: {}", total);
    Ok(())
}
