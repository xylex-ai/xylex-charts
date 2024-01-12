// render_engine/candle.rs

use crate::utils::struct_model::OhlcData;


pub fn scale_ohlc_data(
    data: &Vec<OhlcData>,
    height: u32
) -> Vec<OhlcData> {
    // Calculate max and min values
    let max_price: f32 = data.iter().map(
        |x: &OhlcData | x.high).fold(
   f32::MIN,
      f32::max
        );
    let min_price: f32 = data.iter().map(
        |x: &OhlcData| x.low
    ).fold(
        f32::MAX,
        f32::min
    );

    // Scale data
    data.iter().map(|d: &OhlcData | OhlcData {
        time: d.time,
        open: scale_value(
            d.open,
            min_price,
            max_price,
            height
        ),

        high: scale_value(
            d.high,
            min_price,
            max_price,
            height
        ),

        low: scale_value(
            d.low,
            min_price,
            max_price,
            height
        ),

        close: scale_value(
            d.close,
            min_price,
            max_price,
            height
        ),

    }).collect()
}


pub fn scale_value(
    value: f32,
    min: f32,
    max: f32,
    height: u32
) -> f32 {

    ((value - min) / (max - min)) * height as f32
}