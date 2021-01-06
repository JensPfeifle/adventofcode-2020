use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../22.in").trim();

    let (player1, player2) = input.split("\n\n").next_tuple().unwrap();

    let player1: Vec<usize> = player1
        .trim()
        .lines()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let player2: Vec<usize> = player2
        .trim()
        .lines()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    combat(&player1, &player2, 1);
}

enum Player {
    Player1,
    Player2,
}

fn combat(player1: &[usize], player2: &[usize], game: usize) -> Player {
    let mut prev_rounds: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new(); // store only hashes instead?

    let mut player1: Vec<usize> = player1.iter().cloned().collect();
    let mut player2: Vec<usize> = player2.iter().cloned().collect();
    while player1.len() > 0 && player2.len() > 0 {
        // This prevents infinite games of Recursive Combat, which everyone agrees is a bad idea.
        let is_new = prev_rounds.insert((player1.clone(), player2.clone()));
        if !is_new {
            return Player::Player1;
        }

        let p1_card = player1[0];
        let p2_card = player2[0];

        let round_winner: Player;
        if player1.len() > p1_card && player2.len() > p2_card {
            // start a new game of Recursive Combat
            let p1_deck = &player1[1..=p1_card];
            let p2_deck = &player2[1..=p2_card];
            round_winner = combat(p1_deck, p2_deck, game + 1);
        } else {
            // at least one player must not have enough cards left in their deck to recurse
            // the winner of the round is the player with the higher-value card.
            if p1_card > p2_card {
                round_winner = Player::Player1;
            } else {
                round_winner = Player::Player2;
            }
        }
        match round_winner {
            Player::Player1 => {
                player1 = [&player1[1..], &[p1_card], &[p2_card]].concat();
                player2 = player2[1..].to_vec();
            }
            Player::Player2 => {
                player1 = player1[1..].to_vec();
                player2 = [&player2[1..], &[p2_card], &[p1_card]].concat();
            }
        }
    }

    let game_winner;
    let winning_deck;
    if player1.len() > 0 {
        game_winner = Player::Player1;
        winning_deck = player1;
    } else {
        game_winner = Player::Player2;
        winning_deck = player2;
    }

    if game == 1 {
        // the instructions say "original deck", but the example and
        // correct answer use the final deck ¯\_(ツ)_/¯
        let score: usize = winning_deck
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, card)| (idx + 1) * card)
            .sum();
        println!("Winning score (Part 2): {}", score);
    }

    game_winner
}
