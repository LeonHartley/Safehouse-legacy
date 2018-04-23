#[macro_use] extern crate nickel;
extern crate hyper;
extern crate rustc_serialize;

use api::SafehouseApi;

mod api;

fn main() {
    let api = SafehouseApi::new("0.0.0.0", 1337);

    api.start();
}