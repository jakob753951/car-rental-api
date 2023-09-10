mod models;
use models::{Car, CarType};

use actix_web::{get, middleware::Logger, web::Json, App, HttpServer};
use simple_logger::SimpleLogger;

#[get("/")]
async fn index() -> String {
    "Hello World!".to_string()
}

#[get("/cars")]
async fn get_cars() -> Json<Vec<Car>> {
    Json(vec![
        Car {
            make: "Toyota".to_string(),
            model: "Camry".to_string(),
            year: 2023,
            car_type: CarType::Sedan,
            price_per_day: 60.99,
            location: "Los Angeles".to_string(),
            available: true,
        },
        Car {
            make: "Ford".to_string(),
            model: "Explorer".to_string(),
            year: 2022,
            car_type: CarType::SUV,
            price_per_day: 75.50,
            location: "New York".to_string(),
            available: true,
        },
        Car {
            make: "Honda".to_string(),
            model: "Civic".to_string(),
            year: 2023,
            car_type: CarType::Compact,
            price_per_day: 55.75,
            location: "Chicago".to_string(),
            available: false,
        },
        Car {
            make: "Chevrolet".to_string(),
            model: "Tahoe".to_string(),
            year: 2022,
            car_type: CarType::SUV,
            price_per_day: 80.25,
            location: "Miami".to_string(),
            available: true,
        },
        Car {
            make: "Nissan".to_string(),
            model: "Altima".to_string(),
            year: 2023,
            car_type: CarType::Sedan,
            price_per_day: 63.00,
            location: "San Francisco".to_string(),
            available: true,
        },
        Car {
            make: "Volkswagen".to_string(),
            model: "Jetta".to_string(),
            year: 2019,
            car_type: CarType::Sedan,
            price_per_day: 45.50,
            location: "Dallas".to_string(),
            available: true,
        },
        Car {
            make: "Hyundai".to_string(),
            model: "Tucson".to_string(),
            year: 2018,
            car_type: CarType::SUV,
            price_per_day: 52.75,
            location: "Denver".to_string(),
            available: true,
        },
    ])
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new().init().unwrap();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_cars)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}