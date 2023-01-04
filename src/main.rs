use chrono::{DateTime, NaiveDateTime};
use reqwest::Response;

#[derive(Debug)]
enum WeatherCode {
    Clear,
    Cloudy,
    Overcast,
    Fog,
    Drizzle,
    Rainy,
    Snow,
    Showers,
    Thunderstorm,
}

impl std::fmt::Display for WeatherCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl WeatherCode {
    fn from_code(code: u64) -> Self {
        match code {
            0 | 1 => Self::Clear,
            2 => Self::Cloudy,
            3 => Self::Overcast,
            45 | 48 => Self::Fog,
            51..=57 => Self::Drizzle,
            61..=67 => Self::Rainy,
            71..=77 => Self::Snow,
            80..=86 => Self::Showers,
            95..=99 => Self::Thunderstorm,
            _ => {
                eprintln!("Unepected Weather Code {} received from api. \nProgram closing.", code);
                std::process::exit(1);
            }
        }
    }
}

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct ApiError {
    error: bool,
    reason: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

#[derive(serde::Deserialize)]
struct GeoLocation {
    lat: f32,
    lon: f32,
}

#[derive(Debug)]
struct CurrentWeather {
    temp: f64,
    feelslike: f64,
    windspeed: f64,
    weathercode: WeatherCode,
    time: u64,
}

impl CurrentWeather {
    fn new(temp: f64, feelslike: f64, windspeed: f64, weathercode: WeatherCode, time: u64) -> Self {
        Self {
            temp,
            feelslike,
            windspeed,
            weathercode,
            time,
        }
    }

    fn from_json(obj: serde_json::Value) -> Option<Self> {
        let current_weather = obj.get("current_weather")?;
        let daily = obj.get("daily")?;

        let feels_like = 0.5
        * (daily.get("apparent_temperature_max")?.as_array()?[0].as_f64()?
        + daily.get("apparent_temperature_min")?.as_array()?[0].as_f64()?);
        
        // this method is an error bomb
        Some(Self::new(
            current_weather.get("temperature")?.as_f64()?,
            feels_like,
            current_weather.get("windspeed")?.as_f64()?,
            WeatherCode::from_code(current_weather.get("weathercode")?.as_u64()?),
            current_weather.get("time")?.as_u64()?,
        ))
    }
}

impl std::fmt::Display for CurrentWeather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time: DateTime<chrono::Utc> = DateTime::from_utc(
            NaiveDateTime::from_timestamp_opt(self.time as i64, 0).unwrap(),
            chrono::Utc,
        );

        writeln!(f, " \x1b[1mCurrent Weather is:\x1b[0m ")?;
        writeln!(f, "     Temperature: {} °C ", self.temp)?;
        writeln!(f, "     Wind Speed: {} kmh", self.windspeed)?;
        writeln!(f, "     Conditions: {}", self.weathercode)?;
        writeln!(f, "     Today is going to feel like: {} °C", self.feelslike)?;
        writeln!(f, " \x1b[3mLast updated at: {}\x1b[0m", time)
    }
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let url = build_url().await?;
    let response = reqwest::get(&url).await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            print_weather(response).await?;
        }

        _ => {
            eprintln!(
                "Error Getting Weather from API. Error Code: {}",
                response.status()
            );
            eprintln!("URL used: {}", url);
            eprintln!("API Error Message: {}", response.json::<ApiError>().await?);
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn build_url() -> reqwest::Result<String> {
    let ip = public_ip::addr().await.unwrap_or_else(|| {
        eprintln!("Error getting ip address");
        std::process::exit(1);
    });

    let url = format!("http://ip-api.com/json/{ip}");

    let response = reqwest::get(url).await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let loc = response.json::<GeoLocation>().await?;

            let (lat, lon) = (loc.lat.to_string(), loc.lon.to_string());

            Ok(
                format!("https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&daily=apparent_temperature_max,apparent_temperature_min,sunset&current_weather=true&timeformat=unixtime&timezone=America%2FSao_Paulo")
            )
        }

        _ => {
            eprintln!(
                "Error Getting Location data from API: ip-api.com. Error Code: {}",
                response.status()
            );
            eprintln!(
                "Ensure a valid ip address was used. IP Address used: {}",
                ip
            );
            std::process::exit(1);
        }
    }


}

async fn print_weather(response: Response) -> reqwest::Result<()> {
    let info = response.json::<serde_json::Value>().await?; // TODO: describe the structure of this api response

    let current_weather = CurrentWeather::from_json(info);

    if let Some(curr) = current_weather {
        println!("{}", curr);
    } else {
        eprintln!("Catastrophic Failure");
        std::process::exit(1);
    }
    Ok(())
}
