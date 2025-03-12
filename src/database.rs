use std::env;
use mongodb::{Client};
use crate::models::Car;
use futures::stream::TryStreamExt;
use mongodb::error::Error;


async fn get_client() -> Result<Client, Error> {
    let connection_string = env::var("CONNECTION_STRING").expect("'CONNECTION_STRING' not specified in ENV");
    
    Client::with_uri_str(connection_string)
        .await
}

pub async fn get_cars() -> Result<Vec<Car>, Error> {
    let client = get_client().await.expect("Couldn't find client");
    let collection = client.database("rental_cars").collection::<Car>("cars");
    let mut cursor = collection.find(None, None).await.unwrap();

    let mut cars = Vec::new();
    while let Ok(Some(car)) = cursor.try_next().await {
        cars.push(car);
    }

    Ok(cars)
}

pub async fn add_car(car: &Car) -> Result<(), Error> {
    let client = get_client().await?;
    let collection = client.database("rental_cars").collection::<Car>("cars");
    collection.insert_one(car, None).await?;
    Ok(())
}