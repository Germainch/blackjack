use crate::blackjack::card::Card;
use crate::blackjack::card::Value::Ace;
use crate::blackjack::deck::Deck;
use crate::blackjack::game_state::GameResult::{Draw, Lose, Win};
use crate::blackjack::game_state::Player::{Cpu, Human};

use crate::blackjack::score::Score;

enum Player{
    Human,
    Cpu,
}

enum GameResult{
    Win,
    Lose,
    Draw,
}

pub struct Blackjack {

    deck: Deck,
    player_hand: Vec<Card>,
    cpu_hand: Vec<Card>,
    player_money: u32,
    player_bet: u32,

    is_game_over: bool,
    turn: u8,
    is_black_jack: bool,
    has_player_fold: bool,
    has_cpu_fold: bool,
    pub player_score: u32,
    pub cpu_score: u32,
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

            is_game_over: false,
            turn: 0,
            is_black_jack: false,
            has_player_fold: false,
            has_cpu_fold: false,
        }
    }

    /// plays a turn for the player when draw or fold is clicked
    pub fn player_draw(&mut self){

        self.draw_card(Human);
        self.calculate_score(Human);
        if self.player_score > 21 { self.has_player_fold = true }
        if self.check_game_over(){
            let game_result = self.check_victory();
            self.update_earnings(game_result);
        }
    }

    fn player_fold(&mut self){
        self.has_player_fold = true;
        if self.check_game_over(){
            let game_result = self.check_victory();
            self.update_earnings(game_result);
        }
    }

    /// cpu ai turn
    fn cpu_turn(&mut self){
        if self.is_game_over || self.has_cpu_fold {
            return;
        }
        match self.cpu_score {
            0..=16 => { self.draw_card( Player::Cpu ) }
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


    fn check_game_over(&mut self) -> bool{
        if self.has_player_fold {
            while !self.has_cpu_fold{
                self.cpu_turn();
            }
            true
        }
        else {
            false
        }
    }

    pub(crate) fn bet(&mut self){
        if self.player_money < 100{
            self.mafia_is_coming();
        }
        self.player_money -= 100;
        self.player_bet += 100;
    }

    fn mafia_is_coming(&self){
        println!("envoyer carte bleue");
    }

    fn update_earnings(&mut self, result: GameResult){
        match result{
            Win => {
                self.player_money += self.player_bet * 2;
            }
            Lose => {

            }
            Draw => {
                self.player_money += self.player_bet;
            }
        }
        self.player_bet = 0;
    }
    /// checks if the game is over and returns the winner
    pub fn check_victory(&self) -> GameResult {

        if self.player_score > 21{
            if self.cpu_score <= 21{
                Lose
            }
            else {
                Draw
            }
        }
        else if self.player_score == 21 {
            if self.cpu_score == 21 {
                Draw
            }
            else {
                Win
            }
        }

        else if self.player_score < 21{
            if self.cpu_score > 21{
                Win
            }
            else if self.cpu_score < self.player_score {
                Win
            }
            else if self.cpu_score == self.player_score{
                Draw
            }
            else { Lose }
        }
        else {
            Lose
        }
    }

    pub fn printstate(&self){
        println!(" ----------------- Game state turn {} -------------------- ", self.turn);
        println!("player score : {}", self.player_score);
        println!("cpu score : {}", self.cpu_score);
        println!("is game over : {}", self.is_game_over);
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
    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }
    pub fn turn(&self) -> u8 {
        self.turn
    }
    pub fn is_black_jack(&self) -> bool {
        self.is_black_jack
    }
    pub fn has_player_fold(&self) -> bool {
        self.has_player_fold
    }
    pub fn has_cpu_fold(&self) -> bool {
        self.has_cpu_fold
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