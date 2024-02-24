use std::{collections::HashMap, f64::consts, fs::{self, create_dir, File}};

mod structs;
use serde::de::Error;
use lazy_static::lazy_static;
use structs::day::Date;

lazy_static! {
    static ref MONTH_TRANSLATION: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

fn main() {
    let tag = Date::new(29,"Februar".to_string(),2024, "Essen".to_string());
    let folder_path = String::from("months");
    let months: [&str; 12] = ["January", "February", "March", "April", "May", 
    "June", "July", "August", "September", "October", "November", "December"];
    if let Err(_) = fs::metadata(&folder_path) {
        match fs::create_dir(&folder_path) {
            Ok(_) => println!("Folder created."),
            Err(e) => eprintln!("Error creating folder: {}", e),
        }
    } else {
        println!("Folder already exists.");
    }
    for month in &months {
        let mut file_name:String = String::new(); 
        file_name.push_str(&folder_path);
        file_name.push_str("/");
        file_name.push_str(month);
        file_name.push_str(".json");
        create_month_file(&file_name);
    }
    // Überprüfen wo Tag eingeordnet werden muss
    save_date(&tag);
}

fn create_month_file(month: &str) {
   if let Err(_) = fs::metadata(&month) {
       match File::create(&month) {
           Ok(_) => println!("file created"),
           Err(_) => println!("Error while creating file"),
           
       }
   } else {
       println!("file already exists");
   }
}

fn save_date(date: &Date) {
    let mut file_path = String::new();
    file_path.push_str("months/"); 
    file_path.push_str(translate_month(find_month(date))); 
    file_path.push_str(".json"); 
    let data = fs::read_to_string(file_path);
    println!("DATA: {:?}", data);
}

fn find_month(date: &Date) -> &String {
    &date.month 

}
fn translate_month(month: &str) -> &str{
    let mut month_translation: HashMap<&str, &str> = HashMap::new();
    month_translation.insert("Januar", "January");
    month_translation.insert("Februar", "February");
    month_translation.insert("März", "March");
    month_translation.insert("April", "April");
    month_translation.insert("Mai", "May");
    month_translation.insert("Juni", "June");
    month_translation.insert("Juli", "July");
    month_translation.insert("August", "August");
    month_translation.insert("September", "September");
    month_translation.insert("Oktober", "October");
    month_translation.insert("November", "November");
    month_translation.insert("Dezember", "December");
    month_translation[month]
}
