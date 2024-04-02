use crate::blackjack::card::Card;
use crate::blackjack::card::Value::Ace;
use crate::blackjack::deck::Deck;
use crate::blackjack::game_state::GameResult::{Draw, Lose, OnGoing, Win};
use crate::blackjack::game_state::Player::{Cpu, Human};


enum Player{
    Human,
    Cpu,
}

pub(crate) enum GameResult{
    Win,
    Lose,
    Draw,
    OnGoing,
}

pub struct Blackjack {

    deck: Deck,
    player_hand: Vec<Card>,
    cpu_hand: Vec<Card>,
    player_money: u32,
    player_bet: u32,

    has_player_fold: bool,
    has_cpu_fold: bool,
    pub player_score: u32,
    pub cpu_score: u32,
    game_result: GameResult,
}


impl Blackjack {
    pub fn new() -> Blackjack {
        let mut deck = Deck::new();
        for i in 0..=4 { deck.shuffle(); }

        Blackjack{
            player_money: 1000,
            player_bet: 0,
            player_hand: Vec::new(),
            cpu_hand: Vec::new(),
            deck,
            player_score: 0,
            cpu_score: 0,
            game_result: OnGoing,
            has_player_fold: false,
            has_cpu_fold: false,
        }
    }

    /// plays a turn for the player when draw or fold is clicked
    pub fn player_draw(&mut self){
        if self.is_game_over(){
            return;
        }

        else {
            self.draw_card(Human);
            self.calculate_score(Human);
            if self.player_score >= 21 {
                self.has_player_fold = true ;
                self.cpu_loop();
                self.check_victory();
                self.update_earnings();
            }
        }
    }

    pub fn player_fold(&mut self){
        //early return if player has already fold
        if self.has_player_fold{
            return
        }
        self.has_player_fold = true;
        self.cpu_loop();
        self.check_victory();
        self.update_earnings();
    }

    /// cpu ai turn
    fn cpu_turn(&mut self){
        if self.has_cpu_fold {
            return;
        }
        match self.cpu_score {
            0..=16 => { self.draw_card( Player::Cpu ); self.calculate_score(Cpu); }
                 _ => { self.has_cpu_fold = true }
        }
    }


    fn draw_card(&mut self, player: Player){

        let card= self.deck.draw().unwrap();

        match player {
            Player::Human => {self.player_hand.push(card)}
            Player::Cpu => {self.cpu_hand.push(card)}
        }
    }

    fn calculate_score(&mut self, player: Player){
        let mut score = 0;
        match player {
            Player::Human => {
                for card in &self.player_hand{
                    score = self.add_score(card, score);
                }
                self.player_score = score;
            }
            Player::Cpu => {
                for card in &self.cpu_hand{
                    score = self.add_score(card, score);
                }
                self.cpu_score = score;
            }
        }
    }

    fn add_score(&self, card: &Card, score: u32) -> u32{
        let mut new_score = score;
        match card.value(){
            Ace => {
                if new_score <= 10 {
                    new_score += 11;
                }
                else {
                    new_score += 1;
                }
            }
            _ => new_score +=  card.value().value_to_int() as u32,
        }

        new_score
    }


    fn cpu_loop(&mut self){
        while !self.has_cpu_fold{
            self.cpu_turn();
        }
    }

    fn is_game_over(&self) -> bool{
        return self.has_player_fold && self.has_cpu_fold
    }

    pub(crate) fn bet(&mut self){
        if self.is_game_over() {
            return
        }
        if self.player_money < 100{
            self.mafia_is_coming();
        }
        else{
            self.player_money -= 100;
            self.player_bet += 100;
        }
    }

    fn mafia_is_coming(&self){
        println!("envoyer carte bleue");
    }

    fn update_earnings(&mut self){
        match self.game_result{
            Win => {
                println!( " YOU WON !");
                self.player_money += self.player_bet * 2;
            }
            Lose => {
                println!( " YOU LOSE !");
            }
            Draw => {
                println!( " DRAW !");
                self.player_money += self.player_bet;
            }
            OnGoing => { println!( "Game is on going!");}
        }
        self.player_bet = 0;
    }
    /// checks if the game is over and returns the winner
    pub fn check_victory(&mut self){
        let mut result = OnGoing;

        match self.player_score{
            // player score in range 0-21
            0..=21 => {
                match self.cpu_score {
                    0..=21 => {
                        if self.cpu_score == self.player_score{
                            self.game_result = Draw;
                            return;
                        }
                        else if self.cpu_score < self.player_score{
                            self.game_result = Win;
                            return;
                        }
                        else {
                            self.game_result = Lose;
                            return;
                        }
                    }

                    _ =>{
                        self.game_result = Win;
                        return;
                    }
                }
            }

            // player score above 21
            _ => {
                match self.cpu_score{
                    0..=21 => {
                        self.game_result = Lose;
                        return;
                    }
                    _ => {
                        self.game_result = Draw;
                        return;
                    }
                }
            }
        }
    }

    pub fn printstate(&self){
        println!(" ----------------- Game state -------------------- ");
        println!("player score : {}", self.player_score);
        println!("cpu score : {}", self.cpu_score);
        println!("has player fold : {}", self.has_player_fold);
        println!("has cpu fold : {}", self.has_cpu_fold);
        println!(" ------------------------------------- ");
    }
    pub fn deck(&self) -> &Deck {
        &self.deck
    }
    pub fn player_hand(&self) -> &Vec<Card> {
        &self.player_hand
    }
    pub fn cpu_hand(&self) -> &Vec<Card> {
        &self.cpu_hand
    }
    pub fn player_money(&self) -> u32 {
        self.player_money
    }
    pub fn player_bet(&self) -> u32 {
        self.player_bet
    }
    pub fn has_player_fold(&self) -> bool {
        self.has_player_fold
    }
    pub fn has_cpu_fold(&self) -> bool {
        self.has_cpu_fold
    }
    pub fn game_result(&self) -> &GameResult {
        &self.game_result
    }
}
