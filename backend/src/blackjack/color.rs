use std::fmt;
use std::fmt::{write, Display, Formatter};

#[derive(Copy, Clone)]
pub enum Color {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let display: String = match self {
            Color::Club =>  "♣".to_string(),
            Color::Diamond =>  "♦".to_string(),
            Color::Heart =>  "♥".to_string(),
            Color::Spade =>  "♠".to_string(),
        };
        write!(f, "{}", display)
    }
}
