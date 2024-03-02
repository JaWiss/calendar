use std::{collections::HashMap, f64::consts, fmt::Result, fs::{self, create_dir, File}, io::{prelude::*, BufReader, ErrorKind}};

mod structs;
use structs::day::Date;



fn main() {
    let tag = Date::new(2, 3, 2024, "Essen".to_string());
    create_month_folders();
   // println!("{:?}", save_date(&tag));
    find_closest_date(&tag);
    println!("Nächste Termin ist: {:?}",find_closest_date(&tag)); 
}

fn create_month_folders() {
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

fn find_closest_date(date: &Date) -> Date { 
    let file_result = File::open(get_file_path(&date));
    let mut closest_date: Date = Date::new(1,1,1970,"FEHLER".to_string());
    let mut min_distance: u16 = 367;
    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(get_file_path(&date)) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
    let data = read_dates_out_of_json(file);
    for dates in data {
       if((dates.day.abs_diff(date.day) as u16) < min_distance) {
            min_distance = dates.day.abs_diff(date.day) as u16;
            closest_date = dates; 
       }
    }
    return closest_date;
}

fn read_dates_out_of_json(file: File) -> Vec<Date>{
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    let deserializer = serde_json::Deserializer::from_reader(reader);
    let iterator = deserializer.into_iter::<Date>();
    for item in iterator {
        let item_to_be_pushed = match item {
            Ok(date) => date,
            Err(error) => panic!("error reading a JSON date: {}", error),
        };   
        data.push(item_to_be_pushed);
    }
    return data;
}
