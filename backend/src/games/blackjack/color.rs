use std::fmt;
use std::fmt::{Display, Formatter, write};

#[derive(Copy, Clone)]
pub enum Color{
    Club,
    Diamond,
    Heart,
    Spade
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let display: String;
        match self {
            Color::Club => { display = "♣".to_string() }
            Color::Diamond => {display = "♦".to_string()}
            Color::Heart => {display = "♥".to_string()}
            Color::Spade => {display = "♠".to_string()}
        }
        write!(f, "{}", display)
    }
}
