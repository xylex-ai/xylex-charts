extern crate csv;
use std::error::Error;
use std::fs::File;

use crate::utils::struct_model::OhlcData;


// Function to parse CSV data
pub fn read_ohlc_data(
    file_path: &str
) -> Result<Vec<OhlcData>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(File::open(file_path)?);
    let mut data = Vec::new();

    for result in rdr.deserialize() {
        let record: OhlcData = result?;
        data.push(record);
    }

    Ok(data)
}
