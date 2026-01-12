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
    // Your existing logic here...
    println!("Meditrack is running!");
}
use std::env;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;

const FILE_NAME: &str = "medications.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "add" => add_medication(&args),
        "list" => list_medications(),
        "take" => take_medication(&args),
        "help" => print_help(),
        _ => print_help(),
    }
}

fn add_medication(args: &[String]) {
    if args.len() < 5 {
        println!("Usage: meditrack add <name> <dose> <time>");
        return;
    }

    let entry = format!("{},{},{},NOT_TAKEN\n", args[2], args[3], args[4]);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILE_NAME)
        .expect("Cannot open file");

    file.write_all(entry.as_bytes()).unwrap();
    println!("Medication added.");
}

fn list_medications() {
    let content = read_to_string(FILE_NAME).unwrap_or_default();

    if content.is_empty() {
        println!("No medications found.");
        return;
    }

    println!("Your medications:");
    for (i, line) in content.lines().enumerate() {
        println!("{}: {}", i + 1, line);
    }
}

fn take_medication(args: &[String]) {
    if args.len() < 3 {
        println!("Usage: meditrack take <number>");
        return;
    }

    let index: usize = args[2].parse().unwrap_or(0);
    let content = read_to_string(FILE_NAME).unwrap_or_default();
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    if index == 0 || index > lines.len() {
        println!("Invalid medication number.");
        return;
    }

    lines[index - 1] = lines[index - 1].replace("NOT_TAKEN", "TAKEN");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_NAME)
        .unwrap();

    for line in lines {
        writeln!(file, "{}", line).unwrap();
    }

    println!("Medication marked as taken.");
}

fn print_help() {
    println!("Meditrack - Simple Medication Tracker");
    println!("Commands:");
    println!("  add <name> <dose> <time>");
    println!("  list");
    println!("  take <number>");
    println!("  help");
}
