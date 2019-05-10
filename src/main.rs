#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;

fn main() {

    rocket::ignite().mount("/", StaticFiles::from("static")).launch();
}
