use crate::blackjack::card::Card;
use crate::blackjack::card::Value::Ace;
use crate::blackjack::deck::Deck;
use crate::blackjack::game_state::GameResult::{Draw, Lose, OnGoing, Win};
use crate::blackjack::game_state::Player::{Cpu, Human};
use std::cmp::Ordering;

enum Player {
    Human,
    Cpu,
}

pub(crate) enum GameResult {
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

#[allow(unused)]
impl Blackjack {
    /// constructor for a new game of blackjack
    pub fn new(player_money: u32) -> Blackjack {
        let mut deck = Deck::new();
        for _i in 0..=4 {
            deck.shuffle();
        }

        Blackjack {
            player_money,
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

    /// plays a turn for the player when he draws a card
    pub fn player_draw(&mut self) {
        if !self.is_game_over() {
            self.draw_card(Human);
            self.calculate_score(Human);

            self.cpu_turn();

            if self.player_score >= 21 {
                self.has_player_fold = true;
                self.cpu_loop();
                self.check_victory();
                self.update_earnings();
            }
        }
    }

    /// performs the fold action for the player then plays the AI turns and end game functions
    pub fn player_fold(&mut self) {
        //early return if player has already fold
        if self.has_player_fold {
            return;
        }
        self.has_player_fold = true;
        self.cpu_loop();
        self.check_victory();
        self.update_earnings();
    }

    /// cpu ai turn
    fn cpu_turn(&mut self) {
        if self.has_cpu_fold {
            return;
        }
        match self.cpu_score {
            0..=16 => {
                self.draw_card(Cpu);
                self.calculate_score(Cpu);
            }
            _ => self.has_cpu_fold = true,
        }
    }

    /// draws a card from the deck into a player's hand
    fn draw_card(&mut self, player: Player) {
        let card = self.deck.draw().unwrap();

        match player {
            Human => self.player_hand.push(card),
            Cpu => self.cpu_hand.push(card),
        }
    }

    /// takes a player's hand and calculates its score
    fn calculate_score(&mut self, player: Player) {
        let mut score = 0;
        match player {
            Human => {
                for card in &self.player_hand {
                    score = self.add_score(card, score);
                }
                self.player_score = score;
            }
            Cpu => {
                for card in &self.cpu_hand {
                    score = self.add_score(card, score);
                }
                self.cpu_score = score;
            }
        }
    }

    /// adds the score of a card to a score
    fn add_score(&self, card: &Card, score: u32) -> u32 {
        let mut new_score = score;
        match card.value() {
            Ace => {
                if new_score <= 10 {
                    new_score += 11;
                } else {
                    new_score += 1;
                }
            }
            _ => new_score += card.value().value_to_int() as u32,
        }

        new_score
    }

    /// plays the bot turns after the player has finished playing
    fn cpu_loop(&mut self) {
        while !self.has_cpu_fold {
            self.cpu_turn();
        }
    }

    /// checks if the game is over
    fn is_game_over(&self) -> bool {
        self.has_player_fold && self.has_cpu_fold
    }

    /// increase the player's bet by 100
    pub(crate) fn bet(&mut self) {
        if self.is_game_over() {
            return;
        }
        if self.player_money < 100 {
            println!("not enough money to bet")
        } else {
            self.player_money -= 100;
            self.player_bet += 100;
        }
    }

    /// updates the bank and the bet of the player
    fn update_earnings(&mut self) {
        match self.game_result {
            Win => {
                println!(" YOU WON !");
                self.player_money += self.player_bet * 2;
            }
            Lose => {
                println!(" YOU LOSE !");
            }
            Draw => {
                println!(" DRAW !");
                self.player_money += self.player_bet;
            }
            OnGoing => {
                println!("Game is on going!");
            }
        }
        self.player_bet = 0;
    }

    /// checks if the game is over and sets the winner
    pub fn check_victory(&mut self) {
        match self.player_score {
            // player score in range 0-21
            0..=21 => match self.cpu_score {
                0..=21 => match self.cpu_score.cmp(&self.player_score) {
                    Ordering::Less => self.game_result = Win,
                    Ordering::Equal => self.game_result = Draw,
                    Ordering::Greater => self.game_result = Lose,
                },

                _ => {
                    self.game_result = Win;
                }
            },

            // player score above 21
            _ => match self.cpu_score {
                0..=21 => {
                    self.game_result = Lose;
                }
                _ => {
                    self.game_result = Draw;
                }
            },
        }
    }

    /// prints the state of the game object
    fn print_state(&self) {
        println!(" ----------------- Game state -------------------- ");
        println!("player score : {}", self.player_score);
        println!("cpu score : {}", self.cpu_score);
        println!("has player fold : {}", self.has_player_fold());
        println!("has cpu fold : {}", self.has_cpu_fold());
        println!("deck size : {}", self.deck().cards().len());
        println!(" ------------------------------------- ");
    }

    // ***************** GETTERS ***************************
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

// ********************************* TESTING *********************************
#[test]
fn test_construction() {
    let game = Blackjack::new();
    game.print_state();
}
