#![allow(clippy::single_char_pattern)]
#![allow(clippy::needless_return)]

use chrono::offset::{Local, TimeZone};
use chrono::{Date, Duration};
use plotters::prelude::*;
use chrono::NaiveDateTime;
use serde_json::Value;
use std::string::String;





fn parse_time(t: &str) -> Date<Local> {
    println!("{}", t);
    Local
        .datetime_from_str(t, "%Y-%m-%d %H:%M")
        .unwrap()
        .date()


}

const OUT_FILE_NAME: &str = "stock.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<(String, f32, f32, f32, f32)> = load_ohlc_data("EURUSD", "15m");


    let root = BitMapBackend::new(OUT_FILE_NAME, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let length: usize = data.len();
    let (to_date, from_date) = (
        parse_time(&data[0].0) + Duration::days(1),
        parse_time(&data[length - 1].0) - Duration::days(1), // FIXME this is not correct
    );

    println!("from_date {:?}", from_date);
    println!("to_date {:?}", to_date);


    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(0)
        .y_label_area_size(0)
        .build_cartesian_2d(from_date..to_date, 1.06f32..1.09f32)?;

    chart.configure_mesh()
        .disable_mesh()
        .disable_axes()
        .light_line_style(WHITE)
        .draw()?;

    let candle_color_bull = convert_hex_to_rgb("#5b9cf6");
    let candle_color_bear = convert_hex_to_rgb("#9598a1");

    chart.draw_series(
        data.iter().map(|x| {
            CandleStick::new(
                parse_time(&x.0), x.1, x.2, x.3, x.4, candle_color_bull.filled(), candle_color_bear.filled(), 15)
        }),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);



    Ok(())
}


fn load_ohlc_data(
    pairname: &str,
    timeframe: &str,
) -> Vec<(String, f32, f32, f32, f32)> {
    let request_url: String = format!(
        "https://api.v2.xylex.ai/data/realtime/tohlc?pairname={}&timeframe={}&api_key={}",
        pairname, timeframe, 123
    );

    let response: String = reqwest::blocking::get(request_url).unwrap().text().unwrap();
    let v: Value = serde_json::from_str(&response).expect("Unable to parse JSON");

    let time: Vec<i64> = v["tohlc_frame"]["time"].as_array().unwrap().iter().map(|x| x.as_i64().unwrap()).collect();
    let open: Vec<f32> = v["tohlc_frame"]["open"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap() as f32).collect();
    let high: Vec<f32> = v["tohlc_frame"]["high"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap() as f32).collect();
    let low: Vec<f32> = v["tohlc_frame"]["low"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap() as f32).collect();
    let close: Vec<f32> = v["tohlc_frame"]["close"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap() as f32).collect();

    let ohlc_data: Vec<(String, f32, f32, f32, f32)> = time.iter().enumerate().map(|(i, &t)| {
        let date = NaiveDateTime::from_timestamp(t / 1000, 0);
        let date = Local.from_utc_datetime(&date).format("%Y-%m-%d %H:%M:%S").to_string();
        (date, open[i], high[i], low[i], close[i])
    }).collect();

    // limit the number of rows to 100
    let mut ohlc_data: Vec<(String, f32, f32, f32, f32)> = ohlc_data.iter().rev().take(100).cloned().collect();
    ohlc_data.reverse();

    // take the newest date and round it to a full day and then strip all the time from all rows and count in full days back for each row
    let base_date = NaiveDateTime::parse_from_str(&ohlc_data[0].0, "%Y-%m-%d %H:%M:%S").unwrap().date();
    for (i, row) in ohlc_data.iter_mut().enumerate() {
        let new_date = base_date + chrono::Duration::days(i as i64);
        row.0 = new_date.format("%Y-%m-%d").to_string();
    }


    println!("{:?}", ohlc_data);

    ohlc_data
}


fn convert_hex_to_rgb(hex: &str) -> RGBColor {
    let hex: &str = hex.trim_start_matches('#');
    let r: u8 = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g: u8 = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b: u8 = u8::from_str_radix(&hex[4..6], 16).unwrap();
    RGBColor(r, g, b)
}