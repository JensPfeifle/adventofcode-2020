use itertools::Itertools;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("../22.in").trim();

    let (player1, player2) = input.split("\n\n").next_tuple().unwrap();

    let mut player1: VecDeque<usize> = player1
        .trim()
        .lines()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mut player2: VecDeque<usize> = player2
        .trim()
        .lines()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut round = 0;
    while player1.len() > 0 && player2.len() > 0 {
        println!("--- Round {} ---", round);
        round += 1;
        println!("Player 1's deck: {:?}", player1);
        println!("Player 2's deck: {:?}", player2);
        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();
        println!("Player 1 plays: {:?}", p1);
        println!("Player 2 plays: {:?}", p2);

        if p1 > p2 {
            println!("Player 1 wins the round!");
            player1.push_back(p1);
            player1.push_back(p2);
        } else {
            println!("Player 2 wins the round!");
            player2.push_back(p2);
            player2.push_back(p1);
        }
    }
    let winner;
    if player1.len() > 0 {
        println!("Player 1 wins!");
        winner = player1;
    } else {
        println!("Player 2 wins!");
        winner = player2;
    }

    let score: usize = winner
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, card)| (idx + 1) * card)
        .sum();
    println!("Part 1: {}", score);
}
