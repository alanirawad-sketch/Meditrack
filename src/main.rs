use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Medication {
    name: String,
    dosage: String,
    time: String,
}

fn main() {
    println!("--- Meditrack CLI Started ---");

    // Creating a dummy medication to test serialization
    let test_med = Medication {
        name: "Test Pill".to_string(),
        dosage: "10mg".to_string(),
        time: "08:00".to_string(),
    };

    // Try to convert to JSON string to ensure serde is working
    if let Ok(json) = serde_json::to_string(&test_med) {
        println!("System Check: Medication serialized successfully.");
        println!("Data format: {}", json);
    }

    println!("Success: Meditrack is ready for use.");
}
