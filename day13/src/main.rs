fn find_earliest_departure(me_arrival: usize, busses: Vec<usize>) -> (usize, usize) {
    // returns (bus_id, departure_time)
    let mut t = me_arrival;
    loop {
        for bus_id in busses.iter() {
            if t % bus_id == 0 {
                return (*bus_id, t);
            }
        }
        t += 1;
    }
}

fn parse(input: &str) -> (usize, Vec<&str>) {
    let input: Vec<&str> = input.trim().lines().collect();
    let arrival: usize = input[0].parse().unwrap();
    let busses: Vec<&str> = input[1].split(',').collect();
    (arrival, busses)
}

fn drop_unkown(busses: &Vec<&str>) -> Vec<usize> {
    let mut bus_ids: Vec<usize> = Vec::new();
    for entry in busses.iter() {
        let id = entry.parse().unwrap_or(0);
        if id != 0 {
            bus_ids.push(id);
        }
    }
    bus_ids
}

fn part2_naive(schedule: Vec<&str>, t0: usize) -> usize {
    // don't do this... ;)
    let bus_ids = drop_unkown(&schedule);

    let mut time_deltas: Vec<usize> = Vec::new();
    for (dt, entry) in schedule.iter().enumerate() {
        if entry != &"x" {
            time_deltas.push(dt);
        }
    }

    let mut t = t0;
    'clock: loop {
        t += 1;
        if t % 100000000 == 0 {
            println!("t={}", t);
        }
        for (i, bus_id) in bus_ids.iter().enumerate() {
            if (t + time_deltas[i]) % bus_id != 0 {
                continue 'clock;
            }
        }
        break 'clock;
    }

    t
}

fn part2(schedule: Vec<&str>, t0: usize) -> usize {
    // calculate time offsets for each bus
    // e.g. '2,x,x,5,x,10' becomes [0,3,5]
    let mut offsets: Vec<usize> = Vec::new();
    for (dt, entry) in schedule.iter().enumerate() {
        if entry != &"x" {
            offsets.push(dt);
        }
    }

    // drop 'x' from schedule and convert to usize
    let bus_ids = drop_unkown(&schedule);

    // kudos to https://www.reddit.com/r/adventofcode/comments/kc4njx/2020_day_13_solutions/gfnwnf3/
    let mut t = bus_ids[0];
    for i in 0..bus_ids.len() {
        let factor: usize = bus_ids.iter().take(i).product();
        let mut n = 0;
        while ((t + factor * n) + offsets[i]) % bus_ids[i] != 0 {
            n += 1;
        }
        t = t + factor * n;
    }
    t
}

fn main() {
    let input = include_str!("../13.in").trim();
    let (arrival, schedule) = parse(input);
    println!("{}, {:?}", arrival, schedule);

    let busses = drop_unkown(&schedule);
    let (bus_id, departure) = find_earliest_departure(arrival, busses);
    let wait_time = departure - arrival;
    println!("part1: {}", bus_id * wait_time);
    println!("part2: {}", part2(schedule, 100000000000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let arrival = 939;
        let busses = vec![7, 13, 59, 31, 19];
        let (bus_id, departure) = find_earliest_departure(arrival, busses);
        assert_eq!(bus_id, 59);
        assert_eq!(departure, 944);
    }

    #[test]
    fn test_part2() {
        let input = "0\n7,13,x,x,59,x,31,19\n";
        let (_, busses) = parse(input);
        assert_eq!(part2(busses, 1060000), 1068781);

        let input = "0\n17,x,13,19\n";
        let (_, busses) = parse(input);
        assert_eq!(part2(busses, 0), 3417);

        let input = "0\n67,7,59,61\n";
        let (_, busses) = parse(input);
        assert_eq!(part2(busses, 750000), 754018);

        let input = "0\n67,x,7,59,61\n";
        let (_, busses) = parse(input);
        assert_eq!(part2(busses, 779000), 779210);

        let input = "0\n67,7,x,59,61\n";
        let (_, busses) = parse(input);
        assert_eq!(part2(busses, 1261000), 1261476);

        let input = "0\n1789,37,47,1889\n";
        let (_, busses) = parse(input);
        assert_eq!(part2(busses, 1202161480), 1202161486);
    }
}
