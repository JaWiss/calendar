use serde::{Deserialize, Serialize};
use serde_json::{self, to_string_pretty};

#[derive(Debug,Serialize,Deserialize)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
    pub reason: String,
}
impl Date {
    pub fn new(day: u8, month: u8, year: u16, reason: String) -> Date {
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
