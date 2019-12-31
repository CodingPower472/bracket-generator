
mod bracket_generator;

use bracket_generator::{Team, Match, fill_byes, x_elim};

fn main() {
    //let player_names = ["a", "b", "c", "d"].iter().map(|a| a.to_string()).collect();
    let list = (1..=12).map(|num| num.to_string()).collect::<Vec<String>>();
    let (dblelim, finalists, match_num) = x_elim(list, 2, true);
    for (i, round) in dblelim.iter().enumerate() {
        println!("Round {}: ", i + 1);
        for (j, bracket) in round.iter().enumerate() {
            println!("  Bracket {}: ", j + 1);
            for m in bracket {
                println!("      {}", m);
            }
        }
    }
    println!();
}
