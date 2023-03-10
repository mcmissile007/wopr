use std::time::{Duration, Instant};
use wopr::indicator;
use wopr::VERSION;
#[allow(unused_imports)]
use wopr::indicator::*;



fn main() {
    let highs: [f64; 42] = [
        274.0, 273.25, 272.0, 270.75, 270.0, 270.5, 268.5, 265.5, 262.5, 263.5, 269.5, 267.25,
        267.5, 269.75, 268.25, 264.0, 268.0, 266.0, 274.0, 277.5, 277.0, 272.0, 267.25, 269.25,
        266.0, 265.0, 264.75, 261.0, 257.5, 259.0, 259.75, 257.25, 250.0, 254.25, 254.0, 253.25,
        253.25, 251.75, 253.0, 251.5, 246.25, 244.25,
    ];
    let lows: [f64; 42] = [
        272.0, 270.25, 269.75, 268.0, 269.0, 268.0, 266.5, 263.0, 259.0, 260.0, 263.0, 265.0,
        265.5, 266.0, 263.25, 261.5, 266.25, 264.25, 267.0, 273.5, 272.5, 269.5, 264.0, 263.0,
        263.5, 262.0, 261.5, 255.5, 253.0, 254.0, 257.5, 250.0, 247.0, 252.75, 250.50, 250.25,
        251.0, 250.50, 249.5, 245.25, 240.0, 241.25,
    ];
    let closes: [f64; 42] = [
        272.75, 270.75, 270.0, 269.25, 269.75, 270.0, 266.5, 263.25, 260.25, 263.0, 266.5, 267.0,
        265.75, 268.5, 264.25, 264.0, 266.5, 265.25, 273.0, 276.75, 273.0, 270.25, 266.75, 263.0,
        265.5, 262.25, 262.75, 255.5, 253.0, 257.5, 257.5, 250.0, 249.75, 253.75, 251.25, 250.5,
        253.0, 251.5, 250.0, 245.75, 242.75, 243.5,
    ];
    println!("indicators version {}", VERSION);
    let mut candles: Vec<indicator::Candle> = Vec::new();
    for i in  0..closes.len() {
        let open: f64;
        if i == 0 {
            open = closes[i];
        } else {
            open = closes[i - 1];
        }
        let candle = indicator::Candle::new(i,open,closes[i],lows[i],highs[i]);
        candles.push(candle);
    }
    //indicator::print_candles(&candles);
    let start = Instant::now();
    for _ in 1..1000 {
        let _result: Vec<f64> = indicator::true_range_1(&candles);
    }
    let duration: Duration = start.elapsed();
    println!("Time elapsed in true_range_1 is {:?}", duration);
    
}
