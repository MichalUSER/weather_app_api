use chrono::prelude::*;
use chrono::Duration;

fn main() -> Result<(), minreq::Error> {
    let api = weather_api::Api {
        url: "http://127.0.0.1:8000/".to_string(),
    };
    let yesterday = Utc::now() - Duration::days(1);
    let average = api
        .fetch_average(yesterday.month(), yesterday.day())?
        .average;
    println!("Yesterday's average temperature: {}°C", average);

    Ok(())
}
