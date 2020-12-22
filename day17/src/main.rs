use std::collections::HashMap;
use std::collections::HashSet;

fn get_range(grid: &HashMap<(isize, isize, isize), char>, dim: char) -> (isize, isize) {
    let min: isize;
    let max: isize;

    match dim {
        'x' => {
            min = *grid.keys().map(|(x, _, _)| x).min().unwrap();
            max = *grid.keys().map(|(x, _, _)| x).max().unwrap();
        }
        'y' => {
            min = *grid.keys().map(|(_, y, _)| y).min().unwrap();
            max = *grid.keys().map(|(_, y, _)| y).max().unwrap();
        }
        'z' => {
            min = *grid.keys().map(|(_, _, z)| z).min().unwrap();
            max = *grid.keys().map(|(_, _, z)| z).max().unwrap();
        }
        _ => panic!("Dimension must be 'x', 'y', or 'z'."),
    }
    (min, max)
}

fn pprint(grid: &HashMap<(isize, isize, isize), char>) {
    let (min_z, max_z) = get_range(grid, 'z');
    let (min_y, max_y) = get_range(grid, 'y');
    let (min_x, max_x) = get_range(grid, 'x');
    for z in min_z..=max_z {
        println!("z={:?}", z);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                print!("{}", grid.get(&(x, y, z)).unwrap());
            }
            println!();
        }
    }
}

fn count_neighbors(
    grid: &HashMap<(isize, isize, isize), char>,
) -> HashMap<(isize, isize, isize), usize> {
    let mut neighbor_count: HashMap<(isize, isize, isize), usize> = HashMap::new();
    let mut outer: HashSet<(isize, isize, isize)> = HashSet::new(); // coordinates of outer ring, which is not in grid
    println!("inner");
    for (x, y, z) in grid.keys() {
        let mut count = 0;
        for xp in x - 1..=x + 1 {
            for yp in y - 1..=y + 1 {
                for zp in z - 1..=z + 1 {
                    if (xp, yp, zp) != (*x, *y, *z) {
                        if let Some(val) = grid.get(&(xp, yp, zp)) {
                            if val == &'#' {
                                count += 1;
                            }
                        } else {
                            // point is not on grid, but next to one that is,
                            // we need to check its neighbors as well
                            outer.insert((xp, yp, zp));
                        }
                    }
                }
            }
        }
        neighbor_count.insert((*x, *y, *z), count);
    }
    println!("outer");
    // outer
    for (x, y, z) in outer {
        let mut count = 0;
        for xp in x - 1..=x + 1 {
            for yp in y - 1..=y + 1 {
                for zp in z - 1..=z + 1 {
                    if (xp, yp, zp) != (x, y, z) {
                        if let Some(val) = grid.get(&(xp, yp, zp)) {
                            if val == &'#' {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        neighbor_count.insert((x, y, z), count);
    }

    neighbor_count
}
fn count_active(grid: &HashMap<(isize, isize, isize), char>) -> usize {
    grid.values().filter(|&&state| state == '#').count()
}

fn step(
    grid: &mut HashMap<(isize, isize, isize), char>,
    neighbor_count: &HashMap<(isize, isize, isize), usize>,
) {
    for (x, y, z) in neighbor_count.keys() {
        let ncount = *neighbor_count.get(&(*x, *y, *z)).unwrap();
        let current_value = *grid.get(&(*x, *y, *z)).unwrap_or(&'.');
        if current_value == '#' {
            if !(ncount == 2 || ncount == 3) {
                grid.insert((*x, *y, *z), '.');
            } else {
                // else remains active, insert anyways to handle grid expansion
                grid.insert((*x, *y, *z), '#');
            }
        } else {
            // inactive
            if ncount == 3 {
                grid.insert((*x, *y, *z), '#');
            } else {
                // else remains inactive,insert anyways to handle grid expansion
                grid.insert((*x, *y, *z), '.');
            }
        }
    }
}

fn main() {
    let example = vec![".#.", "..#", "###"];
    let input = include_str!("../17.in").trim();
    // let lines = example.iter();
    let lines = input.lines();

    let mut grid: HashMap<(isize, isize, isize), char> = HashMap::new();
    let y_layer: Vec<Vec<char>> = lines
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    for (yp, y_slice) in y_layer.iter().enumerate() {
        for (xp, &val) in y_slice.iter().enumerate() {
            let x = xp as isize;
            let y = yp as isize;
            grid.insert((x, y, 0), val);
        }
    }

    for i in 1..=6 {
        let ncount = count_neighbors(&grid);
        step(&mut grid, &ncount);
        println!("After {} cycles:", i);
        pprint(&grid);
    }

    println!("part1: {}", count_active(&grid));
}
