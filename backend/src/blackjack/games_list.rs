use crate::blackjack::game_state::Blackjack;
use std::collections::HashMap;
use std::iter::Map;

pub struct GameList {
    game_list: HashMap<String, Blackjack>,
}

impl GameList {
    pub fn find_game(&mut self, login: String) -> Option<&mut Blackjack> {
        self.game_list.get_mut(&login)
    }
    pub(crate) fn new() -> GameList {
        GameList {
            game_list: HashMap::new(),
        }
    }
    pub fn add_game(&mut self, login: String) {
        self.game_list.insert(login, Blackjack::new());
    }
    pub fn remove_game(&mut self, login: String) {
        self.game_list.remove(&login);
    }
}
