#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::{Json, Value};

#[get("/")]
fn index() -> Json<Value> {
    Json(json!({
        "status": "online"
    }))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}