use serde::Serialize;
use serde_json::{self, to_string_pretty};

#[derive(Serialize)]
pub struct Date {
    pub day: u8,
    pub month: String,
    pub year: u16,
    pub reason: String,
}
impl Date {
    pub fn new(day: u8, month: String, year: u16, reason: String) -> Date {
        Date {
            day,
            month,
            year,
            reason,
        }
    }
    pub fn convert_to_json(&self) -> String {
        to_string_pretty(&self).expect("Error converting Date to JSON")
    }
}
