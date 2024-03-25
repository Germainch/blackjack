use crate::games::blackjack;
use crate::games::blackjack::card::Card;
use crate::games::blackjack::deck::Deck;
struct Player {
    name: String,
    cards: Vec<Card>,
    score: u8
}

impl Player {
    fn new(name: String) -> Player {
        Player{
            name,
            cards: vec![],
            score: 0,
        }
    }

    fn draw_card(&mut self, deck : &mut Deck){
        match deck.draw(){
            None => {println!("plus de carte dans le paquet")}
            Some(c) => {
                self.cards.push(c);
            }
        }
    }
    /// Adds the value of the drawn card to the player's score
    /// if the card is an ace, it can be an 11 or a 1 depending on what is the best for the player
    fn add_score(card: &Card){
    }
}