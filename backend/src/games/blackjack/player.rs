use crate::games::blackjack;
use crate::games::blackjack::card::{Card, Value};
use crate::games::blackjack::card::Value::Ace;
use crate::games::blackjack::color::Color;
use crate::games::blackjack::color::Color::{Club, Spade};
use crate::games::blackjack::deck::Deck;
pub(crate) struct Player {
    name: String,
    cards: Vec<Card>,
    score: u8,
    money: u32,
}

impl Player {
    pub(crate) fn new(name: String) -> Player {
        Player{
            name,
            cards: vec![],
            score: 0,
            money: 1000,
        }
    }

    /// subtracts the money the player bets from his wallet if he has that much
    fn bet(&mut self, amount: u8){
        if(amount >= self.money){
            println!("you don't have that kind of money! ");
            todo!()
        }
        else{
            self.money -= amount;
        }
    }

    fn draw_card(&mut self, deck : &mut Deck){
        match deck.draw(){
            None => {println!("plus de carte dans le paquet")}
            Some(c) => {
                self.cards.push(c);
                self.add_score(&c);
            }
        }
    }
    /// Adds the value of the drawn card to the player's score
    /// if the card is an ace, it can be an 11 or a 1 depending on what is the best for the player
    fn add_score(&mut self, card: &Card){
        match card.value(){
            Ace => {
                if self.score <= 10 {
                    self.score += 11;
                }
                else {
                    self.score += 1;
                }
            }
            _ => self.score += card.value().value_to_int(),
        }
    }
}

#[test]
fn test_draw(){
    let mut player: Player = Player::new("Janno".to_string());
    let mut deck: Deck = Deck::new();
    deck.shuffle();
    player.draw_card(&mut deck);

    for card in player.cards{
        card.print();
        println!("score : {}", player.score)
    }
}
#[test]
fn test_add_score_2_ace(){
    let mut player: Player = Player::new("Janno".to_string());
    player.add_score(&Card::new(Ace,Spade));
    println!("{}", player.score);
    player.add_score(&Card::new(Ace,Club));
    println!("{}", player.score);
}