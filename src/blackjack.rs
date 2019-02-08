pub mod shoe;

use shoe::Shoe;
use shoe::Card;

struct Player {
    cards: Vec<Card>
}

impl Player {
    fn new() -> Player {
        Player{ cards: Vec::new() }
    }

    fn score(&mut self) -> i32 {
        let mut amount: i32 = 0;
        for c in self.cards.iter() {
            let v = match c.text() {
                'K' | 'Q' | 'J' | 'T' => 10,
                'A' => 11,
                _ => c.text().to_string().parse::<i32>().unwrap()
            };
            amount += v;
        }

        amount 
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

            let player_score = self.player.score();
            let dealer_score = self.dealer.score();
            println!("Dealer score: {}", dealer_score);
            println!("Player score: {}", player_score);
            
            if self.player_bust() {
                break
            }
        }
    }

    pub fn player_bust(&mut self) -> bool {
        true
    }

    pub fn dealer_bust(&mut self) -> bool {
        true
    }
}