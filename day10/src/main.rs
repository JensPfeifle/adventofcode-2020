use std::str::FromStr;

fn parse<T: FromStr>(input: Vec<&str>) -> Result<Vec<T>, T::Err> {
    input.iter().map(|line| line.parse()).collect()
}

fn find_joltages(adapters: &Vec<i64>) -> Vec<i64> {
    // expects a chain of adapters, returns joltage jumpgs between each
    let mut joltages: Vec<i64> = Vec::new();
    let mut output_joltage = 0;

    let mut adapters = adapters.iter();
    while let Some(adapter) = adapters.next() {
        joltages.push(adapter - output_joltage);
        output_joltage = *adapter;
    }
    // built-in device adapter
    joltages.push(3);

    joltages
}

fn count_1j_and_3j(joltages: &Vec<i64>) -> (usize, usize) {
    let d1j = joltages.iter().filter(|&n| *n == 1).count();
    let d3j = joltages.iter().filter(|&n| *n == 3).count();
    (d1j, d3j)
}

fn count_arrangements(adapters: &Vec<i64>) -> i64 {
    let joltages = find_joltages(adapters);
    let mut num_arrangements: i64 = 1;
    let mut num_of_continous_1s = 0;
    for j in joltages {
        match j {
            1 => num_of_continous_1s += 1,
            _ => {
                // hardcoded number of combinations for
                // each length of continous 1 jolt jumps
                // see notes
                match num_of_continous_1s {
                    0 => (),
                    1 => (),
                    2 => num_arrangements *= 2,
                    3 => num_arrangements *= 4,
                    4 => num_arrangements *= 7,
                    _ => panic!(format!(
                        "Combination count not implemented for {} continuous '1's",
                        num_of_continous_1s
                    )),
                };
                num_of_continous_1s = 0;
            }
        }
    }

    num_arrangements
}

fn main() -> std::io::Result<()> {
    let input = include_str!("../10.in").trim();
    let mut adapters = parse::<i64>(input.lines().collect()).unwrap();
    adapters.sort();
    let joltages = find_joltages(&adapters);
    let (d1j, d3j) = count_1j_and_3j(&joltages);
    println!("part1: {} ({}x 1j and {}x 3j)", d1j * d3j, d1j, d3j);
    println!("part2: {}", count_arrangements(&adapters));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = include_str!("../example.in").trim();
        let mut adapters = parse::<i64>(input.lines().collect()).unwrap();
        adapters.sort();
        let joltages = find_joltages(&adapters);
        assert_eq!(count_1j_and_3j(&joltages), (7, 5));
    }

    #[test]
    fn test_part1_example2() {
        let input = include_str!("../example2.in").trim();
        let mut adapters = parse::<i64>(input.lines().collect()).unwrap();
        adapters.sort();
        let joltages = find_joltages(&adapters);
        assert_eq!(count_1j_and_3j(&joltages), (22, 10));
    }

    #[test]
    fn test_part2_example1() {
        let input = include_str!("../example.in").trim();
        let mut joltages = parse::<i64>(input.lines().collect()).unwrap();
        joltages.sort();
        assert_eq!(count_arrangements(&joltages), 8);
    }

    #[test]
    fn test_part2_example2() {
        let input = include_str!("../example2.in").trim();
        let mut joltages = parse::<i64>(input.lines().collect()).unwrap();
        joltages.sort();
        assert_eq!(count_arrangements(&joltages), 19208);
    }
}
