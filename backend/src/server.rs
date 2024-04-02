use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use rouille::{post_input, Request, Response, router, try_or_400};
use crate::blackjack::games_list::GameList;

use crate::users::is_user;
use crate::templates::bet;
use crate::templates::bet::bet_to_string;
use crate::templates::canvas::canvas_to_string;

#[derive(Debug,Clone)]
pub(crate) struct SessionData{
    login: String,
}

pub(crate) fn concat_html_css(html: &str, css: &str) -> String{
    let mut concat:String = String::new();
    concat = html.to_owned() + "<style>" + css + "</style>";
    concat
}

pub(crate) fn handle_route(request: &Request, session_data: &mut Option<SessionData>, game_list: &mut GameList) -> Response {
    match session_data {
        // the client is not logged in
        None => {
            handle_route_not_logged_in(request, session_data)
        }
        // the client is logged in
        Some(s) => {
            handle_route_logged_in(request, session_data, game_list)
        }
    }
}
pub(crate) fn handle_route_not_logged_in(request:&Request, session_data: &mut Option<SessionData>) -> Response {
    router!(request,

        (GET) (/) => {
            let file = File::open("./index.html");
            let mut html = String::new();
            match file{
                Ok(mut f) => {
                    f.read_to_string(&mut html).unwrap_or_default();
                    Response::html(html)
                }
                Err(_) => { Response::empty_404() }
            }
        },

        (POST) (/login) => {
            let data = try_or_400!(post_input!(request, {
                login : String,
                password : String,
            }));

            let mut lock = io::stdout().lock();
            writeln!(lock, "user tried to connect with login : {:?} and password : {:?}",data.login, data.password).unwrap();

            // calls the function to check the login+password in the database
             match check_login(&data.login, &data.password){
                Ok(()) => {
                    *session_data = Some(SessionData{ login: data.login });
                    return Response::redirect_303("/start_game");
                }
                Err(_) => {
                    return Response::html("wrong login/password");
                }
            }
        },

        _ => Response::empty_404()
    )
}
pub(crate) fn handle_route_logged_in(request: &Request, session_data: &mut Option<SessionData>, game_list: &mut GameList) -> Response{
    router!(request,
        (GET) (/) => {
            Response::redirect_303("/start_game")
        },
        (GET) (/start_game) => {

            let file = File::open("./templates/start_game_button.html");
            let mut html = String::new();
            match file{
                Ok(mut f) => {
                    f.read_to_string(&mut html).unwrap_or_default();
                    Response::html(html)
                }
                Err(_) => { Response::empty_404() }
            }
        },
        (GET) (/game) => {
            // accessing game list and checking if the user has a game ongoing
            let login = session_data.to_owned().unwrap().login;

            let game = game_list.find_game(login.clone());
            match game{
                None => {
                    game_list.add_game(login.clone());
                    let canvas = canvas_to_string(game_list.find_game(login).unwrap());
                    Response::html(canvas)
                }
                Some(game) => {
                    let canvas = canvas_to_string(game);
                    Response::html(canvas)
                }
            }
        },

        (GET) (/deck) => {
            let image = File::open("deck.png").unwrap();
            Response::from_file("image/png", image)
        },

        (POST) (/bet) => {
            let login = session_data.to_owned().unwrap().login;

            let game = game_list.find_game(login);

            match game {
                None => { Response::empty_404() }
                Some(g) => {
                    g.bet();
                    let bet = g.player_bet();
                    let money = g.player_money();

                    Response::html(bet_to_string(bet, money))
                }
            }

        },

        (POST) (/draw) => {
            let login = session_data.to_owned().unwrap().login;

            let game = game_list.find_game(login);

            match game {
                None => { Response::empty_404() }
                Some(g) => {
                    g.player_draw();

                    let canvas = canvas_to_string(g);
                    Response::html(canvas)
                }
            }
        },

        (POST) (/logout) => {
            *session_data = None;
            return Response::html(r#"LoggedOut"#)
        },


        _ => Response::empty_404()
    )
}

pub(crate) fn check_login(login:&str,password: &str) -> Result<(),()>{
    // checking if the db has the combination of log/password
    if is_user(login,password){
        return Ok(());
    }
    return Err(());
}