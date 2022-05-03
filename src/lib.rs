use chrono::prelude::*;
use chrono::Duration;

mod types;

use crate::types::{Average, Temp};

// TODO: make url take ip address struct
pub struct Api {
    pub url: String,
}

fn round(average: f32) -> f32 {
    let mut average = format!("{:.2}", average);
    if average.chars().last().unwrap() == '0' {
        average.pop();
    }

    average.parse::<f32>().unwrap()
}

impl Api {
    pub fn week_averages(&self, temps: Vec<Temp>) -> Vec<f32> {
        let mut averages: Vec<f32> = Vec::new();
        let now = Utc::now();
        for i in 0..8 {
            let day = now - Duration::days(i);
            let day_temps = temps.iter().filter(|t| t.d == day.day() as i32);
            let count = day_temps.clone().count() as f32;
            let average =
                day_temps.fold(0.0, |a, t| a + t.averageTemp.parse::<f32>().unwrap()) / count;
            if average.is_nan() {
                averages.push(average);
                continue;
            }
            averages.push(round(average));
        }
        averages
    }

    pub fn fetch_last(&self) -> Result<Temp, minreq::Error> {
        let response = minreq::get(format!("{}last_temp", self.url)).send()?;
        response.json()
    }

    pub fn fetch_average(&self, month: u32, day: u32) -> Result<Average, minreq::Error> {
        let response = minreq::get(format!("{}average/{}/{}", self.url, month, day)).send()?;
        response.json()
    }

    pub fn fetch_last_days(&self, day: u32) -> Result<Vec<Temp>, minreq::Error> {
        let response = minreq::get(format!("{}last_days/{}", self.url, day)).send()?;
        response.json()
    }
}

#[cfg(test)]
mod tests {
    use crate::Api;
    use chrono::prelude::*;
    use chrono::Duration;

    #[test]
    fn test_endpoints() -> Result<(), minreq::Error> {
        let api = Api {
            url: "http://127.0.0.1:8000/".to_string(),
        };
        let yesterday = Utc::now() - Duration::days(1);
        api.fetch_average(yesterday.month(), yesterday.day())?;
        api.fetch_last()?;
        api.fetch_last_days(8)?;
        Ok(())
    }
}
