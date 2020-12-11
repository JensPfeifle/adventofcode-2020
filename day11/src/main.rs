use std::convert::TryInto;
use std::fmt::Display;

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut parsed = Vec::new();
    for line in input.lines() {
        parsed.push(line.chars().collect());
    }
    parsed
}

fn pprint<T>(grid: &Vec<Vec<T>>)
where
    T: Display,
{
    for line in grid.iter() {
        for ch in line {
            print!("{}", ch)
        }
        println!("")
    }
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_neighbors(seats: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut neighbors = Vec::new();
    for (y, row) in seats.iter().enumerate() {
        let mut row_neighbors = Vec::new();
        for (x, seat) in row.iter().enumerate() {
            if *seat == 'L' || *seat == '#' {
                let mut neighbor_count = 0;
                // over each of 8 directions
                'directions: for (h, v) in DIRECTIONS.iter() {
                    // start small, increasing radius only if needed
                    let mut radius: isize = 1;
                    loop {
                        let new_x: isize = x as isize + (h * radius);
                        let new_y: isize = y as isize + (v * radius);
                        if new_x >= 0 && new_y >= 0 {
                            let new_x = new_x.try_into().unwrap();
                            let new_y = new_y.try_into().unwrap();
                            if (new_x, new_y) != (x, y) && new_x < row.len() && new_y < seats.len()
                            {
                                let s = seats[new_y][new_x];
                                if s != '.' {
                                    if s == '#' {
                                        neighbor_count += 1;
                                    }
                                    continue 'directions;
                                }
                            }
                        }
                        radius += 1;
                        if radius > row.len() as isize && radius > seats.len() as isize {
                            break;
                        }
                    }
                }
                row_neighbors.push(neighbor_count);
            } else {
                row_neighbors.push(0);
            }
        }
        neighbors.push(row_neighbors);
    }
    neighbors
}

fn step(seats: &mut Vec<Vec<char>>, neighbors: &Vec<Vec<usize>>) -> bool {
    let mut changed = false;
    for y in 0..seats.len() {
        for x in 0..seats[y].len() {
            if seats[y][x] == 'L' && neighbors[y][x] == 0 {
                seats[y][x] = '#';
                changed = true;
            } else if seats[y][x] == '#' && neighbors[y][x] >= 5 {
                seats[y][x] = 'L';
                changed = true;
            }
        }
    }
    print!("\x1B[2J"); // control character to clear terminal screen
    pprint(seats);
    changed
}

fn main() {
    // part 2 only
    let input = include_str!("../11.in").trim();
    let mut seats = parse(input);
    loop {
        let neighbor_counts = count_neighbors(&seats);
        let changed = step(&mut seats, &neighbor_counts);
        if !changed {
            break;
        }
    }
    println!(
        "part2: {}",
        seats.into_iter().flatten().filter(|s| *s == '#').count()
    );
}
