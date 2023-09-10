use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CarType {
    Sedan,
    SUV,
    Compact,
    Truck,
}

#[derive(Serialize, Deserialize)]
pub struct Car {
    pub make: String,
    pub model: String,
    pub year: u32,
    pub car_type: CarType,
    pub price_per_day: f64,
    pub location: String,
    pub available: bool,
}

// CarType to String conversion
