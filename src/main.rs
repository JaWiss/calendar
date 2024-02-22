use std::fs::{self, create_dir, File};

mod structs;
use serde::de::Error;
use structs::day::Date;

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
    file_path.push_str(find_month(date)); 
    file_path.push_str(".json"); 
    // open file and find where the new entry needs to be sorted.
}

fn find_month(date: &Date) -> &String {
   &date.month 
}

fn get_file(address: &str) -> File {
    let mut file = match File::open(address) {
        Ok(file) => file,
        Err(err) => {
            create_month_file(address);
            File::open(address)
        }
    };
}
