use askama::Template;

#[derive(Template)]
#[template(path = "bet.html")]
#[macro_use]
struct BetTemplate {
    bet: u32,
    bank: u32,
}

pub fn bet_to_string(value: u32, inital_bank: u32) -> String {
    let bet = BetTemplate{bet: value, bank: (inital_bank - value)};
    match bet.render(){
        Ok(s) => { s }
        Err(e) => { println!( "{}", e ); String::new()}
    }
}