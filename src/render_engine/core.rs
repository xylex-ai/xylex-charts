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
            [1000, 480]
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
    let candle_x_spacing: f64 = 2.0; // space between each candlestick

    render_candlestick_series(
        &mut window,
        &scaled_data,
        candle_width,
        candle_x_spacing
    );
}


fn render_candlestick_series(
    window: &mut PistonWindow,
    scaled_data: &Vec<OhlcData>,
    candle_width: f64,
    candle_x_spacing: f64
) {
    let mut horizontal_offset: f64 = 0.0;
    let mut vertical_offset: f64 = 0.0;
    let mut is_panning: bool = false;
    let mut last_mouse_pos: Option<[f64; 2]> = None;
    let scroll_speed: f64 = 25.0;


    while let Some(event) = window.next() {
        // Debugging
        if let Some(pos) = event.mouse_cursor_args() {
            println!("Mouse position: {:?}", pos);
        }

        if let Some(Button::Mouse(button)) = event.press_args() {
            if button == MouseButton::Left {
                is_panning = true;
                println!("Panning started");
            }
        }

        if let Some(Button::Mouse(button)) = event.release_args() {
            if button == MouseButton::Left {
                is_panning = false;
                last_mouse_pos = None;
                println!("Panning ended");
            }
        }

        if is_panning {
            if let Some(pos) = event.mouse_cursor_args() {
                if let Some(last_pos) = last_mouse_pos {
                    let delta_x = pos[0] - last_pos[0];
                    let delta_y = pos[1] - last_pos[1];
                    horizontal_offset += delta_x;
                    vertical_offset += delta_y;
                    println!("Offsets: ({}, {})", horizontal_offset, vertical_offset);
                }
                last_mouse_pos = Some(pos);
            }
        }

        if let Some(args) = event.mouse_scroll_args() {
            horizontal_offset += args[1] * scroll_speed;
            println!("Horizontal offset: {}", horizontal_offset);
        }

        window.draw_2d(&event, |context, graphics, _| {
            clear([0.0, 0.0, 0.0, 1.0], graphics);

            // Draw OHLC data with offsets
            for (i, ohlc) in scaled_data.iter().enumerate() {
                let x_position: f64 = (i as f64 * (candle_width + candle_x_spacing)) + horizontal_offset;
                let y_open: f64 = ohlc.open as f64 + vertical_offset;
                let y_close: f64 = ohlc.close as f64 + vertical_offset;

                let rect: [f64; 4] = [x_position, y_open, candle_width, y_close - y_open];
                let color: [f32; 4] = if ohlc.close > ohlc.open {
                    [0.5843, 0.5961, 0.6314, 1.0] // green
                } else {
                    [0.3569, 0.6118, 0.9647, 1.0] // red
                };


                rectangle(color, rect, context.transform, graphics);
            }
        });
    }
}

