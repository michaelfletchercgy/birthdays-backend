use diesel::*;
use chrono;
use chrono::{Datelike};
use chrono::Duration;

pub fn push_to_subscriptions(con:&PgConnection) {
    use schema;
    use schema::subscriptions::dsl::*;
    use schema::birthdays::dsl::*;
    
    use reqwest;

    let today = chrono::Local::now().naive_local().date();
    let seven_days = today + Duration::days(7);
    // filter for unique user ids with birthdays in the next seven days.  Gotta figure out a weird OR clause to do that.

    println!("Sending notifications.");

    let users_with_nots = birthdays
        .select(schema::birthdays::dsl::user_id)
        .filter(
            (
            month.eq(today.month() as i32).and(day.gt(today.day() as i32))
            ).or(
                month.eq(seven_days.month() as i32).and(day.lt(seven_days.day() as i32))
            )
        )
        .distinct()
        .load::<i32>(con)
        .expect("error loading birthdays");

    println!("Users with notifications: {}", users_with_nots.len());

    let client = reqwest::Client::new();

    for user_id in users_with_nots {
        println!("Sending notifications for {}", user_id);

        let subs = subscriptions
            .select(url)
            .filter(schema::subscriptions::dsl::user_id.eq(user_id))
            .load::<String>(con)
            .expect("error");

        println!("{} has {} subscriptions", user_id, subs.len());
        for u in subs {
            println!("Posting to {}", u);
            client.post(&u)
                .body("")
                .send().unwrap();
        }
    }
}