use std::collections::HashMap;
use std::collections::HashSet;

fn get_range(grid: &HashMap<(isize, isize, isize, isize), char>, dim: char) -> (isize, isize) {
    let min: isize;
    let max: isize;
    match dim {
        'x' => {
            min = *grid.keys().map(|(x, _, _, _)| x).min().unwrap();
            max = *grid.keys().map(|(x, _, _, _)| x).max().unwrap();
        }
        'y' => {
            min = *grid.keys().map(|(_, y, _, _)| y).min().unwrap();
            max = *grid.keys().map(|(_, y, _, _)| y).max().unwrap();
        }
        'z' => {
            min = *grid.keys().map(|(_, _, z, _)| z).min().unwrap();
            max = *grid.keys().map(|(_, _, z, _)| z).max().unwrap();
        }
        'w' => {
            min = *grid.keys().map(|(_, _, _, w)| w).min().unwrap();
            max = *grid.keys().map(|(_, _, _, w)| w).max().unwrap();
        }
        _ => panic!("Dimension must be 'x', 'y', or 'z'."),
    }
    (min, max)
}

// pretty-print the grid
fn pprint(grid: &HashMap<(isize, isize, isize, isize), char>) {
    let (min_z, max_z) = get_range(grid, 'z');
    let (min_y, max_y) = get_range(grid, 'y');
    let (min_x, max_x) = get_range(grid, 'x');
    let (min_w, max_w) = get_range(grid, 'w');
    for w in min_w..=max_w {
        for z in min_z..=max_z {
            println!("z={}, w={}", z, w);
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    print!("{}", grid.get(&(x, y, z, w)).unwrap());
                }
                println!();
            }
        }
    }
}

fn count_neighbors(
    grid: &HashMap<(isize, isize, isize, isize), char>,
) -> HashMap<(isize, isize, isize, isize), usize> {
    let mut neighbor_count: HashMap<(isize, isize, isize, isize), usize> = HashMap::new();
    let mut outer: HashSet<(isize, isize, isize, isize)> = HashSet::new();

    // check grid points
    for (x, y, z, w) in grid.keys() {
        let mut count = 0;
        for xp in x - 1..=x + 1 {
            for yp in y - 1..=y + 1 {
                for zp in z - 1..=z + 1 {
                    for wp in w - 1..=w + 1 {
                        if (xp, yp, zp, wp) != (*x, *y, *z, *w) {
                            if let Some(val) = grid.get(&(xp, yp, zp, wp)) {
                                if val == &'#' {
                                    count += 1;
                                }
                            } else {
                                // outer is a set of coordinates which are not in the grid HashMap,
                                // but are neighbors of points which are in the grid,
                                // and thus also need to be considered for activation
                                outer.insert((xp, yp, zp, wp));
                            }
                        }
                    }
                }
            }
        }
        neighbor_count.insert((*x, *y, *z, *w), count);
    }
    // now check the outer points
    for (x, y, z, w) in outer {
        let mut count = 0;
        for xp in x - 1..=x + 1 {
            for yp in y - 1..=y + 1 {
                for zp in z - 1..=z + 1 {
                    for wp in w - 1..=w + 1 {
                        if (xp, yp, zp, wp) != (x, y, z, w) {
                            if let Some(val) = grid.get(&(xp, yp, zp, wp)) {
                                if val == &'#' {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        neighbor_count.insert((x, y, z, w), count);
    }

    neighbor_count
}
fn count_active(grid: &HashMap<(isize, isize, isize, isize), char>) -> usize {
    grid.values().filter(|&&state| state == '#').count()
}

// calculate a step/cycle, mutating the grid in-place
fn step(
    grid: &mut HashMap<(isize, isize, isize, isize), char>,
    neighbor_count: &HashMap<(isize, isize, isize, isize), usize>,
) {
    for (x, y, z, w) in neighbor_count.keys() {
        let ncount = *neighbor_count.get(&(*x, *y, *z, *w)).unwrap();
        let current_value = *grid.get(&(*x, *y, *z, *w)).unwrap_or(&'.');
        if current_value == '#' {
            if !(ncount == 2 || ncount == 3) {
                grid.insert((*x, *y, *z, *w), '.');
            } else {
                // remains active
                // insert anyway to handle grid expansion
                grid.insert((*x, *y, *z, *w), '#');
            }
        } else {
            // inactive
            if ncount == 3 {
                grid.insert((*x, *y, *z, *w), '#');
            } else {
                // remains inactive
                // insert anyway to handle grid expansion
                grid.insert((*x, *y, *z, *w), '.');
            }
        }
    }
}

fn main() {
    // let example = vec![".#.", "..#", "###"];
    // let lines = example.iter();

    let input = include_str!("../17.in").trim();
    let lines = input.lines();

    let mut grid: HashMap<(isize, isize, isize, isize), char> = HashMap::new();
    let y_layer: Vec<Vec<char>> = lines
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    // insert input into grid
    for (yp, y_slice) in y_layer.iter().enumerate() {
        for (xp, &val) in y_slice.iter().enumerate() {
            let x = xp as isize;
            let y = yp as isize;
            grid.insert((x, y, 0, 0), val);
        }
    }

    // calculate 6 cycles
    for i in 1..=6 {
        let ncount = count_neighbors(&grid);
        step(&mut grid, &ncount);
        println!("After {} cycles:", i);
        pprint(&grid);
    }
    println!("part2: {}", count_active(&grid));
}
