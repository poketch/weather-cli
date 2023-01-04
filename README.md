# Weather

This is a small project meant to create a cli weather app that presents you with basic weather information. 

Information is pulled from [Open-Meteo Weather Forecast API](https://open-meteo.com/)

IMPORTANT: The program uses the [public-ip crate](https://crates.io/crates/public-ip) to fetch your connections public ip and geolocates you through [Ip-Api](https://ip-api.com/).


## Installation 

```console
$ cargo install weather-cli
```

## Usage

```console 
$ weather

$ Current Weather is:
$    Temperature: 31.2 °C
$    Wind Speed: 13.6 kmh
$    Conditions: Overcast
$    Today is going to feel like: 29.85 °C
$ Last updated at: 2023-01-04 15:00:00 UTC
```

*Once the app's executable is in your path*

## Extensions

This was mostly a bite sized learning project but there are some extensions I would consider adding:

 - Feature Flags: 
    -h, --hourly : Prints information for the weather over the next 24 hours
    -d, --daily : Prints information for the current and next six days
    -f, --full : Prints hourly and daily information
    -n, --noip : Geolocates without looking for your public ip
    -l, --location : Prints the weather for a provided city/country (pref. without using google maps api fof geolocation)
 
 - More Robust Object Deserialization with serde

 - Reading Timezone from Open-Meteo and process it through Chrono to give the *"Last Updated"* in local time

 - Production Level Error Handling with [thiserror crate](https://crates.io/crates/thiserror)