use std::str::FromStr;

fn parse<T: FromStr>(input: Vec<&str>) -> Result<Vec<T>, T::Err> {
    input.iter().map(|word| word.parse()).collect()
}

fn part1(numbers: &Vec<i64>, preamble_len: usize) -> i64 {
    for n in preamble_len..numbers.len() {
        let mut valid = false;
        let start = n - preamble_len;
        for other in numbers[start..n].iter() {
            if numbers[start..n].contains(&(numbers[n] - other)) {
                valid = true;
                break;
            }
        }
        if !valid {
            return numbers[n];
        }
    }
    panic!("Could not find an invalid number!")
}
fn part2(numbers: &Vec<i64>, target: i64) -> i64 {
    for i in 1..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[i..j].iter().sum::<i64>() == target {
                let max = numbers[i..j].iter().max().unwrap();
                let min = numbers[i..j].iter().min().unwrap();
                return max + min;
            }
        }
    }
    panic!("Could not find a matching set of numbers!")
}
fn main() {
    let input = include_str!("../9.in")
        .trim()
        .lines()
        .collect::<Vec<&str>>();
    let preamble_len: usize = 25;
    let numbers = parse::<i64>(input).unwrap();
    let part1 = part1(&numbers, preamble_len);
    println!("part1: {}", part1);
    println!("part2: {}", part2(&numbers, part1));
}
