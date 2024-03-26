use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]
#[macro_use]

struct HelloTemplate<'a>{
    name : &'a str,
}

pub fn template_to_string() -> String {
    let hello = HelloTemplate{ name : "joshua" };
    hello.render().unwrap()
}