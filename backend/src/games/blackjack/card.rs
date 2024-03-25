use color::Color;
use crate::games::blackjack::color;

#[derive(Copy, Clone)]
pub struct Card {
     value : u8,
     color : Color
}


impl Card{
    pub fn new(value: u8, color: Color) -> Self {
        Card{value, color}
    }

    fn compare_value(&self, other: &Card) {
        todo!()
    }

    pub(crate) fn print(&self){
        println!("{}; {}", self.value, self.color)
    }
}