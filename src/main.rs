mod blackjack;

use blackjack::Blackjack;
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

fn read_int() -> i32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // trim trailing new line
    input = String::from(input.trim());

    match input.parse() {
        Ok(num) => num,
        Err(_) => -1 
    }
}


fn main() {
    let mut player_continue: bool = true;
    let mut balance: i32;
    let mut game = Blackjack::new();

    while player_continue  {
        println!("Enter a bankroll Grin amount (e.g. 1000)");
        balance = read_int();

        if balance < 0 {
            continue
        }

        while balance > 0 {
            let mut wagered_amount: i32 = 0;
            while wagered_amount == 0 {
                println!("Enter wager amount (balance {})", balance);
                wagered_amount = read_int();
            }

            let result = game.play_round(wagered_amount);
            balance += result;
        }

        player_continue = play_again();
    }
}