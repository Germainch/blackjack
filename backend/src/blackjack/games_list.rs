use std::collections::HashMap;
use std::iter::Map;
use crate::blackjack::game_state::Blackjack;

pub struct GameList{
    game_list: HashMap<String, Blackjack>
}

impl GameList {
    pub fn find_game(&mut self, login: String) -> Option<&mut Blackjack>{
        self.game_list.get_mut(&login)
    }
    pub(crate) fn new() -> GameList{
        GameList{
            game_list: HashMap::new(),
        }
    }
    pub fn add_game(&mut self, login: String){
        self.game_list.insert(login, Blackjack::new());
    }
}

