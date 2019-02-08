use rand::Rng;
use std::fmt;

const CARDS: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const SUIT: [char; 4] = ['C', 'D', 'H', 'S'];

#[derive(Debug)]
pub struct Card {
    text: char,
    suit: char,
}

impl Card {
    pub fn new(text: char, suit: char) -> Card {
        Card{ text: text, suit: suit }
    }
}

impl fmt::Display for Card {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.text, self.suit)
    }
}


#[derive(Debug)]
pub struct Shoe {
     cards: Vec<Card>
}

impl Shoe {
    pub fn new() -> Shoe {
        let mut shoe = Shoe{ cards: Vec::new() };
        for c in CARDS.iter() {
            for s in SUIT.iter() {
                let card = Card::new(*c, *s) ;
                shoe.cards.push(card);
            }
        }
        shoe
    }

    pub fn shuffle(&mut self) {
        let mut i = 0;
        while i  <  self.cards.len() {
            let rand = rand::thread_rng().gen_range(0, self.cards.len());
            let card = self.cards.swap_remove(rand);
            self.cards.push(card);
            i+=1;
        }
    }

    pub fn deal_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}