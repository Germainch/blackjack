
use crate::blackjack::deck::Deck;
use crate::blackjack::player::Player;
use crate::blackjack::score::Score;


pub struct Blackjack {
    player: Player,
    cpu: Player,
    deck: Deck,
    is_game_over: bool,
    turn: u8,
    is_black_jack: bool,
    has_player_fold: bool,
    has_cpu_fold: bool,
}


impl Blackjack {
    pub fn new(player_username: &str) -> Blackjack {
        let mut deck = Deck::new();
        for i in 0..=4 { deck.shuffle(); }
        Blackjack{
            player: Player::new(player_username),
            cpu: Player::new("cpu"),
            deck,
            is_game_over: false,
            turn: 0,
            is_black_jack: false,
            has_player_fold: false,
            has_cpu_fold: false,
        }
    }

    /// plays a turn for the player when draw or fold is clicked
    pub fn turn (&mut self){
        if self.is_game_over { self.check_victory(); return; }
        self.turn += 1;
        self.player_turn();
        self.cpu_turn();
        self.check_victory();
    }

    /// cpu ai turn
    fn cpu_turn(&mut self){
        if self.is_game_over || self.has_cpu_fold{
            return;
        }
        match self.cpu.score(){
            0..=16 => {self.cpu.draw_card(&mut self.deck)}
                 _ => {self.has_cpu_fold = true}
        }
    }

    /// player turn depending on which of draw or fold he chose
     fn player_turn(&mut self) {
        if self.is_game_over || self.has_player_fold {
            return;
        }
        self.player.draw_card(&mut self.deck);
        match self.player.check_score(){
            Score::Over => {self.is_game_over = true}
            Score::Under => {}
            Score::Blackjack => {self.is_game_over = true; self.is_black_jack = true}
        }
    }

    /// checks if the game is over and returns the winner
    pub fn check_victory(&self){
        if !self.is_game_over{
            return
        }
        if self.player.score() > self.cpu.score() && self.player.score() <= 21{
            println!("player has won");
        }
        else { println!("cpu has won"); }
    }

    pub fn printstate(&self){
        println!(" ----------------- Game state turn {} -------------------- ", self.turn);
        println!("player score : {}", self.player.score());
        println!("cpu score : {}", self.cpu.score());
        println!("is game over : {}", self.is_game_over);
        println!("has player fold : {}", self.has_player_fold);
        println!("has cpu fold : {}", self.has_cpu_fold);
        println!(" ------------------------------------- ");
    }
}

#[test]
fn test_game(){
    let mut game = Blackjack::new("Bob");
    for i in 0..4{
        game.turn();
    }
    game.printstate();
}