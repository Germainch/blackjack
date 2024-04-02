use crate::blackjack::card::{Card, Value};
use crate::blackjack::color::Color;
use askama::Template;

#[derive(Template)]
#[template(path = "card.html")]
pub(crate) struct CardTemplate<'a> {
    pub value: &'a str,
    pub color: &'a str,
}

pub fn hand_to_templates(hand: &Vec<Card>) -> Vec<CardTemplate> {
    let mut res = Vec::new();
    let mut color;
    let mut value;

    for card in hand {
        match card.color() {
            Color::Club => color = "♣",
            Color::Diamond => color = "♦",
            Color::Heart => color = "♥",
            Color::Spade => color = "♠",
        }
        match card.value() {
            Value::Ace => value = "A",
            Value::King => value = "K",
            Value::Queen => value = "Q",
            Value::Jack => value = "J",
            Value::Ten => value = "10",
            Value::Nine => value = "9",
            Value::Eight => value = "8",
            Value::Seven => value = "7",
            Value::Six => value = "6",
            Value::Five => value = "5",
            Value::Four => value = "4",
            Value::Three => value = "3",
            Value::Two => value = "2",
        }
        res.push(CardTemplate { value, color });
    }
    res
}
