use crate::games::blackjack::deck::Deck;
use crate::games::blackjack::player::Player;


struct Blackjack {
    player: Player,
    cpu: Player,
    deck: Deck,
    is_game_over: bool,
    turn: u8,

}


impl Blackjack {
    fn new(player_username: String) -> Blackjack {
        Blackjack{
            player: Player::new(player_username),
            cpu: Player::new("cpu".to_string()),
            deck: Deck::new(),
            is_game_over: false,
            turn: 0,
        }
    }

    fn run(&mut self){
        let mut turn: u8 = 0;

        while !self.is_game_over {
            turn += 1;
            /// waits until the player has chosen on either draw a card or pass turn
            if self.player.is_turn_over(){
                /// plays the cpu turn
                self.cpu.bot_turn()
            }
            self.is_game_over = self.check_winning_conditions();
        }
    }
}