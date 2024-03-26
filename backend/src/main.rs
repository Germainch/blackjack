use std::fs::{File, read_to_string};
use std::io::Read;
use std::ops::Add;
use askama::{Html, Template};
use rouille::{Request, router};
use rouille::Response;
use crate::games::blackjack::blackjack::Blackjack;

mod games;
mod templates;
mod db;

fn concat_html_css(html: &str, css: &str) -> String{
    let mut concat:String = String::new();
    concat = html.to_owned() + "<style>" + css + "</style>";
    concat
}

fn main() {
    let mut blackjack: Blackjack = Blackjack::new(db::accounts::FRANCIS);

    rouille::start_server("localhost:8000", move |request| {
        router!(request,
            (GET) (/) => {

                // combine css and html files to serve

                let mut html_stringified: String = read_to_string("../site/dist/index.html").unwrap();

                // TODO : récupérer le fichier sans passer par le nom ou alors trouver un moyen de servir le dossier du front
                let mut css_stringified: String = read_to_string("../site/dist/assets/index-BtvWOTnS.css").unwrap();

                let html_css = concat_html_css(html_stringified.as_str(), css_stringified.as_str());

                rouille::Response::html(html_css)
            },

            (POST) (/games/blackjack/run) => {
                rouille::Response::text("cpartie")
            },
            (POST) (/games/blackjack/draw) => {
                blackjack.turn();
                rouille::Response::html("<p>allors comme ça</p>")

            },
            (GET) (/hello) => {
                rouille::Response::text(templates::hello::template_to_string())
            },
            _ => rouille::Response::empty_404()
        )
    });
}
