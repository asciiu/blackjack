mod shoe;

use shoe::Shoe;
use std::io;


fn play_again() -> bool {
    println!("Play again? [y|n]");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim() {
        "y" => true,
        _ => false
    }
}

fn main() {
    let mut player_continue: bool = true;
    let mut shoe = Shoe::new();

    while player_continue {
        println!("Enter a bankroll Grin amount (e.g. 1000)");

        let mut roll = String::new();

        io::stdin()
            .read_line(&mut roll)
            .expect("Failed to read line");

        // trim trailing new line
        roll = String::from(roll.trim());

        let balance: i32 = match roll.parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        shoe.shuffle();
        match shoe.deal_card() {
            Some(c) => println!("{}", c.to_string()),
            _ => shoe = Shoe::new(),
        }

        println!("balance: {}", balance);

        player_continue = play_again();
    }
}