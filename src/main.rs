use chrono::Datelike;
#[cfg(feature = "preserve_order")]
use indexmap::{self, IndexMap};
use serde_json;
use std::fs;

fn read_data() -> serde_json::value::Value {
    let file = fs::File::open("data.json").expect("file should open read only");
    serde_json::from_reader(file).expect("file should be proper JSON")
}
fn current_day() -> std::string::String {
    let now = chrono::Utc::now();
    use chrono::Weekday;
    match now.weekday() {
        Weekday::Mon => "Lunedi".to_string(),
        Weekday::Tue => "Martedi".to_string(),
        Weekday::Wed => "Mercoledi".to_string(),
        Weekday::Thu => "Giovedi".to_string(),
        Weekday::Fri => "venerdi".to_string(),
        Weekday::Sat => "Sabato".to_string(),
        Weekday::Sun => "Domenica".to_string(),
    }


}

fn materie(){
    let data = read_data();
    let data = &data[current_day()];
    if data.is_array() {
        println!("{}", current_day());
        for materia in data.as_array().unwrap(){
            println!("{}" , materia);
            // println!("{:?}", materie_array)
        }
    }
    else {
        println!("Sembra tu oggi sia libero 😄");
    }
    // println!("{}", current_day());
    // println!("{}", data);

        
    }


fn main() {
    materie();
    
}
