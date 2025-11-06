use cofj::{CoffeeEntry, Preparation, ProcessingMethod, Roast, save_entry_to_file};
use inquire::{Select, Text};
use std::process;

fn main() {
    println!("☕ Coffee Journal ☕\n");

    // Gather user input
    let brand = match Text::new("Brand Name:").prompt() {
        Ok(input) => input,
        Err(_) => {
            eprintln!("Error reading brand name");
            process::exit(1);
        }
    };

    let name = match Text::new("Coffee Name:").prompt() {
        Ok(input) => input,
        Err(_) => {
            eprintln!("Error reading coffee name");
            process::exit(1);
        }
    };

    let origin = match Text::new("Origin:").prompt() {
        Ok(input) => input,
        Err(_) => {
            eprintln!("Error reading origin");
            process::exit(1);
        }
    };

    // Processing method selection
    let processing_options = vec![
        "Anerobic Fermentation",
        "Natural",
        "Pulped Natural",
        "Washed",
    ];

    let processing_choice = match Select::new("Processing Method:", processing_options).prompt() {
        Ok(choice) => choice,
        Err(_) => {
            eprintln!("Error selecting processing method");
            process::exit(1);
        }
    };

    let processing = match processing_choice {
        "Anerobic Fermentation" => ProcessingMethod::AnerobicFermentation,
        "Natural" => ProcessingMethod::Natural,
        "Pulped Natural" => ProcessingMethod::PulpedNatural,
        "Washed" => ProcessingMethod::Washed,
        _ => unreachable!(),
    };

    // Roast selection
    let roast_options = vec!["Dark", "Light", "Medium"];

    let roast_choice = match Select::new("Roast:", roast_options).prompt() {
        Ok(choice) => choice,
        Err(_) => {
            eprintln!("Error selecting roast");
            process::exit(1);
        }
    };

    let roast = match roast_choice {
        "Dark" => Roast::Dark,
        "Light" => Roast::Light,
        "Medium" => Roast::Medium,
        _ => unreachable!(),
    };

    // Preparation selection (alphabetical order)
    let preparation_options = vec![
        "Americano",
        "Cortado",
        "Drip",
        "Espresso",
        "Latte",
        "Pourover",
    ];

    let preparation_choice = match Select::new("Preparation:", preparation_options).prompt() {
        Ok(choice) => choice,
        Err(_) => {
            eprintln!("Error selecting preparation");
            process::exit(1);
        }
    };

    let preparation = match preparation_choice {
        "Americano" => Preparation::Americano,
        "Cortado" => Preparation::Cortado,
        "Drip" => Preparation::Drip,
        "Espresso" => Preparation::Espresso,
        "Latte" => Preparation::Latte,
        "Pourover" => Preparation::Pourover,
        _ => unreachable!(),
    };

    // Rating selection
    let rating_options = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

    let rating_choice = match Select::new("Rating:", rating_options).prompt() {
        Ok(choice) => choice,
        Err(_) => {
            eprintln!("Error selecting rating");
            process::exit(1);
        }
    };

    let rating: u8 = rating_choice.parse().expect("Invalid rating");

    // Create the entry
    let entry = match CoffeeEntry::new(brand, name, origin, processing, roast, preparation, rating) {
        Ok(entry) => entry,
        Err(e) => {
            eprintln!("Error creating entry: {}", e);
            process::exit(1);
        }
    };

    // Save to file
    match save_entry_to_file(&entry, "cofj.txt") {
        Ok(_) => {
            println!("\n✓ Entry saved to cofj.txt");
        }
        Err(e) => {
            eprintln!("Error saving entry: {}", e);
            process::exit(1);
        }
    }
}
