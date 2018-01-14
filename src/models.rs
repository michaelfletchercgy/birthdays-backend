use ::BirthdayEndpoint;

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
    pub fn as_birthday_endpoint(&self) -> BirthdayEndpoint {
        BirthdayEndpoint {
            id: self.id,
            title: self.title.clone(),
            year: match self.year {
                None => "".to_string(),
                Some(x) => x.to_string()                
            }, 
            month: self.month.to_string(),
            day: self.day.to_string()
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