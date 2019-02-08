pub mod shoe;

use shoe::Shoe;
use shoe::Card;
use std::io;

enum PlayerInput {
    Hit,
    Stand,
    //Split
}

struct Player {
    cards: Vec<Card>
}

impl Player {
    fn new() -> Player {
        Player{ cards: Vec::new() }
    }

    fn score(&mut self) -> i32 {
        let mut total = 0;

        for c in self.cards.iter() {
            let v = match c.text() {
                'K' | 'Q' | 'J' | 'T' => 10,
                'A' => 0,
                _ => c.text().to_string().parse::<i32>().unwrap()
            };
            total += v;
        }

        // add up all aces
        for c in self.cards.iter() {
            if c.text() == 'A' {
                total += 11;
                
                if total > 31 {
                    total -= 20;
                } else if total > 21 {
                    total -= 10;
                }
            }
        }

        total 
    }

    fn print_hand(&mut self) {
        print!("Player: ");
        for c in self.cards.iter() {
            print!("{} ", c);
        }
        print!("[{}]\n", self.score());
    }
}

fn read_input() -> PlayerInput {
    println!("(h)it (s)tand");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // trim trailing new line
    input = String::from(input.trim());

    match input.trim() {
        "h" => PlayerInput::Hit,
        _ => PlayerInput::Stand,
    }
}

pub struct Blackjack {
    shoe: Shoe,
    player: Player,
    dealer: Player,
}

impl Blackjack {
    pub fn new() -> Blackjack {
        let mut s = Shoe::new();
        s.shuffle();

        Blackjack{
            shoe: s, 
            player: Player::new(),
            dealer: Player::new()
        }
    }

    pub fn deal_card(&mut self) -> Card {
        match self.shoe.deal_card() {
            Some(c) => c,
            _ => {
                let mut shoe = Shoe::new();
                shoe.shuffle();
                self.shoe = shoe;
                self.shoe.deal_card().unwrap()
            }
        }
    }

    pub fn run(&mut self) {
        loop {

            let player_card: Card = self.deal_card(); 
            let dealer_card: Card = self.deal_card(); 
            println!("Dealer: {}", dealer_card);
            println!("Player: {}", player_card);

            self.dealer.cards.push(dealer_card);
            self.player.cards.push(player_card);

            let mut player_score = self.player.score();
            let dealer_score = self.dealer.score();
            let mut stand = false;

            while player_score < 21 || stand {
                let input = read_input();
                match input {
                    PlayerInput::Hit => {
                        let player_card: Card = self.deal_card(); 
                        self.player.cards.push(player_card);
                        self.player.print_hand();
                        player_score = self.player.score();
                    }
                    _ => stand = true
                }
            }

            println!("Dealer score: {}", dealer_score);
            println!("Player score: {}", player_score);
            
            if player_score > 21 || dealer_score > 21 {
                break
            }
        }

        self.player.cards.clear();
        self.dealer.cards.clear();
    }
}