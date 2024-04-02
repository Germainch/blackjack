use askama::Template;
use crate::blackjack::game_state::Blackjack;
use crate::blackjack::card::Card;
use crate::templates::card::{CardTemplate, hand_to_templates};

#[derive(Template)]
#[template(path = "canvas.html")]
#[macro_use]
struct CanvasTemplate<'a> {
    player_score: u32,
    cpu_score: u32,
    player_hand: &'a Vec<CardTemplate<'a>>,
    cpu_hand: &'a Vec<CardTemplate<'a>>,
    bet : u32,
    bank: u32,
}


pub fn canvas_to_string_1(game: &Blackjack) -> String {
    let canvas = CanvasTemplate{
        player_score: 0,
        cpu_score: 0,
        player_hand: &vec![],
        cpu_hand: &vec![],
        bet: 0,
        bank: 0,
    };
    return canvas.render().unwrap();
}

pub fn canvas_to_string(game: &Blackjack) -> String {
    let canvas = CanvasTemplate{
        player_score: game.player_score,
        cpu_score: game.cpu_score,
        player_hand: &hand_to_templates(game.player_hand()),
        cpu_hand: &hand_to_templates(game.cpu_hand()),
        bet: game.player_bet(),
        bank: game.player_money(),
    };
    return canvas.render().unwrap();
}