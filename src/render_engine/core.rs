// render_engine/core.rs

extern crate piston_window;
use piston_window::*;

use crate::data::csv_parser::read_ohlc_data;
use crate::render_engine::candle::scale_ohlc_data;
use crate::utils::struct_model::OhlcData;


pub fn initialize_chart() {
    let mut window: PistonWindow =
        WindowSettings::new(
            "xylex-charts",
            [640, 480]
        )

            .exit_on_esc(true)
            .build()
            .unwrap();

    let ohlc_data: Vec<OhlcData> = read_ohlc_data(
        "test.csv"
    ).unwrap();

    let scaled_data: Vec<OhlcData> = scale_ohlc_data(
        &ohlc_data,
        480
    );
    let candle_width: f64 = 5.0; // width of each candlestick

    while let Some(event) = window.next() {
        window.draw_2d(
            &event,
            |context,
            graphics, _| {
            clear(
                [0.0, 0.0, 0.0, 1.0],
                graphics
            );

            // Draw OHLC data
            for (i,
                ohlc
            ) in scaled_data.iter().enumerate() {

                let x_position: f64 = i as f64 * candle_width; // Calculate x position

                let y_open: f64 = ohlc.open as f64;
                let y_close: f64 = ohlc.close as f64;

                let rect: [f64; 4] = [
                    x_position,
                    y_open,
                    candle_width,
                    y_close - y_open
                ]; // Rectangle parameters

                let color: [f32; 4] = if ohlc.close > ohlc.open {
                    [0.0, 1.0, 0.0, 1.0] // green
                } else {
                    [1.0, 0.0, 0.0, 1.0] // red
                };

                rectangle(
                    color,
                    rect,
                    context.transform,
                    graphics
                );
            }
        });
    }
}
