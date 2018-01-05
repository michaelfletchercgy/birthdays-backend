#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
extern crate dotenv;

#[macro_use]
extern crate serde_derive;

extern crate r2d2_diesel;
extern crate r2d2;
//extern crate r2d2_postgres;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::path::Path;
use std::path::PathBuf;

use self::models::*;
//use self::diesel::prelude::*;
use diesel::Connection;

use rocket_contrib::Json;

use rocket::Outcome;
use rocket::response::NamedFile;
use rocket::http::Status;
use rocket::http::Cookies;
use rocket::http::Cookie;
use rocket::request::{self, Request, FromRequest};


struct User {
    user_id:String
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        match request.cookies().get_private("user_id") {
            Some(user_id) => {
                Outcome::Success(
                    User{
                        user_id:user_id.to_string()
                    }
                )
            },
            None => {
                Outcome::Failure((Status::Unauthorized, ()))
            }
        }
    }
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[get("/check")]
fn check(_unused:User) -> () {    
    
}

#[get("/birthdays")]
fn index(_unused:User) -> Json<Vec<Birthday>> {
    use ::schema::birthdays::dsl::*;

    let connection = establish_connection();

    let results = birthdays
        .order(title.asc())
        .load::<Birthday>(&connection)
        .expect("Error loading birthdays");

    // apparently has a 1mb limit
    Json(results)
}

// #[post("foo")] delete, put, 
#[get("/birthdays/<birthday_id>")]
fn bday_get(_unused:User, birthday_id:i32) -> Json<Birthday> {
    use ::schema::birthdays::dsl::*;

    let connection = establish_connection();
    let result = birthdays
        .limit(1)
        .filter(id.eq(birthday_id))
        .first::<Birthday>(&connection)
        .expect("Error loading birthdays");

    // apparently has a 1mb limit
    Json(result)
}

#[post("/birthdays/<birthday_id>", data = "<bday>")]
fn bday_post(_unused:User, birthday_id: i32, bday: Json<Birthday>) -> Json<Birthday> {
    use ::schema::birthdays::dsl::*;

    let connection = establish_connection();

    let result = if birthday_id == 0 {
        let new_bday = NewBirthday {
            title: bday.title.clone(),
            year: bday.year,
            month: bday.month,
            day: bday.day
        };

        diesel::insert_into(birthdays)
        .values(&new_bday)
        .get_result(&connection)
        .expect("Error saving new post")
    } else {
        diesel::update(birthdays.find(birthday_id))
        .set(
            (
                title.eq(&bday.title),
                year.eq(&bday.year),
                month.eq(&bday.month)
            ))
        .get_result::<Birthday>(&connection)
        .expect("expected a birthday")
    };
    

    
    // apparently has a 1mb limit
    Json(result)
}

#[delete("/birthdays/<birthday_id>")]
fn bday_delete(_unused:User, birthday_id: i32) -> String {
    use ::schema::birthdays::dsl::*;

    let connection = establish_connection();

    diesel::delete(
        birthdays.filter(id.eq(birthday_id))
    )
    .execute(&connection)
    .expect("delete ok");
    // should have done something here.
    
    String::from("ok")
}

#[derive(FromForm)]
struct LoginArgs {
    user_id:String,
    password:String
}

#[get("/login?<args>")]
fn login(args:Option<LoginArgs>, mut cookies:Cookies) {
    let l = args.unwrap();
    println!("Checking {}, {}", l.user_id, l.password);
    if &l.user_id == "michaelfletcher" && l.password=="helloworld" {
        let cookie = Cookie::build("user_id", "michaelfletcher")
            .permanent()
            .finish();
        cookies.add_private(cookie);
        println!("setting cookie");
    }
}

#[get("/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    let frontend_path = env::var("FRONTEND_PATH")
        .expect("FRONTEND_PATH must be set");
    println!("{:?}-{:?}", &frontend_path, &file);
    NamedFile::open(Path::new(&frontend_path).join(file)).ok()
}

embed_migrations!("migrations");

fn main() {
    dotenv().ok();


    let connection = establish_connection();

    // By default the output is thrown out. If you want to redirect it to stdout, you
    // should call embedded_migrations::run_with_output.
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();

    rocket::ignite()
        .mount("/bday/", routes![static_files])
        .mount("/api/", routes![index, bday_get, bday_post, login, bday_delete])        
        .launch();
}

