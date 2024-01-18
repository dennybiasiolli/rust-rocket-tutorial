const MINUTE: u64 = 1_000 * 60;
const MINUTES_5: u64 = MINUTE * 5;
const MINUTES_15: u64 = MINUTE * 15;
const MINUTES_30: u64 = MINUTE * 30;
const MINUTES_45: u64 = MINUTE * 45;
const HOUR: u64 = MINUTE * 60;
const HOURS_2: u64 = HOUR * 2;
const HOURS_4: u64 = HOUR * 4;
const DAY: u64 = HOUR * 24;
const WEEK: u64 = DAY * 7;
const MONTH: u64 = DAY * 30;

#[derive(Debug)]
enum TimeUnit {
    Minute,
    Minutes5,
    Minutes15,
    Minutes30,
    Minutes45,
    Hour,
    Hours2,
    Hours4,
    Day,
    Week,
    Month,
}

// #[derive(Hash, Eq, PartialEq, Debug)]
#[derive(Debug)]
struct MyKey {
    symbol: String,
    timestamp: u64,
}

impl TimeUnit {
    fn value(&self) -> u64 {
        match *self {
            TimeUnit::Minute => MINUTE,
            TimeUnit::Minutes5 => MINUTES_5,
            TimeUnit::Minutes15 => MINUTES_15,
            TimeUnit::Minutes30 => MINUTES_30,
            TimeUnit::Minutes45 => MINUTES_45,
            TimeUnit::Hour => HOUR,
            TimeUnit::Hours2 => HOURS_2,
            TimeUnit::Hours4 => HOURS_4,
            TimeUnit::Day => DAY,
            TimeUnit::Week => WEEK,
            TimeUnit::Month => MONTH,
        }
    }
}

fn get_rounded_timestamp(source: u64, unit: TimeUnit) -> u64 {
    let value = unit.value();
    source / value * value
}

fn main() {
    let symbol = String::from("BTCUSDT");
    let milliseconds: u64 = 1705406188993;

    let key = MyKey {
        symbol: symbol.clone(),
        timestamp: milliseconds,
    };
    println!("Event:\n{:?}\n", key);

    let key = MyKey {
        symbol: symbol.clone(),
        timestamp: get_rounded_timestamp(milliseconds, TimeUnit::Minute),
    };
    println!("Rounded at the minute:\n{:?}\n", key);

    let key = MyKey {
        symbol: symbol.clone(),
        timestamp: get_rounded_timestamp(milliseconds, TimeUnit::Minutes5),
    };
    println!("Rounded at the 5 minutes:\n{:?}\n", key);

    let key = MyKey {
        symbol: symbol.clone(),
        timestamp: get_rounded_timestamp(milliseconds, TimeUnit::Minutes15),
    };
    println!("Rounded at the 15 minutes:\n{:?}\n", key);

    let key = MyKey {
        symbol: symbol.clone(),
        timestamp: get_rounded_timestamp(milliseconds, TimeUnit::Minutes30),
    };
    println!("Rounded at the 30 minutes:\n{:?}\n", key);

    let key = MyKey {
        symbol: symbol.clone(),
        timestamp: get_rounded_timestamp(milliseconds, TimeUnit::Minutes45),
    };
    println!("Rounded at the 45 minutes:\n{:?}\n", key);

    let key = MyKey {
        symbol: symbol.clone(),
        timestamp: get_rounded_timestamp(milliseconds, TimeUnit::Hour),
    };
    println!("Rounded at the hour:\n{:?}\n", key);

    let key = MyKey {
        symbol: symbol.clone(),
        timestamp: get_rounded_timestamp(milliseconds, TimeUnit::Hours2),
    };
    println!("Rounded at the 2 hours:\n{:?}\n", key);

    let key = MyKey {
        symbol: symbol.clone(),
        timestamp: get_rounded_timestamp(milliseconds, TimeUnit::Hours4),
    };
    println!("Rounded at the 4 hours:\n{:?}\n", key);

    let key = MyKey {
        symbol: symbol.clone(),
        timestamp: get_rounded_timestamp(milliseconds, TimeUnit::Day),
    };
    println!("Rounded at the day:\n{:?}\n", key);

    let key = MyKey {
        symbol: symbol.clone(),
        timestamp: get_rounded_timestamp(milliseconds, TimeUnit::Week),
    };
    println!("Rounded at the week:\n{:?}\n", key);

    let key = MyKey {
        symbol: symbol.clone(),
        timestamp: get_rounded_timestamp(milliseconds, TimeUnit::Month),
    };
    println!("Rounded at the month:\n{:?}\n", key);
}
