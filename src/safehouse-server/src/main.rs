#[macro_use] extern crate lazy_static;
#[macro_use] extern crate nickel;
extern crate hyper;
extern crate rustc_serialize;
extern crate mysql;

use api::SafehouseApi;
use realtime::SafehouseRealtime;
use database::DatabaseCtx;

use mysql::{Pool};

mod error;
mod models;
mod database;
mod api;
mod realtime;

static SAFEHOUSE_SERVER_API_PORT: i16 = 1337;
static SAFEHOUSE_SERVER_REALTIME_PORT: i16 = 1338;

fn main() {
    // start realtime server (websockets)
    SafehouseRealtime::new("0.0.0.0", SAFEHOUSE_SERVER_REALTIME_PORT).start();

    // start api server (http)
    SafehouseApi::new("0.0.0.0", SAFEHOUSE_SERVER_API_PORT).start();
}