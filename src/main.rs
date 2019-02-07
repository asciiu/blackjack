use std::io;

const CARDS: [char; 14] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2', '1'];
const SUIT: [char; 4] = ['C', 'D', 'H', 'S'];

#[derive(Debug)]
struct Card {
    value: char,
    suit: char,
}

#[derive(Debug)]
struct Shoe {
     cards: Vec<Card>
}

impl Shoe {
    fn new() -> Shoe {
        let mut shoe = Shoe{ cards: Vec::new() };
        for c in CARDS.iter() {
            for s in SUIT.iter() {
                let card = Card{ value: *c, suit: *s };
                shoe.cards.push(card);
            }
        }
        shoe
    }
}

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

        let shoe = Shoe::new();
        println!("{:#?}", shoe);

        println!("balance: {}", balance);

        player_continue = play_again();
    }
}