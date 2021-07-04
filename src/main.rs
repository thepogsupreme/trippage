#![allow(dead_code)]
#![allow(unused_variables)]

use colored::*;
use rand::Rng;
use std::error::Error;
use std::io;
use std::process;

static COLOR_ARRAY: [Color; 16] = [
    { Color::Black },
    { Color::Red },
    { Color::Green },
    { Color::Yellow },
    { Color::Blue },
    { Color::Magenta },
    { Color::Cyan },
    { Color::White },
    { Color::BrightBlack },
    { Color::BrightRed },
    { Color::BrightGreen },
    { Color::BrightYellow },
    { Color::BrightBlue },
    { Color::BrightMagenta },
    { Color::BrightCyan },
    { Color::BrightWhite },
];

fn main() {
    // Clear
    loop {
        std::process::Command::new("sh")
            .args(&["-c", "clear"])
            .output()
            .expect("Well shit ツ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let input = input.trim();

        if input.len() == 0 {
            let mut input = String::new();
            println!("You need to actually type something ツ");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");
            continue;
        } else {
            let input = input.trim();
            match input {
                "q" | "Q" => break,
                "generate" => {
                    if let Err(e) = rand_image() {
                        eprintln!("Application error: {}", e);
                        process::exit(1);
                    }
                }

                "help" => {
                    if let Err(e) = help_message() {
                        eprintln!("Application error: {}", e);
                        process::exit(1);
                    }
                }

                "seed" => {
                    println!("Feed me:");
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line.");

                    if let Err(e) = seed_image(input) {
                        eprintln!("Application error: {}", e);
                        process::exit(1);
                    }
                }

                "load" => {
                    println!("Feed me:");
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line.");

                    if let Err(e) = load_image(input) {
                        eprintln!("Application error: {}", e);
                        process::exit(1);
                    }
                }

                "export" => {
                    println!("Feed me:");
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line.");

                    if let Err(e) = export_image(input) {
                        eprintln!("Application error: {}", e);
                        process::exit(1);
                    }
                }

                _ => {
                    println!("Invalid selection.");
                    continue;
                }
            }
        }
    }

    std::process::Command::new("sh")
        .args(&["-c", "clear"])
        .output()
        .expect("Well shit ツ");
}

fn help_message() -> Result<(), Box<dyn Error>> {
    println!(
        "
{}      Returns this page :)
{}  Generate image
{}      Use a seed for generation
{}      Load an image
{}    Export image as PNG",
        "help".blue(),
        "generate".blue(),
        "seed".blue(),
        "load".blue(),
        "export".blue(),
    );
    Ok(())
}

fn rand_image() -> Result<(), Box<dyn Error>> {
    let pixel_count = (16, 16);

    let mut rand_seed;
    let mut image = String::new();

    for i in 0..pixel_count.0 {
        for j in 0..pixel_count.1 {
            rand_seed = rand::thread_rng().gen_range(0..16);
            image.push_str(&"██".color(COLOR_ARRAY[rand_seed]).to_string());
        }
        image.push('\n');
    }

    let image = image.trim();

    println!("{}{}{}", "\n\n", image, "\n\n");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    Ok(())
}

fn seed_image(_s: String) -> Result<(), Box<dyn Error>> {
    println!("Pepe");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    Ok(())
}

fn load_image(_s: String) -> Result<(), Box<dyn Error>> {
    println!("Pepe");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    Ok(())
}

fn export_image(_s: String) -> Result<(), Box<dyn Error>> {
    println!("Pepe");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    Ok(())
}
