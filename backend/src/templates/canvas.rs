use crate::blackjack::game_state::Blackjack;
use crate::blackjack::game_state::GameResult;

use crate::templates::card::{hand_to_templates, CardTemplate};
use askama::Template;

#[derive(Template)]
#[template(path = "canvas.html")]
struct CanvasTemplate<'a> {
    player_score: u32,
    cpu_score: u32,
    player_hand: &'a Vec<CardTemplate<'a>>,
    cpu_hand: &'a Vec<CardTemplate<'a>>,
    bet: u32,
    bank: u32,
    game_result: &'a GameResult,
}

pub fn canvas_to_string(game: &Blackjack) -> String {
    let canvas = CanvasTemplate {
        player_score: game.player_score,
        cpu_score: game.cpu_score,
        player_hand: &hand_to_templates(game.player_hand()),
        cpu_hand: &hand_to_templates(game.cpu_hand()),
        bet: game.player_bet(),
        bank: game.player_money(),
        game_result: game.game_result(),
    };
    canvas.render().unwrap()
}
