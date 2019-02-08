mod blackjack;

use blackjack::shoe::Shoe;
use blackjack::shoe::Card;
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
    let mut shoe = Shoe::new();
    shoe.shuffle();
    let mut balance: i32;

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

            println!("Wager amount: {}", wagered_amount);

            let card: Card = match shoe.deal_card() {
                Some(c) => c,
                _ => {
                    shoe = Shoe::new();
                    shoe.shuffle();
                    shoe.deal_card().unwrap()
                }
            };

            println!("Dealer: {}", card);


            balance -= wagered_amount;
        }

        player_continue = play_again();
    }
}