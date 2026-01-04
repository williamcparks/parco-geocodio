use std::time::Duration;

use crate::{GeocodeError, GeocodioClient};

#[tokio::test]
async fn test() {
    let _ = dotenvy::dotenv();

    let api_key = match std::env::var("GEOCODIO_API_KEY") {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!(
                "Did Not Run Geocodio Test Because `GEOCODIO_API_KEY` Could Not Be Retrieved From Environment Variables: {err}"
            );
            return;
        }
    };

    let client = reqwest::Client::new();

    let geocodio_client = GeocodioClient::builder()
        .client(client)
        .api_key(api_key.as_str())
        .build();

    let geocoded = geocodio_client
        .geocode()
        .address("1100 Congress Ave., Austin, TX 78701")
        .country("USA")
        .limit(1)
        .build()
        .send()
        .await
        .unwrap()
        .try_accurate()
        .unwrap();

    dbg!(geocoded);

    tokio::time::sleep(Duration::from_millis(1000)).await;

    let geocoded = geocodio_client
        .geocode()
        .address("invalid")
        .country("USA")
        .limit(1)
        .build()
        .send()
        .await;

    match geocoded {
        Ok(ok) => panic!("Geocode Expected To Fail With Api Error: {ok:#?}"),
        Err(GeocodeError::Api(api_err)) => {
            println!("Got Correct Api Error: {api_err}");
        }
        Err(err) => panic!("Geocode Failed For Wrong Reason Expected Api Error Got: {err}"),
    }
}
