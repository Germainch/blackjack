use color::Color;
use crate::blackjack::card::Value::{Ace, Eight, Five, Four, Jack, King, Nine, Queen, Seven, Six, Ten, Three, Two};
use crate::blackjack::color;
use crate::blackjack::color::Color::Club;

#[derive(Copy, Clone)]
pub struct Card {
     value : Value,
     color : Color
}

#[derive(Debug,Copy,Clone)]
pub(crate) enum Value{
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

impl Value{
    /// returns the value of a variant, the Ace can be 1 or 11 depending on the state of the player's hand
    /// the default value of the Ace is 11, it can be changed by the game that computes the player's score
    pub fn value_to_int(&self) -> u8 {
        match self {
            Value::Ace => { 11 }
            Value::King => { 10 }
            Value::Queen => { 10 }
            Value::Jack => { 10 }
            Value::Ten => { 10 }
            Value::Nine => { 9 }
            Value::Eight => { 8 }
            Value::Seven => { 7 }
            Value::Six => { 6 }
            Value::Five => { 5 }
            Value::Four => { 4 }
            Value::Three => { 3 }
            Value::Two => { 2 }
        }
    }

    /// returns a variant depending on an id given in parameters
    /// id is between 2 and 14 with ace being 14.
    pub fn value_by_id(id: u8) -> Value {

        assert!(id >= 2 && id <= 14);

        match id{
            2 => {Two},
            3 => {Three},
            4 => {Four},
            5 => {Five},
            6 => {Six},
            7 => {Seven},
            8 => {Eight},
            9 => {Nine},
            10 => {Ten},
            11 => {Jack},
            12 => {Queen},
            13 => {King},
            14 => {Ace},

            // no need for option because we assert that id is correct
            // this case will never be used
            _  => {Ace},
        }
    }
}


impl Card{
    pub fn new(value: Value, color: Color) -> Self {
        Card{value, color}
    }

    fn compare_value(&self, other: &Card) {
        todo!()
    }

    pub fn print(&self){
        println!("{:?}; {}", self.value, self.color)
    }
    pub fn value(&self) -> Value {
        self.value
    }
    pub fn color(&self) -> Color {
        self.color
    }
}

#[test]
fn test_card_creation(){
    let c = Card::new(Value::Ace, Color::Club);
    c.print();
}

#[test]
fn test_value_by_id(){
    let v: Value = Value::value_by_id(3);
    let c: Card  = Card::new(v, Club);
    c.print();
}