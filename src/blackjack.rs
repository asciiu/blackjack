pub mod shoe;

use shoe::Shoe;
use shoe::Card;
use std::io;

enum PlayerInput {
    Double,
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

fn read_input(card_num: usize) -> PlayerInput {
    if card_num == 2 {
        println!("\n(h)it (s)tand (d)ouble: ");
    } else {
        println!("\n(h)it (s)tand: ");
    }

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("");

    // trim trailing new line
    input = String::from(input.trim());

    match input.trim() {
        "d" => PlayerInput::Double,
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

    pub fn play_round(&mut self, mut wager: i32) -> i32 {
        let dealer_card: Card = self.deal_card(); 
        self.dealer.cards.push(dealer_card);

        while self.player.cards.len() < 2 {
            let card: Card = self.deal_card(); 
            self.player.cards.push(card);
        }

        let mut player_score = self.player.score();
        let mut dealer_score = self.dealer.score();
        let mut stand = false;

        self.player.print_hand("Player");
        self.dealer.print_hand("Dealer");

        while player_score < 21 && !stand {
            let input = read_input(self.player.cards.len());
            match input {
                PlayerInput::Double => {
                    wager += wager;
                    let player_card: Card = self.deal_card(); 
                    self.player.cards.push(player_card);

                    self.player.print_hand("Player");
                    self.dealer.print_hand("Dealer");

                    player_score = self.player.score();
                    stand = true;
                }
                PlayerInput::Hit => {
                    let player_card: Card = self.deal_card(); 
                    self.player.cards.push(player_card);

                    self.player.print_hand("Player");
                    self.dealer.print_hand("Dealer");

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
        self.player.cards.clear();
        self.dealer.cards.clear();

        if player_score > dealer_score && player_score <= 21 {
            println!("Player (win!)\n");
            wager
        } else if player_score < dealer_score && dealer_score <= 21 {
            println!("Player (lose)\n");
            -wager
        } else if player_score > 21 && dealer_score <= 21 {
            println!("Player (lose)\n");
            -wager
        } else if dealer_score > 21 && player_score <= 21 {
            println!("Player (win!)\n");
            wager
        } else if dealer_score == player_score {
            println!("push\n");
            0 
        } else {
            println!("push\n");
            0 
        }
    }
}