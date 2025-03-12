mod models;
mod database;
use models::Car;

use std::env;
use actix_cors::Cors;
use futures::future::join_all;

use actix_web::{get, middleware::Logger, web::Json, App, HttpServer, post};
use dotenv::dotenv;
use simple_logger::SimpleLogger;

#[get("/")]
async fn index() -> String {
    "Query /cars to get the available cars :)".to_string()
}

#[get("/ping")]
async fn ping() -> String {
    "pong!".to_string()
}

#[get("/cars")]
async fn get_cars() -> Json<Vec<Car>> {
    match database::get_cars().await {
        Ok(cars) => {
            Json(cars)
        }
        Err(_) => {
            Json(vec![])
        }
    }
}

#[post("/cars")]
async fn add_car(car: Json<Car>) -> String {
    match database::add_car(&car.into_inner()).await {
        Ok(_) => "Added successfully".to_string(),
        Err(e) => e.to_string()
    }
}

#[post("/cars/bulk")]
async fn add_cars(cars: Json<Vec<Car>>) -> String {
    let cars = cars.into_inner();
    let futures = cars
        .iter()
        .map(database::add_car);
    
    let results = join_all(futures).await;
    let successful_inserts = results.iter().filter(|res| res.is_ok()).count();
    
    match successful_inserts {
        0 => "No cars inserted".to_string(),
        _ => format!("Successfully inserted {successful_inserts} cars"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new().init().unwrap();
    dotenv().ok();
    
    let port = env::var("PORT")
        .unwrap_or("8000".to_string())
        .parse::<u16>()
        .expect("The PORT env variable should be an unsigned 16-bit integer");
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .service(index)
            .service(get_cars)
            .service(add_car)
            .service(add_cars)
        
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
