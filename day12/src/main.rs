fn parse(input: &str) -> Vec<(char, isize)> {
    let mut instructions: Vec<(char, isize)> = Vec::new();
    for line in input.lines() {
        let d = line.chars().nth(0).unwrap();
        let s: isize = line.chars().skip(1).collect::<String>().parse().unwrap();
        instructions.push((d, s));
    }
    instructions
}

#[derive(Debug)]
struct Coords {
    x: isize,
    y: isize,
}

fn rotate(x: isize, y: isize, dir: &char, angle: &isize) -> Coords {
    match dir {
        'R' => match angle {
            90 => return Coords { x: y, y: -x },
            180 => return Coords { x: -x, y: -y },
            270 => return Coords { x: -y, y: x },
            _ => panic!(),
        },
        'L' => match angle {
            90 => return Coords { x: -y, y: x },
            180 => return Coords { x: -x, y: -y },
            270 => return Coords { x: y, y: -x },
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn navigate(instructions: &Vec<(char, isize)>) -> Coords {
    let mut wp = Coords { x: 10, y: 1 }; // relative to ship
    let mut ship = Coords { x: 0, y: 0 };
    for (action, value) in instructions.iter() {
        match action {
            'N' => wp.y += value,
            'S' => wp.y -= value,
            'E' => wp.x += value,
            'W' => wp.x -= value,
            'L' | 'R' => wp = rotate(wp.x, wp.y, action, value),
            'F' => {
                ship.x += wp.x * value;
                ship.y += wp.y * value;
            }
            _ => panic!(),
        }
    }
    ship
}

fn main() {
    let input = include_str!("../12.in").trim();
    let instructions = parse(input);
    println!("{:?}", instructions);
    let ship_pos = navigate(&instructions);
    println!("part2: {}", ship_pos.x.abs() + ship_pos.y.abs())
}
