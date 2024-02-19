use serde::Serialize;

#[derive(Serialize)]
pub struct Day {
    pub day: u8,
    pub month: String,
    pub year: u16
}
impl Day {
    pub fn new(day: u8, month: String, year: u16) -> Day {
       Day {day, month, year} 
    }
}
