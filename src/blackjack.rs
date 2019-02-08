pub mod shoe;

use shoe::Shoe;

struct Blackjack {
    shoe: Shoe,
}

impl Blackjack {
    fn new() -> Blackjack {
        Blackjack{shoe: Shoe::new()}
    }
}