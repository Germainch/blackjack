use askama::Template;

#[derive(Template)]
#[template(path = "bet.html")]
struct BetTemplate {
    bet: u32,
    bank: u32,
}

pub fn bet_to_string(value: u32, bank: u32) -> String {
    let bet = BetTemplate { bet: value, bank };
    bet.render().unwrap_or_else(|e| {
        println!("{}", e);
        String::new()
    })
}
