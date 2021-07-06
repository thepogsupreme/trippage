#![allow(dead_code)]
#![allow(unused_variables)]

mod perlin_noise;

use colored::*;
use core::f64::consts::PI;
use perlin_noise::*;
use rand::Rng;
use std::error::Error;
use std::io::{self, Write};

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

const SAMPLE_INTERVAL: f64 = 0.05489325;

pub fn help_message() -> Result<(), Box<dyn Error>> {
    println!(
        "
{}      Returns this page :)
{}  Generate image
{}      Use a seed for generation
{}      Load an image
{}    Export image as PNG
{}       Export image a GIF \n",
        "help".blue(),
        "generate".blue(),
        "seed".blue(),
        "load".blue(),
        "export".blue(),
        "gif".blue(),
    );

    println!("Press ENTER to return to menu...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}

pub fn rand_image(length: String, height: String) -> Result<(), Box<dyn Error>> {
    println!("Random image!");

    let length: usize = length.parse().unwrap_or(16);
    let height: usize = height.parse().unwrap_or(16);

    let seed_x = rand::thread_rng().gen_range(0..10000);
    let seed_y = rand::thread_rng().gen_range(0..10000);

    // println!("-----\n{}\n{}\n-----", seed_x, seed_y);

    let seed_x: f64 = (1.0 / seed_x as f64) * 10_f64.powf((seed_x.to_string().len() - 1) as f64);
    let seed_y: f64 = (1.0 / seed_y as f64) * 10_f64.powf((seed_y.to_string().len() - 1) as f64);

    // println!("-----\n{}\n{}\n-----", seed_x, seed_y);

    let mut image = String::new();

    let mut max_g = f64::MIN;
    let mut min_g = f64::MAX;

    for i in 0..height {
        let y = set_x_y(i, seed_y);

        for j in 0..length {
            let x = set_x_y(j, seed_x);
            let value = normalized_perlin(x, y);
            let prln_r = (((360.0 + 360.0 * value) * (PI / 180.0)).sin() + 1.0) * 0.5;
            let prln_g = (((240.0 + 360.0 * value) * (PI / 180.0)).sin() + 1.0) * 0.5;
            let prln_b = (((120.0 + 360.0 * value) * (PI / 180.0)).sin() + 1.0) * 0.5;

            let prln_r = prln_r * 256.0;
            let prln_g = prln_g * 256.0;
            let prln_b = prln_b * 256.0;

            if prln_g > max_g {
                max_g = prln_g;
            }
            if prln_g < min_g {
                min_g = prln_g;
            }

            image.push_str(
                &"██"
                    .truecolor(prln_r as u8, prln_g as u8, prln_b as u8)
                    .to_string(),
            );
        }

        image.push('\n');
    }

    let image = image.trim();

    println!("{}{}", image, "\n\n");

    println!("Press ENTER to return to menu...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}

pub fn seed_image(seed: String, length: String, height: String) -> Result<(), Box<dyn Error>> {
    let mut seed = seed.trim().to_string();
    io::stdout().flush().unwrap();
    let is_empty = seed.len() == 0;
    let length: usize = length.parse().unwrap_or(16);
    let height: usize = height.parse().unwrap_or(16);
    let mut seed_x;
    let mut seed_y;

    if is_empty {
        println!("Randomizing image!");
        seed_x = rand::thread_rng().gen_range(0..10000) as f64;
        seed_y = rand::thread_rng().gen_range(0..10000) as f64;
        seed.push_str(&seed_x.to_string());
        seed.push_str(&seed_y.to_string());
        seed_x = seed_x + 1.0 / seed_x;
        seed_y = seed_x;
    } else {
        println!("Seeding image!");
        //let seeds = seed.match_indices("").nth(seed.len()/2).map(|(index, _)| seed.split_at(index)).unwrap();
        seed_x = 1.0 / seed.parse().unwrap_or(420.0);
        seed_y = seed_x;
    }

    let mut image = String::new();

    let mut max_g = f64::MIN;
    let mut min_g = f64::MAX;

    for i in 0..height {
        let y = set_x_y(i, seed_y);

        for j in 0..length {
            let x = set_x_y(j, seed_x);
            let value = normalized_perlin(x, y);
            let prln_r = ((90.0 + 11.25 * value).sin() + 1.0) * 0.5;
            let prln_g = ((60.0 + 11.25 * value).sin() + 1.0) * 0.5;
            let prln_b = ((30.0 + 11.25 * value).sin() + 1.0) * 0.5;

            let prln_r = prln_r * 256.0;
            let prln_g = prln_g * 256.0;
            let prln_b = prln_b * 256.0;

            if prln_g > max_g {
                max_g = prln_g;
            }
            if prln_g < min_g {
                min_g = prln_g;
            }

            image.push_str(
                &"██"
                    .truecolor(prln_r as u8, prln_g as u8, prln_b as u8)
                    .to_string(),
            );
        }

        image.push('\n');
    }
    let image = image.trim();

    println!("\nMin G: {}\nMax G: {}", min_g, max_g);

    println!("{}{}", image, "\n\n");

    println!(
        "{}{}",
        if is_empty {
            "Randomly generated seed: "
        } else {
            "Your seed: "
        },
        seed
    );

    println!("Press Enter to return to menu...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    Ok(())
}

pub fn load_image(_s: String) -> Result<(), Box<dyn Error>> {
    println!("Press ENTER to return to menu...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}

pub fn export_image(_s: String) -> Result<(), Box<dyn Error>> {
    println!("Press ENTER to return to menu...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}

pub fn export_gif(_s: String) -> Result<(), Box<dyn Error>> {
    println!("Press ENTER to return to menu...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}

fn set_x_y(value: usize, seed: f64) -> f64 {
    (value as f64) * SAMPLE_INTERVAL + seed
}
