use serde_json::{self, to_string_pretty};

mod structs;
use structs::day::Date;

fn main() {
    let tag = Date::new(20,"Februar".to_string(),2024, "Essen".to_string());

}
