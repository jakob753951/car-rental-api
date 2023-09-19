use mongodb::{Client};
use crate::models::Car;
use futures::stream::TryStreamExt;
use mongodb::error::Error;


async fn get_client() -> Result<Client, Error> {
    let connection_string = "mongodb+srv://jakoblm:61lQ5Il8jf0n4BaO@cluster0.vh1fcbg.mongodb.net/?retryWrites=true&w=majority";
    Client::with_uri_str(connection_string)
        .await
}

pub async fn get_cars() -> Result<Vec<Car>, Error> {
    let client = get_client().await?;
    let collection = client.database("rental_cars").collection::<Car>("cars");
    let mut cursor = collection.find(None, None).await.unwrap();

    let mut cars = Vec::new();
    while let Ok(Some(car)) = cursor.try_next().await {
        cars.push(car);
    }

    Ok(cars)
}

pub async fn add_car(car: Car) -> Result<(), Error> {
    let client = get_client().await?;
    let collection = client.database("rental_cars").collection::<Car>("cars");
    collection.insert_one(car, None).await?;
    Ok(())
}