use std::collections::HashMap;
use std::fs::{File, read_to_string};
use std::io;
use std::io::{Read, Write};
use std::ops::Add;
use std::sync::{Mutex, MutexGuard};
use askama::Template;
use rouille::{log, post_input, Request, Response, router, session, try_or_400};
use crate::blackjack::games_list::GameList;
use crate::server::{handle_route, SessionData};

mod templates;
pub(crate) mod blackjack;
mod server;
mod users;


fn main() {

    let sessions_storage: Mutex<HashMap<String, SessionData>> = Mutex::new(HashMap::new());
    let mut games_storage  = Mutex::new(GameList::new());

    rouille::start_server("localhost:8000", move |request| {



        rouille::log(&request, io::stdout(), || {
            rouille::session::session(request, "SID", 3600, |session| {


                let mut session_data;

                if session.client_has_sid(){
                    match sessions_storage.lock().unwrap().get(session.id()){
                        None => {
                            session_data = None;
                        }
                        Some(data) => {
                            session_data = Some(data.clone());
                        }
                    }
                }
                else {
                    session_data = None;
                }


                let mut game_list = games_storage.lock().unwrap();


                let response = handle_route(&request, &mut session_data, &mut game_list);

                match session_data {
                    None => {
                        if session.client_has_sid() {
                            // If `handle_route` erased the content of the `Option`, we remove the session
                            // from the storage. This is only done if the client already has an identifier,
                            // otherwise calling `session.id()` will assign one.
                            sessions_storage.lock().unwrap().remove(session.id());
                        }
                    }
                    Some(d) => {
                        sessions_storage.lock().unwrap().insert(session.id().to_owned(), d);
                    }
                };

                // During the whole handling of the request, the `sessions_storage` mutex was only
                // briefly locked twice. This shouldn't have a lot of influence on performances.

                response
            })
        })
    });
}
