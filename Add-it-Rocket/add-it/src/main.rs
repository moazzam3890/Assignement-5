#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::request::Form;

#[get("/")]
fn intro() -> String {
    format!("Please Enter your number at e.g: /get?number=100")
}

#[derive(FromForm)]
struct Num {
    number: usize,
}

#[get("/get?<num..>")]
fn gettin(num: Form<Num>) -> String {
    let result = num.number + 50;
    format!(" You have entered {} and answer after adding 50 is: {}", num.number, result)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![intro, gettin])
    .launch();
}