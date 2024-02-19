mod structs {
    pub mod day;
}

use day::day;

fn main() {
    let tag = day::new(20,String::from("Februar"),2024);
    println!("{}",tag.month);
    println!("Hello, world!");
}
