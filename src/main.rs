mod structs;
use structs::day::Day;

fn main() {
    let tag = Day::new(20,String::from("Februar"),2024);
    println!("{}",tag.month);
    println!("Hello, world!");
}
