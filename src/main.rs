use std::{collections::HashMap, f64::consts,io::prelude::*, fs::{self, create_dir, File}};

mod structs;
use structs::day::Date;



fn main() {
    let tag = Date::new(29, 3, 2024, "Essen".to_string());
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
    println!("{:?}", save_date(&tag));
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

fn save_date(date: &Date) -> std::io::Result<()>{
    let file_path: String = get_file_path(date);
    let mut data = fs::read_to_string(&file_path)?;
    let mut file = File::create(&file_path)?;
    data.push_str(&date.convert_to_json());
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn find_month(date: &Date) -> &u8 {
    &date.month 

}

fn get_file_path(date: &Date) -> String {
    let mut file_path = String::new();
    file_path.push_str("months/"); 
    file_path.push_str(translate_month(find_month(date))); 
    file_path.push_str(".json");
    file_path
}

fn translate_month(month: &u8) -> &str{
    let mut month_translation: HashMap<u8, &str> = HashMap::new();
    month_translation.insert(1,"January");
    month_translation.insert(2, "February");
    month_translation.insert(3, "March");
    month_translation.insert(4, "April");
    month_translation.insert(5, "May");
    month_translation.insert(6, "June");
    month_translation.insert(7, "July");
    month_translation.insert(8, "August");
    month_translation.insert(9, "September");
    month_translation.insert(10, "October");
    month_translation.insert(11, "November");
    month_translation.insert(12, "December");
    month_translation[month]
}

fn find_closest_date(date: &Date) {
    
}
