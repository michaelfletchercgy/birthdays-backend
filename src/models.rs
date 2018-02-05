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
        let mut short_display = String::new();

        if self.month > 0 && self.month < 13 {
            short_display.push_str(&format!("{} ", month_name(self.month)));
        }

        short_display.push_str(&format!("{}", self.day));

        if self.year.is_some() {
            short_display.push_str(&format!(" {}", self.year.unwrap()));
        }

        
        BirthdayEndpoint {
            id: self.id,
            title: self.title.clone(),
            year: match self.year {
                None => "".to_string(),
                Some(x) => x.to_string()                
            },
            month: format!("{} - {}", self.month.to_string(), month_name(self.month)),
            day: self.day.to_string(),
            short_display: Some(short_display)
        }
    }
}

pub fn month_name(m:i32) -> String {
    match m {
        1 => "January".to_string(),
        2 => "February".to_string(),
        3 => "March".to_string(),
        4 => "April".to_string(),
        5 => "May".to_string(),
        6 => "June".to_string(),
        7 => "July".to_string(),
        8 => "August".to_string(),
        9 => "September".to_string(),
        10 => "October".to_string(),
        11 => "November".to_string(),
        12 => "December".to_string(),
        _ => "".to_string()         
    }
}

pub fn parse_month(month_string:&str) -> Option<i32> {
    match month_string.split(" - ").nth(0) {
        Some(x) => {
            match x.parse() {
                Ok(y) => Some(y),
                Err(_) => None
            }
        },
        None => None
    }
    //month_string.split(" - ").nth(0).unwrap().parse().unwrap()

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