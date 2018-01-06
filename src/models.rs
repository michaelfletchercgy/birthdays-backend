use ::Birthday;

#[derive(Queryable)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BirthdayRecord {
    pub id: i32,
    pub title: String,
    pub year: Option<i32>,
    pub month: i32,
    pub day: i32,
    pub user_id: i32
}

impl BirthdayRecord {
    pub fn as_birthday(&self) -> Birthday {
        Birthday {
            id: self.id,
            title: self.title.clone(),
            year: self.year,
            month: self.month,
            day: self.day
        }
    }
}


#[derive(Queryable)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,    
}


use super::schema::birthdays;

#[derive(Insertable)]
#[table_name="birthdays"]
pub struct NewBirthdayRecord {
    pub title: String,
    pub year: Option<i32>,
    pub month: i32,
    pub day: i32,
    pub user_id: i32
}