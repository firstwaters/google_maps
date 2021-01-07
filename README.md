🗺 An unofficial Google Maps Platform client library for the Rust programming
language. This client currently implements the Directions API, Distance Matrix
API, Elevation API, Geocoding API, and Time Zone API.

![alt text](https://www.arkiteq.ca/crates/google_maps/banner.jpg "Unofficial Google Maps Platform Client for Rust")

# Welcome

This crate is expected to work well and have the more important Google Maps
features implemented. It should work well because Reqwest and Serde do most of
the heavy lifting! While it's an early release, this crate should work fine as
is for most people.

I created this client library because I needed several Google Maps Platform
features for a project that I'm working on. So, I've decided to spin my library
off into a public crate. This is a very small token of gratitude and an attempt
to give back to the Rust community. I hope it saves someone out there some work.

# Before You Begin

* In your project's `Cargo.toml` file, under the `[dependencies]` section:

	* Add `google_maps = "1.0.3"`. Check
		[crates.io](https://crates.io/crates/google_maps) for the latest
		version number.

	* Add `rust_decimal = "1.9.0"`. Check
		[crates.io](https://crates.io/crates/rust_decimal) for the
		latest version number.

* The full documentation is available at [docs.rs](https://docs.rs/google_maps/)

# What's new?

* 1.0.3: 2021-01-06: Updated dependencies. A few minor corrections. Async
support is planned for the next month or two.

* 1.0.2: 2020-08-07: Corrected error where string formatted for display were
being sent to the Google Maps Platform API. Thanks
[victorct-pronto](https://github.com/victorct-pronto)!

* 1.0.1: 2020-05-25: Ensuring all public structures use Serde's serialize and
deserialize traits. Thanks [qrayven](https://github.com/qrayven)!

* 1.0.0: 2020-05-16: Inteface stable.

* The full [change
log](https://github.com/leontoeides/google_maps/blob/master/CHANGELOG.md) is
available on GitHub.

## Example Directions API Request

```rust
use google_maps::prelude::*;
let mut google_maps_client = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);

// Example request:

let directions = google_maps_client.directions(
    // Origin: Canadian Museum of Nature
    Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
    // Destination: Canada Science and Technology Museum
    Location::LatLng(LatLng::try_from(dec!(45.403_509), dec!(-75.618_904)).unwrap()),
)
.with_travel_mode(TravelMode::Transit)
// Ensure this date is a weekday in the future or this query will return zero
// results.
.with_arrival_time(NaiveDate::from_ymd(2020, 3, 2).and_hms(13, 00, 0))
.execute();

// Dump entire response:

println!("{:#?}", directions);
```

## Example Distance Matrix API Request

```rust
use google_maps::prelude::*;
let mut google_maps_client = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);

// Example request:

let distance_matrix = google_maps_client.distance_matrix(
    // Origins
    vec![
        // Microsoft
        Waypoint::Address(String::from("One Microsoft Way, Redmond, WA 98052, United States")),
        // Cloudflare
        Waypoint::Address(String::from("101 Townsend St, San Francisco, CA 94107, United States")),
    ],
    // Destinations
    vec![
        // Google
        Waypoint::PlaceId(String::from("ChIJj61dQgK6j4AR4GeTYWZsKWw")),
        // Mozilla
        Waypoint::LatLng(LatLng::try_from(dec!(37.387_316), dec!(-122.060_008)).unwrap()),
    ],
)
.execute();

// Dump entire response:

println!("{:#?}", distance_matrix);
```

## Example Elevation API Positional Request

```rust
use google_maps::prelude::*;
let mut google_maps_client = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);

// Example request:

let elevation = google_maps_client.elevation()
    // Denver, Colorado, the "Mile High City"
    .for_positional_request(&LatLng::try_from(dec!(39.739_154), dec!(-104.984_703)).unwrap())
    .execute();

// Dump entire response:

println!("{:#?}", elevation);

// Parsing example:

println!("Elevation: {} meters", elevation.unwrap().results.unwrap()[0].elevation);
```

## Example Geocoding API Request

```rust
use google_maps::prelude::*;
let mut google_maps_client = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);

// Example request:

let location = google_maps_client.geocoding()
    .with_address("10 Downing Street London")
    .execute();

// Dump entire response:

println!("{:#?}", location);

// Parsing example:

for result in &location.unwrap().results {
    println!("{}", result.geometry.location)
}
```

## Example Reverse Geocoding API Request

```rust
use google_maps::prelude::*;
let mut google_maps_client = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);

// Example request:

let location = google_maps_client.reverse_geocoding(
    // 10 Downing St, Westminster, London
    LatLng::try_from(dec!(51.503_364), dec!(-0.127_625)).unwrap(),
)
.with_result_type(PlaceType::StreetAddress)
.execute();

// Dump entire response:

println!("{:#?}", location);

// Parsing example:

for result in &location.unwrap().results {
    for address_component in &result.address_components {
        print!("{} ", address_component.short_name);
    }
    println!(""); // New line.
}
```

## Example Time Zone API Request

```rust
use google_maps::prelude::*;
let mut google_maps_client = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);

// Example request:

let time_zone = google_maps_client.time_zone(
     // St. Vitus Cathedral in Prague, Czechia
     LatLng::try_from(dec!(50.090_903), dec!(14.400_512)).unwrap(),
     // The time right now in UTC (Coordinated Universal Time)
     Utc::now()
).execute().unwrap();

// Dump entire response:

println!("{:#?}", time_zone);

// Parsing example:

println!("Time at your computer: {}", Local::now().timestamp());

println!("Time in {}: {}",
    time_zone.time_zone_id.unwrap().name(),
    Utc::now().timestamp() + time_zone.dst_offset.unwrap() as i64 +
        time_zone.raw_offset.unwrap() as i64
);
```

## Example Client Settings

The Google Maps client settings can be used to change the request rate and
automatic retry parameters.

```rust
use google_maps::prelude::*;
use std::time::Duration;

let mut google_maps_client = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE)
    // For all Google Maps Platform APIs, the client will limit 2 sucessful
    // requests for every 10 seconds:
    .with_rate(Api::All, 2, Duration::from_secs(10))
    // For unsuccessful request attempts, the client will attempt 10 retries
    // before giving up:
    .with_max_retries(10)
    // For unsuccessful requests, the delay between retries is increased after
    // each attempt. This parameter ensures that the client will not delay for
    // more than 32 seconds between retries:
    .with_max_delay(Duration::from_secs(32))
    // Returns the `ClientSettings` struct to the caller. This struct is used to
    // make Google Maps Platform requests.
    .finalize();
```

## [Geolocation API](https://developers.google.com/maps/documentation/geolocation/intro)

Google's Geolocation API seems to be offline. While the online documentation
is still available and the API appears configurable through the Google Cloud
Platform console, the Geolocation API responds Status code `404 Not Found` with
an empty body to all requests. This API cannot be implemented until the server
responds as expected.

# Feedback

I would like for you to be successful with your project! If this crate is not
working for you, doesn't work how you think it should, or if you have requests,
or suggestions - please [report them to
me](https://github.com/leontoeides/google_maps/issues)! I'm not always fast at
responding but I will respond. Thanks!

# To do

1. Track both _requests_ and request _elements_ for rate limiting.
2. Make a generic get() function for that can be used by all APIs.
3. Considering move from `reqwest` to a lighter-weight HTTP client.
4. Look into making APIs optional, i.e. as features. Possible? Desirable?
5. Look into integrating [yaiouom](https://crates.io/crates/yaiouom).
6. Convert explicit query validation to session types wherever reasonable.
7. [Places API](https://developers.google.com/places/web-service/intro). There
are no immediate plans for supporting this API. It's quite big and I have no
current need for it. If you would like to have to implemented, please contact
me.
8. [Roads API](https://developers.google.com/maps/documentation/roads/intro).
There are no immediate plans for supporting this API. It's quite big and I have
no current need for it. If you would like to have to implemented, please contact
me.