use crate::blackjack::game_state::Blackjack;

struct Game{
    blackjack: Blackjack,
    login: String,
}
struct GameList{
    game_list: Vec<(Game)>
}

