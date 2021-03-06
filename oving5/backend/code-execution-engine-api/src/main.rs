#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, execute_code])
        .launch();
}

#[get("/code")]
fn execute_code() -> String {
    return String::from("This is da code");
}
