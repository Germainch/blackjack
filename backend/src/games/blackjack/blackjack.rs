use crate::games::blackjack::deck::Deck;
use crate::games::blackjack::player::Player;


struct Blackjack {
    player: Player,
    cpu: Player,
    deck: Deck,
}


impl Blackjack {
    fn new(player_username: String) -> Blackjack {
        Blackjack{
            player: Player::new(player_username),
            cpu: Player::new("cpu".to_string()),
            deck: Deck::new(),
        }
    }

    fn run(&mut self){

    }
}