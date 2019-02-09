pub mod shoe;

use shoe::Shoe;
use shoe::Card;
use std::io;

enum PlayerInput {
    Hit,
    Stand,
    Split,
    Unknown,
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

    fn print_hand(&mut self, label: &str) {
        print!("{}: ", label);
        for c in self.cards.iter() {
            print!("{} ", c);
        }
        print!("[{}]\n", self.score());
    }
}

fn read_input() -> PlayerInput {
    println!("\n(h)it (s)tand: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("");

    // trim trailing new line
    input = String::from(input.trim());

    match input.trim() {
        "h" => PlayerInput::Hit,
        "s" => PlayerInput::Stand,
        "p" => PlayerInput::Split,
        _ => PlayerInput::Unknown,
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
        let player_card: Card = self.deal_card(); 
        let dealer_card: Card = self.deal_card(); 
        println!("Dealer: {}", dealer_card);
        println!("Player: {}", player_card);

        self.dealer.cards.push(dealer_card);
        self.player.cards.push(player_card);

        let mut player_score = self.player.score();
        let mut dealer_score = self.dealer.score();
        let mut stand = false;

        while player_score < 21 && !stand {
            let input = read_input();
            match input {
                PlayerInput::Hit => {
                    let player_card: Card = self.deal_card(); 
                    self.player.cards.push(player_card);
                    self.player.print_hand("Player");
                    player_score = self.player.score();
                }

                PlayerInput::Stand => {
                    stand = true;
                    self.player.print_hand("Player");
                }
                _ => ()
            }
        }

        while dealer_score < 17 {
            let dealer_card: Card = self.deal_card(); 
            self.dealer.cards.push(dealer_card);
            dealer_score = self.dealer.score();
        }
        
        self.dealer.print_hand("Dealer");

        //println!("Dealer score: {}", dealer_score);

        if player_score > dealer_score && player_score <= 21 {
            println!("Player (win!)");
        } else if player_score < dealer_score && dealer_score <= 21 {
            println!("Player (lose)");
        } else if player_score > 21 && dealer_score <= 21 {
            println!("Player (bust)");
        } else if dealer_score > 21 && player_score <= 21 {
            println!("Dealer (bust)");
        } else if dealer_score == player_score {
            println!("push");
        } else if dealer_score > 21 && player_score > 21 {
            println!("push");
        }
        println!("");
            
        self.player.cards.clear();
        self.dealer.cards.clear();
    }
}