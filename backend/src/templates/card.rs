use askama::Template;

#[derive(Template)]
#[template(path = "card.html")]
#[macro_use]
struct CardTemplate<'a>{
    value : &'a str,
    color : &'a str,
}

pub fn card_to_string(value: &str, color: &str) -> String {
    let card = CardTemplate{ value, color };
    card.render().unwrap()
}
