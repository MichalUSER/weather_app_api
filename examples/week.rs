use chrono::prelude::*;
use chrono::Duration;

fn main() -> Result<(), minreq::Error> {
    let api = weather_api::Api {
        url: "http://127.0.0.1:8000/".to_string(),
    };

    let temps = api.fetch_last_days(8)?;
    let averages = api.week_averages(temps);
    let now = Utc::now();

    for i in 0..8 {
        let day = now - Duration::days(i);
        let average = averages[i as usize];
        if average.is_nan() {
            continue;
        }
        println!("{}.{} average: {}°C", day.day(), day.month(), average);
    }

    Ok(())
}
