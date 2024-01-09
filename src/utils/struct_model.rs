extern crate serde;
extern crate serde_derive;

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OhlcData {
    // Make sure the types of these fields match the CSV data format
    pub time: u64,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
}
