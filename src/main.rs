#![feature(proc_macro_hygiene, decl_macro)]

use std::{fs::File, ops::Index};

use audiotags::Tag;
use rocket::{http::Status, Response};
use sqlite::{Connection, open};

#[macro_use] extern crate rocket;

#[get("/")]
fn not_found() -> Status {
    Status::NotFound
}

#[get("/v0/all")]
fn song() -> String {
    let connection = Connection::open("songs.sqlite").unwrap();
    let mut statement = connection.prepare("SELECT filename FROM songs").unwrap();

    "Hello, world!".to_string()
}

#[get("/song")]
fn track() -> Response<'static> {
    Response::build()
        .header(rocket::http::ContentType::new("audio", "mpeg"))
        .sized_body(File::open("D:\\Users\\Sergio\\Music\\Actual Music\\(G)I-DLE - HWAA.flac").unwrap())
        .finalize()
}

fn main() {
    rocket::ignite().mount("/", routes![not_found, song, track]).launch();
}
