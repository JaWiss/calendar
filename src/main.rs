use std::{collections::HashMap, f64::consts, fmt::Result, fs::{self, create_dir, File}, io::{prelude::*, BufReader, ErrorKind}};

mod structs;
use structs::day::Date;
use chrono::prelude::*;


fn main() {
    let tag = Date::new(2, 3, 2024, "Essen".to_string());
    create_month_folders();
   // println!("{:?}", save_date(&tag));
    let result = find_closest_dates_around_date(&tag, 4);
    let dates_before = result.0;
    let dates_after = result.1;
    println!("Nächste Termin ist: {:?}, {:?}",dates_before, dates_after); 
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

fn get_file_path_from_month(month: u8) -> String {
    let mut file_path = String::new();
    file_path.push_str("months/"); 
    file_path.push_str(translate_month(&month)); 
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

fn get_month_file(month: u8) -> File {
    let file_result = File::open(get_file_path_from_month(month));
    let mut closest_date: Date = Date::new(1,1,1970,"FEHLER".to_string());
    let mut min_distance: u16 = 367;
    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(get_file_path_from_month(month)) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
    return file;
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

fn calculate_month_distance(month1:u8, month2: u8) -> i16 {
    let month_duration: [u16; 12] = [31,28,31,30,31,30,31,31,30,31,30,31]; 
    let mut total_duration: i16 = 0;
    let mut index = month1;
    while index != month2 {
       total_duration += month_duration[(index - 1) as usize] as i16; 
       index =  index % 12 + 1;
    }
    return total_duration;
}

fn calculate_difference(date1: &Date, date2: &Date) -> i16{
    let mut total_difference: i16 = 0;
    total_difference += calculate_month_distance(*find_month(date1), *find_month(date2));
    total_difference += calculate_day_difference(date1.day, date2.day);
    return total_difference;
}

fn calculate_day_difference(day1: u8, day2: u8) -> i16 {
    return (day2 as i16 - day1 as i16) as i16;
}   


fn find_closest_date_after(date: &Date, month: u8) -> Date{
    let mut closest_date: Date = Date::new(1,1,1970,"FEHLER".to_string());
    let mut min_distance: u16 = 367;
    let data = read_dates_out_of_json(get_month_file(month));
    for dates in data {
        let distance: i16 = calculate_difference(date, &dates);  
        if (dates.day.abs_diff(date.day) as u16) >= min_distance {
            continue;
        }
        if distance < 0 {
           continue;
        }
        if dates.reason == date.reason && distance == 0 {
            continue;
        }
        min_distance = dates.day.abs_diff(date.day) as u16;
        closest_date = dates; 
    }
    if month as u32 == Local::now().month() - 1 {
        return closest_date;
    }
    if closest_date.year == 1970 {
       let new_month: u8= month % 12 + 1;
       return find_closest_date_after(date,new_month); 
    }
    return closest_date;
}

// CHange it so it gets all months that are before or after a date and then searches for closest
// values
fn find_closest_date_before(date: &Date, month: u8) -> Date{
    let mut closest_date: Date = Date::new(1,1,1970,"FEHLER".to_string());
    let mut min_distance: i16 = -367;
    let data = read_dates_out_of_json(get_month_file(month));
    println!("DATE: {:?}", date);
    for dates in data {
        println!("DATa: {:?}",dates);
        let distance: i16 = calculate_difference(date, &dates);
        println!("DISCTANCE: {:?}", distance);
        if distance < min_distance {
            continue;
        }
        if distance > 0 {
            continue;
        }
        if dates.reason == date.reason && distance == 0 {
            continue;
        }
        min_distance = distance;
        closest_date = dates;
    }
    if month as u32 == (Local::now().month() % 12) + 1{
        //println!("AM ENDE: {}", month);
        return closest_date;
    }
    if closest_date.year == 1970 {
       let new_month: u8= month - 2 % 12 + 1;
       return find_closest_date_after(date, new_month); 
    }
    return closest_date;
}

fn find_closest_dates_around_date(date: &Date, number_of_dates: u8) -> (Vec<Date>, Vec<Date>) {
    let mut dates_after: Vec<Date> = Vec::new();  
    let mut dates_before: Vec<Date> = Vec::new();  
    dates_after.push(find_closest_date_after(&date, *find_month(date)));
    dates_before.push(find_closest_date_before(&date, *find_month(date)));
    for i in 0..number_of_dates - 1 {
        dates_after.push(find_closest_date_after(&dates_after[i as usize], *find_month(date)));
        dates_before.push(find_closest_date_before(&dates_before[i as usize], *find_month(date)));
    }
    return (dates_before, dates_after);
}
// MIT DEM BORROW CHECKER WEGEN FILES ÜBERGEBEN, STREITEN!
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
