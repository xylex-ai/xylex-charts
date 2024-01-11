// render_engine/core.rs



extern crate piston_window;
use piston_window::*;
use dotenv::dotenv;
use std::env;

use crate::data::csv_parser::read_ohlc_data;
use crate::render_engine::candle::scale_ohlc_data;
use crate::utils::struct_model::OhlcData;
use crate::utils::format_normalizer::convert_hex_color_to_normalized;


pub fn initialize_chart() {
    dotenv().ok();

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
    let mut vertical_offset: f64 = 0.0;
    let mut is_panning: bool = false;
    let mut last_mouse_pos: Option<[f64; 2]> = None;
    let mut candle_width: f64 = candle_width;
    let scroll_speed: f64 = 0.5;
    let mut window_width: f64 = 1000.0;

    let min_candle_width: f64 = 1.0;
    let max_candle_width: f64 = 20.0;

    let mut total_chart_width: f64 = scaled_data.len() as f64 * (candle_width + candle_x_spacing);
    let mut horizontal_offset: f64 = window_width - total_chart_width;


    while let Some(event) = window.next() {

        if let Some(size) = event.resize_args() {
            window_width = size.window_size[0];
            horizontal_offset = window_width - total_chart_width;
        }

        // start panning
        if let Some(Button::Mouse(button)) = event.press_args() {
            if button == MouseButton::Left {
                is_panning = true;
            }
        }

        // stop panning
        if let Some(Button::Mouse(button)) = event.release_args() {
            if button == MouseButton::Left {
                is_panning = false;
                last_mouse_pos = None;
            }
        }

        // free panning
        if is_panning {
            if let Some(pos) = event.mouse_cursor_args() {
                if let Some(last_pos) = last_mouse_pos {
                    let delta_x: f64 = pos[0] - last_pos[0];
                    let delta_y: f64 = pos[1] - last_pos[1];
                    horizontal_offset += delta_x;
                    vertical_offset += delta_y;
                }
                last_mouse_pos = Some(pos);
            }
        }

        // Update zoom and horizontal offset
        if let Some(args) = event.mouse_scroll_args() {
            let old_candle_width: f64 = candle_width;
            let attempted_candle_width: f64 = candle_width + args[1] * scroll_speed;

            if attempted_candle_width <= min_candle_width || attempted_candle_width >= max_candle_width {
                // If zoom limit is reached, initiate horizontal panning
                let panning_direction: f64 = if args[1] > 0.0 { -1.0 } else { 1.0 };
                let panning_speed: f64 = 50.0; // adjust as needed
                horizontal_offset += panning_direction * panning_speed;

                // Clamp to prevent going out of bounds
                horizontal_offset = horizontal_offset.clamp(window_width - total_chart_width, 0.0);
            } else {
                // Proceed with normal zooming
                candle_width = attempted_candle_width.clamp(min_candle_width, max_candle_width);

                // Reference point (e.g., center of the window)
                let reference_point: f64 = window_width / 2.0;

                // Find the position of the reference point in chart units
                let chart_units: f64 = (reference_point - horizontal_offset) / (old_candle_width + candle_x_spacing);

                // Recalculate total_chart_width
                total_chart_width = scaled_data.len() as f64 * (candle_width + candle_x_spacing);

                // Adjust horizontal_offset to keep the same data point at the reference point
                horizontal_offset = reference_point - chart_units * (candle_width + candle_x_spacing);

                // Clamp to prevent going out of bounds
                horizontal_offset = horizontal_offset.clamp(window_width - total_chart_width, 0.0);
            }
        }


        // get current window width
        if let Some(size) = event.resize_args() {
            window_width = size.window_size[0];
        }

        // canvas updater
        window.draw_2d(&event, |context, graphics, _| {
            clear([0.0, 0.0, 0.0, 1.0], graphics);

            // Improved calculation for the starting index
            let total_candle_space: f64 = candle_width + candle_x_spacing;
            let start_index: usize = ((-horizontal_offset / total_candle_space).floor() as isize).max(0) as usize;
            let end_index: usize = ((-horizontal_offset + window_width) / total_candle_space).ceil().min(scaled_data.len() as f64) as usize;

            for i in start_index..end_index {
                let ohlc: &OhlcData = &scaled_data[i];
                let x_position: f64 = (i as f64 * (candle_width + candle_x_spacing)) + horizontal_offset;

                // Calculate y positions for high and low (wick)
                let y_high: f64 = ohlc.high as f64 + vertical_offset;
                let y_low: f64 = ohlc.low as f64 + vertical_offset;

                // Calculate candle color
                let color: [f32; 4] = calculate_candle_color(ohlc);

                // Draw the wick
                let wick: Line = line::Line::new(color, 0.5);
                let wick_coords: [f64; 4] = [x_position + candle_width / 2.0, y_high, x_position + candle_width / 2.0, y_low];
                wick.draw(wick_coords, &Default::default(), context.transform, graphics);

                let y_open: f64 = ohlc.open as f64 + vertical_offset;
                let y_close: f64 = ohlc.close as f64 + vertical_offset;
                let rect: [f64; 4] = [x_position, y_open, candle_width, y_close - y_open];

                rectangle(color, rect, context.transform, graphics);
            }
        });
    }
}


fn calculate_candle_color(ohlc: &OhlcData) -> [f32; 4] {
    let color_candle_up = env::var("COLOR_CANDLE_UP").expect("COLOR_CANDLE_UP not found");
    let color_candle_down = env::var("COLOR_CANDLE_DOWN").expect("COLOR_CANDLE_DOWN not found");

    let color_candle_up: [f32; 4] = convert_hex_color_to_normalized(
        &color_candle_up
    );
    let color_candle_down: [f32; 4] = convert_hex_color_to_normalized(
        &color_candle_down
    );

    if ohlc.close > ohlc.open {
        color_candle_up
    } else {
        color_candle_down
    }
}