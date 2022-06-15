use chrono::Datelike;

struct User {
    first_name: String,
    last_name: String,
    birthday_year: u16,
    birthday_month: u8,
    birthday_date: u8,
}

impl User {
    fn age(&self) -> u16 {
        let current_date = chrono::Local::now().year() as u16;
        current_date - self.birthday_year
    }

    fn full_name (&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn full_birthday(&self) -> String {
        let month_string = self.birthday_month.to_string();
        let date_string = self.birthday_date.to_string();
        let year_string = self.birthday_year.to_string();
        format!("{}/{}/{}", month_string, date_string, year_string)
    }
}

fn main() {
    let user1 = User {
        first_name: String::from("Steven"),
        last_name: String::from("Fukase"),
        birthday_date: 7,
        birthday_month: 4,
        birthday_year: 1997,
    };
    println!("{:?}", user1.full_name());
    println!("{:?}", user1.age());
    println!("{:?}", user1.full_birthday());
}
