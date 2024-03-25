use card::Card;
use rand::random;
use crate::games::blackjack::card;
use crate::games::blackjack::color::Color;

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let mut v: Vec<Card> = vec![];
        for i in 1..=13 {
            v.push( Card::new(i, Color::Spade));
            v.push( Card::new(i, Color::Heart));
            v.push( Card::new(i, Color::Diamond));
            v.push( Card::new(i, Color::Club));
        }
        Deck {
            cards: v
        }
    }

    /// Shuffles the deck of cards using the Fisher-Yates algorithm
    ///
    fn shuffle(&mut self){
        let mut j: u8;
        let mut tmp: Card;
        for i in 0 .. 51 {
            j = random::<u8>().clamp(i, 51);

            // swapping cards[i] with cards[j]
            tmp = self.cards[i as usize];
            self.cards[i as usize] = self.cards[j as usize];
            self.cards[j as usize] = tmp;
        }
    }

    fn print(&self){
        for i in 0..52 {
            self.cards[i].print();
        }
    }
}

#[test]
fn testdeck(){
    let mut d: Deck = Deck::new();
    d.shuffle();
    d.print();
}