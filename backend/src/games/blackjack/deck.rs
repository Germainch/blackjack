use card::Card;
use rand::random;
use crate::games::blackjack::card;
use card::Value;
use crate::games::blackjack::color::Color;

pub(crate) struct Deck {
    cards: Vec<Card>,
}

impl Deck {

    /// Creates a new Deck of 52 cards
    pub(crate) fn new() -> Deck {
        let mut v: Vec<Card> = vec![];
        for i  in 2..=14 {
            v.push(Card::new(Value::value_by_id(i), Color::Club));
            v.push(Card::new(Value::value_by_id(i), Color::Spade));
            v.push(Card::new(Value::value_by_id(i), Color::Heart));
            v.push(Card::new(Value::value_by_id(i), Color::Diamond));
        }
        Deck {
            cards: v
        }
    }


    /// Shuffles the deck of cards using the Fisher-Yates algorithm
    pub(crate) fn shuffle(&mut self){
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

    /// Draw a card from the deck
    /// Pops the last card from the deck and returns it if the deck contains a card
    pub(crate) fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }



    fn print(&self){
        for i in 0..52 {
            self.cards[i].print();
        }
    }
}

#[test]
fn test_new_deck_and_shuffle(){
    let mut d: Deck = Deck::new();
    for i in 0..3{
        d.shuffle();
    }
    d.print();
}

#[test]
fn test_draw(){
    let mut deck: Deck = Deck::new();
    for i in 0..deck.cards.len() + 3 {
        let card = deck.draw();
        match card {
            None => { println!("deck vide")}
            Some(c) => {c.print()}
        }
        println!("number of cards remaining {}", deck.cards.len())
    }
}