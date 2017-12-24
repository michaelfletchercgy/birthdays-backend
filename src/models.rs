#[derive(Queryable)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Birthday {
    pub id: i32,
    pub title: String,
    pub year: Option<i32>,
    pub month: i32,
    pub day: i32,
}

use super::schema::birthdays;

#[derive(Insertable)]
#[table_name="birthdays"]
pub struct NewBirthday {
    pub title: String,
    pub year: Option<i32>,
    pub month: i32,
    pub day: i32,
}