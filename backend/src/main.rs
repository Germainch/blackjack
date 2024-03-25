
use askama::Template;
use rouille::{Request, router};
use rouille::Response;
mod games;

#[derive(Template)]
#[template(path = "Hello.html")]
#[macro_use]

struct HelloTemplate<'a>{
    name : &'a str,
}

fn template_to_string() -> String{
    let hello = HelloTemplate{name : "joshua"};
    hello.render().unwrap()
}

fn main() {
    rouille::start_server("localhost:8000", move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::redirect_302("/hello")
            },
            (GET) (/hello) => {
                rouille::Response::text(template_to_string())
            },
            _ => rouille::Response::empty_404()
        )
    });
}
