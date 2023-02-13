pub const VERSION: &'static str = "v.0.0.1";
pub mod indicator {

    #[derive(Debug)]
    pub struct Candle {
        pub time: usize,
        pub open: f64,
        pub close: f64,
        pub low: f64,
        pub high: f64,
    }
    impl Default for Candle{
        fn default() -> Self{
            Self { time: 0, open: 0.0, close: 0.0, low: 0.0, high: 0.0 }
        }
    }
    impl Candle{
        pub fn new(time: usize,open:f64,close:f64,low:f64,high:f64) -> Self{
            Self { time,open,close,low,high }
        }

    }

    pub fn print_candles(candles: &[Candle]) {
        for candle in candles.iter() {
            println!("candle: {:?}", candle);
        }
    }
    pub fn true_range_1(candles: &[Candle]) -> Vec<f64> {
        let mut result: Vec<f64> = Vec::new();
        for i in 0..candles.len() {
            let mut values: [f64; 3] = [0.0;3];
            let true_range: f64;
            if i == 0 {
                true_range = f64::NAN;
            } else {
                values[0] = candles[i].high - candles[i].low;
                values[1] = (candles[i].high - candles[i - 1].close).abs();
                values[2] = (candles[i].low - candles[i - 1].close).abs();
                values.sort_by(|a,b| a.total_cmp(&b));
                true_range = values[2];
            }
            result.push(true_range);
        }
        result
    }
}
