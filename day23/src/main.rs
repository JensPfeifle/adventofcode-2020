use std::collections::VecDeque;

const RADIX: u32 = 10;

fn main() {
    let mut circle: VecDeque<u32> = VecDeque::new();

    let input = "389125467"; // example
                             // let input = "463528179"; // my input
    circle = input.chars().map(|c| c.to_digit(RADIX).unwrap()).collect();
    println!("{:?}", circle);
    let smallest = *circle.iter().min().unwrap();
    let largest = *circle.iter().max().unwrap();

    let mut fill = (smallest..=1_000_000).collect();
    circle.append(&mut fill);

    //rotate so that current is 3 from end
    circle.rotate_left(4);

    let mut mv = 1;
    while mv <= 10_000_000 {
        println!("move {}", mv);
        let mut picked = circle.split_off(circle.len() - 3);
        // current is at end

        let current_label = circle[circle.len() - 1];
        let mut destination_label = current_label - 1;
        if destination_label < smallest {
            destination_label = largest;
        }
        while picked.contains(&destination_label) {
            destination_label = destination_label - 1;
            if destination_label < smallest {
                destination_label = largest;
            }
        }

        // find position of destination cup
        let dest_idx = circle.iter().position(|&l| l == destination_label).unwrap();
        // rotate so that destination is at end
        circle.rotate_left(dest_idx + 1);
        circle.append(&mut picked);

        // rotate so that old current cup is 4 from the end
        // which means that the new current cup is 3 from the end
        let curr_idx = circle.iter().position(|&l| l == current_label).unwrap();
        circle.rotate_left(curr_idx + 5);
        mv += 1;
    }

    // rotate to begin with the cup labelled 1
    let idx = circle.iter().position(|&l| l == 1).unwrap();
    circle.rotate_left(idx);
    println!("{:?}", circle.iter().skip(1).take(2).collect::<Vec<&u32>>());
    println!("{:?}", circle.iter().skip(1).take(2).product::<u32>());
}
