fn main() -> Result<(), minreq::Error> {
    let api = weather_api::Api {
        url: "http://127.0.0.1:8000/".to_string(),
    };
    println!(
        "Last measured temperature: {}°C",
        api.fetch_last()?.averageTemp
    );

    Ok(())
}
