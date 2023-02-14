#![feature(proc_macro_hygiene, decl_macro)]

use std::{fs::File, io::Cursor};

use audiotags::Tag;
use rocket::{http::Status, Response};
use sqlite::{Connection, State};

#[macro_use]
extern crate rocket;

#[get("/")]
fn not_found() -> Status {
    Status::NotFound
}

#[get("/v0/cover/<id>")]
fn cover(id: i64) -> Response<'static> {
    if id == 0 {
        return Response::build()
            .header(rocket::http::ContentType::new("image", "jpeg"))
            .finalize();
    }
    let connection = Connection::open("songs.db").unwrap();
    let mut statement = connection
        .prepare(&format!("SELECT filename from songs WHERE id = {}", id))
        .unwrap();
    let mut filename = String::new();
    while let Ok(State::Row) = statement.next() {
        filename = statement.read::<String, _>(0).unwrap();
    }
    let tag = Tag::default()
        .read_from_path("D:\\Users\\Sergio\\Music\\Actual Music\\".to_owned() + &filename)
        .unwrap();
    let cover = match tag.album_cover() {
        Some(cover) => cover,
        None => return Response::build().status(Status::NotFound).finalize(),
    };
    let bytes = cover.data.to_vec();
    let stream = Cursor::new(bytes);
    Response::build()
        .header(rocket::http::ContentType::new("image", "jpeg"))
        .sized_body(stream)
        .finalize()
}

#[get("/v0/all")]
fn songs() -> Response<'static> {
    let connection = Connection::open("songs.db").unwrap();
    let mut statement = connection.prepare("SELECT * FROM songs").unwrap();
    let mut songs = "[".to_string();
    while let Ok(State::Row) = statement.next() {
        let filename = statement.read::<String, _>(1).unwrap();
        let id = statement.read::<i64, _>(0).unwrap();
        let tag = match Tag::default()
            .read_from_path("D:\\Users\\Sergio\\Music\\Actual Music\\".to_owned() + &filename)
        {
            Ok(tag) => tag,
            Err(_) => continue,
        };
        songs += &format!(
            "{{\"title\":\"{}\",\"artist\":\"{}\",\"album\":\"{}\",\"duration\":{},\"id\":{}}},",
            match tag.title() {
                Some(title) => title,
                None => "",
            },
            match tag.artist() {
                Some(artist) => artist,
                None => "",
            },
            match tag.album_title() {
                Some(album) => album,
                None => "",
            },
            match tag.duration() {
                Some(duration) => duration as i64,
                None => 0,
            },
            id
        );
    }
    // let mut chars = songs.chars();
    // chars.next_back();
    //
    // songs = chars.as_str().to_string();
    songs += "]";
    println!("{}", songs);
    Response::build()
        .header(rocket::http::ContentType::new("text", "html"))
        .sized_body(Cursor::new(songs))
        .finalize()
}

#[get("/tracks/<id>")]
fn track(id: i64) -> Response<'static> {
    let connection = Connection::open("songs.db").unwrap();
    let mut statement = connection
        .prepare(&format!("SELECT filename from songs WHERE id = {}", id))
        .unwrap();
    let mut filename = String::new();
    while let Ok(State::Row) = statement.next() {
        filename = statement.read::<String, _>(0).unwrap();
    }
    let stream =
        File::open("D:\\Users\\Sergio\\Music\\Actual Music\\".to_owned() + &filename).unwrap();
    Response::build()
        .header(rocket::http::ContentType::new("audio", "mpeg"))
        .sized_body(stream)
        .finalize()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![not_found, songs, track, cover])
        .launch();
}
