use std::collections::VecDeque;

const RADIX: u32 = 10;

fn main() {
    let mut circle: VecDeque<u32> = VecDeque::new();

    let example = "389125467";
    let input = "463528179";
    circle = input.chars().map(|c| c.to_digit(RADIX).unwrap()).collect();

    println!("{:?}", circle);

    let smallest = *circle.iter().min().unwrap();
    let largest = *circle.iter().max().unwrap();
    //rotate so that current is 3 from end
    circle.rotate_left(4);

    let mut mv = 1;
    while mv <= 100 {
        println!("move {}", mv);
        let mut picked = circle.split_off(circle.len() - 3);
        println!("{:?} | {:?}", circle, picked);
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

        println!("dest {:?}", destination_label);
        // find position of destination cup
        let dest_idx = circle.iter().position(|&l| l == destination_label).unwrap();
        println!("dest_idx {:?}", dest_idx);
        // rotate so that destination is at end
        circle.rotate_left(dest_idx + 1);
        println!("{:?}", circle);

        circle.append(&mut picked);
        println!("{:?}", circle);

        // rotate so that old current cup is 4 from the end
        // which means that the new current cup is 3 from the end
        let curr_idx = circle.iter().position(|&l| l == current_label).unwrap();
        circle.rotate_left(curr_idx + 5);
        println!("{:?}", circle);
        mv += 1;
        println!("---------");
    }

    // rotate to begin with the cup labelled 1
    let idx = circle.iter().position(|&l| l == 1).unwrap();
    circle.rotate_left(idx);
    println!(
        "{:?}",
        circle
            .iter()
            .skip(1)
            .map(|i| i.to_string())
            .collect::<String>()
    );
}
