#![allow(dead_code)]
#![allow(unused_variables)]

mod perlin_noise;

use colored::*;
use perlin_noise::*;
use rand::Rng;
use std::error::Error;
use std::io;

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

pub fn help_message() -> Result<(), Box<dyn Error>> {
    println!(
        "
{}      Returns this page :)
{}  Generate image
{}      Use a seed for generation
{}      Load an image
{}    Export image as PNG\n",
        "help".blue(),
        "generate".blue(),
        "seed".blue(),
        "load".blue(),
        "export".blue(),
    );

    println!("Press ENTER to return to menu...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    Ok(())
}

pub fn rand_image(length: String, height: String) -> Result<(), Box<dyn Error>> {
    println!("Random image!");

    let length: usize = length.parse().unwrap_or(16);
    let height: usize = height.parse().unwrap_or(16);

    let seed_x = rand::thread_rng().gen_range(0..10000);
    let seed_y = rand::thread_rng().gen_range(0..10000);

    println!("-----\n{}\n{}\n-----", seed_x, seed_y);

    let seed_x: f64 = (1.0 / seed_x as f64) * 10_f64.powf((seed_x.to_string().len() - 1) as f64);
    let seed_y: f64 = (1.0 / seed_y as f64) * 10_f64.powf((seed_y.to_string().len() - 1) as f64);

    println!("-----\n{}\n{}\n-----", seed_x, seed_y);

    let mut image = String::new();

    for i in 0..height {
        for j in 0..length {
            let prln = perlin((j as f64) * 31415.9 + seed_x, (i as f64) * 31415.9 + seed_y);
            let mut prln = (prln * 1.0) + 0.5;
            if prln > 1.0 {
                prln = prln - 1.0;
            } else if prln < 0.0 {
                prln = 1.0 + prln;
            }
            let pixel = (prln * 16.0) as usize;
            image.push_str(&"██".color(COLOR_ARRAY[pixel]).to_string());
        }
        image.push('\n');
    }
    let image = image.trim();

    println!("{}{}", image, "\n\n");

    println!("Press ENTER to return to menu...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    Ok(())
}

pub fn seed_image(seed: String, length: String, height: String) -> Result<(), Box<dyn Error>> {
    println!("Seeding image!");

    let length: usize = length.parse().unwrap_or(16);
    let height: usize = height.parse().unwrap_or(16);

    let seed = seed
        .match_indices("")
        .nth(4)
        .map(|(index, _)| seed.split_at(index))
        .unwrap();

    let seed_x: f64 =
        (1.0 / seed.0.parse().unwrap_or(420.0)) * 10_f64.powf((seed.0.len() - 1) as f64);
    let seed_y: f64 =
        (1.0 / seed.1.parse().unwrap_or(6969.0)) * 10_f64.powf((seed.1.len() - 1) as f64);

    // println!("-----\n{}\n{}\n-----", seed_x, seed_y);

    let mut image = String::new();

    for i in 0..height {
        for j in 0..length {
            let prln = perlin((j as f64) * 500.0 + seed_x, (i as f64) * 500.0 + seed_y);
            let mut prln = (prln * 1.0) + 0.5;
            if prln > 1.0 {
                prln = prln - 1.0;
            } else if prln < 0.0 {
                prln = 1.0 + prln;
            }
            let pixel = (prln * 16.0) as usize;
            image.push_str(&"██".color(COLOR_ARRAY[pixel]).to_string());
        }
        image.push('\n');
    }
    let image = image.trim();

    println!("{}{}", image, "\n\n");

    println!("Press ENTER to return to menu...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    Ok(())
}

pub fn load_image(_s: String) -> Result<(), Box<dyn Error>> {
    println!("Press ENTER to return to menu...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    Ok(())
}

pub fn export_image(_s: String) -> Result<(), Box<dyn Error>> {
    println!("Press ENTER to return to menu...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    Ok(())
}
