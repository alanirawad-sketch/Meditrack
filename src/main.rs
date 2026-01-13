use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Parser)]
#[command(name = "meditrack")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { name: String, dosage: String, time: String },
    List,
    Take { id: usize },
}

#[derive(Serialize, Deserialize, Debug)]
struct Medicine {
    id: usize,
    name: String,
    dosage: String,
    time: String,
    taken: bool,
}

fn main() {
    let cli = Cli::parse();
    let file_path = "meds.json";
    
    let mut inventory: Vec<Medicine> = fs::read_to_string(file_path)
        .map(|c| serde_json::from_str(&c).unwrap_or_default())
        .unwrap_or_default();

    match &cli.command {
        Commands::Add { name, dosage, time } => {
            let id = inventory.len() + 1;
            inventory.push(Medicine { id, name: name.clone(), dosage: dosage.clone(), time: time.clone(), taken: false });
            save_inventory(file_path, &inventory);
            println!("✅ Added {}!", name);
        }
        Commands::List => {
            println!("{:<5} {:<15} {:<10} {:<10}", "ID", "NAME", "DOSAGE", "STATUS");
            for m in &inventory {
                let status = if m.taken { "✅ Taken" } else { "⏳ Pending" };
                println!("{:<5} {:<15} {:<10} {:<10}", m.id, m.name, m.dosage, status);
            }
        }
        Commands::Take { id } => {
            let mut found = false;
            // 1. Find and update the item
            for m in inventory.iter_mut() {
                if m.id == *id {
                    m.taken = true;
                    println!("💊 Marked {} as taken!", m.name);
                    found = true;
                    break;
                }
            }
            
            // 2. Save the inventory AFTER the update loop is finished
            if found {
                save_inventory(file_path, &inventory);
            } else {
                println!("❌ ID {} not found.", id);
            }
        }
    }
}

// Helper function to handle saving without borrow issues
fn save_inventory(path: &str, inventory: &Vec<Medicine>) {
    let data = serde_json::to_string_pretty(inventory).unwrap();
    fs::write(path, data).expect("Unable to save data");
}
