#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate diesel;

use diesel::prelude::*;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

pub mod hero;
pub mod schema;

use crate::hero::{Hero, NewHero};


#[database("hero")]
struct DbConn(diesel::MysqlConnection);

#[get("/")]
fn read(conn: DbConn) -> Result<Json<Vec<Hero>>, String> {
    use crate::schema::hero::dsl::*;
    hero.load(&conn.0).map_err(|err| -> String {
        println!("Error querying: {:?}", err);
        "Error querying heroes from the database".into()
    }).map(Json)
}

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[post("/", format = "json", data = "<newHero>")]
fn create(conn: DbConn, newHero: Json<NewHero>) -> Result<String, String> {
    use crate::schema::hero;
    diesel::insert_into(hero::table)
    .values(&newHero.into_inner())
    .execute(&conn.0)
    .map_err(|_err| -> String {
        "Error when inserting".into()
    })
    .map(|_| {
        "Successfully inserted!".into()
    })
}

#[put("/<id>", data = "<newHero>")]
fn update(conn: DbConn, id: i32, newHero: Json<NewHero>) -> Result<JsonValue, JsonValue> {
    use crate::schema::hero::dsl::hero;
    diesel::update(hero.find(id))
    .set(&newHero.into_inner())
    .execute(&conn.0)
    .map_err(|_| {
        json!({"status": "err"})
    })
    .map(|_| {
        json!({"status": "ok"})
    })
}

#[delete("/<id>")]
fn delete(conn: DbConn, id: i32) -> Result<JsonValue, JsonValue> {
    use crate::schema::hero::dsl::hero;
    diesel::delete(hero.find(id))
    .execute(&conn.0)
    .map_err(|_| {
        json!({"status": "err"})
    })
    .map(|_| {
        json!({"status": "ok"})
    })
}


fn main() {
    rocket::ignite()
    .attach(DbConn::fairing())
    .mount("/", routes![hello])
    .mount("/hero", routes![create, update, delete])
    .mount("/heroes", routes![read])
    .launch();
}