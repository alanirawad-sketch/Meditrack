use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Medication {
    name: String,
    dosage: String,
    time: String,
}

fn main() {
    println!("--- Welcome to Meditrack ---");
    
    // Simple example logic to ensure it compiles and works
    let med = Medication {
        name: String::from("Vitamin C"),
        dosage: String::from("500mg"),
        time: String::from("08:00"),
    };

    println!("Tracking: {} ({}) at {}", med.name, med.dosage, med.time);
    
    // This creates a data folder if it doesn't exist
    let _ = fs::create_dir_all("data");
    
    println!("Setup complete. Ready to automate your health!");
}
