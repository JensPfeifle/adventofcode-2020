use std::fs::File;
use std::io::prelude::*;

// could also uuse include_str! to load?
// let input = include_str!("i.in").split("\n");


fn main() -> std::io::Result<()> {
    let mut file = File::open("1.in")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let inpt: Vec<u32> = contents.lines().map(|x| x.parse::<u32>().unwrap()).collect();

    for i in 0..inpt.len() {
        for j in 0..i {
            for k in 0..j {
                if inpt[i] + inpt[j] + inpt[k] == 2020 {
                    println!("{:#?}", inpt[i] * inpt[j] * inpt[k]);
                }
            }
        }
    }
    Ok(())
}
