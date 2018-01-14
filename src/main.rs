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

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BirthdayEndpoint {
    pub id: i32,
    pub title: String,
    pub year: String,
    pub month: String,
    pub day: String
}

impl BirthdayEndpoint {
    pub fn as_birthday_record(&self, user:&User) -> BirthdayRecord {
        BirthdayRecord {
            id: self.id,
            title: self.title.clone(),
            year: Some(self.year.parse().unwrap()),
            month: self.month.parse().unwrap(),
            day: self.day.parse().unwrap(),
            user_id: user.user_id
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        match request.cookies().get_private("user_name") {
            Some(user_name_cookie) => {
                use ::schema::users::dsl::*;

                let connection = establish_connection();

                let x = users
                    .limit(1)
                    .filter(user_name.eq(&user_name_cookie.value()))
                    .first::<User>(&connection)
                    .expect("Error loading user");
                    
                Outcome::Success(x)
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
fn index(user:User) -> Json<Vec<BirthdayEndpoint>> {
    use ::schema::birthdays::dsl::*;

    let connection = establish_connection();

    let results = birthdays
        .filter(user_id.eq(user.user_id))
        .order(title.asc())
        .load::<BirthdayRecord>(&connection)
        .expect("Error loading birthdays");

    let new_results = results.into_iter().map(|x| x.as_birthday_endpoint()).collect();

    // apparently has a 1mb limit
    Json(new_results)
}

// #[post("foo")] delete, put, 
#[get("/birthdays/<birthday_id>")]
fn bday_get(user:User, birthday_id:i32) -> Json<BirthdayEndpoint> {
    use ::schema::birthdays::dsl::*;

    let connection = establish_connection();
    let result = birthdays
        .limit(1)
        .filter(id.eq(birthday_id))
        .filter(user_id.eq(user.user_id))
        .first::<BirthdayRecord>(&connection)
        .expect("Error loading birthdays");

    // apparently has a 1mb limit
    Json(result.as_birthday_endpoint())
}

#[post("/birthdays/<birthday_id>", data = "<bday>")]
fn bday_post(user:User, birthday_id: i32, bday: Json<BirthdayEndpoint>) -> Json<BirthdayEndpoint> {
    use ::schema::birthdays::dsl::*;

    let connection = establish_connection();

    let t = bday.title.clone();
    
    let y = if bday.year.len() > 0 {
        None
    } else {
        Some(bday.year.parse().expect("Year was not valid."))
    };

    let m = bday.month.parse().expect("Month was not valid.");
    let d = bday.day.parse().expect("Day was not valid.");

    let result = if birthday_id == 0 {
        let new_bday = NewBirthdayRecord {
            title: t,
            year: y,
            month: m,
            day: d,
            user_id: user.user_id
        };

        diesel::insert_into(birthdays)
        .values(&new_bday)
        .get_result(&connection)
        .expect("Error saving new post")
    } else {
        diesel::update(birthdays.find(birthday_id))
        .set(
            (
                title.eq(&t),
                year.eq(&y),
                month.eq(&m),
                day.eq(&d)
            ))
        .get_result::<BirthdayRecord>(&connection)
        .expect("expected a birthday")
    };
    

    
    // apparently has a 1mb limit
    Json(result.as_birthday_endpoint())
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
    
    use ::schema::users::dsl::*;
    let connection = establish_connection();

    users
        .limit(1)
        .filter(user_name.eq(&l.user_id))
        .first::<User>(&connection)
        .expect("Error loading user");

    if &l.password=="helloworld" {
        let u = l.user_id.clone();
        let cookie = Cookie::build("user_name", u)
            .permanent()
            .finish();
        cookies.add_private(cookie);        
    }
}

#[get("/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    let frontend_path = env::var("FRONTEND_PATH")
        .expect("FRONTEND_PATH must be set");
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

