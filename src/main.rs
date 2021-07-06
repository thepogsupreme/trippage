#![allow(dead_code)]
#![allow(unused_variables)]

use colored::*;
use std::io;
use std::io::Write;
use std::process;

use trippage::*;

fn main() -> std::io::Result<()> {
    loop {
        print!("\x1b[2J\x1b[H"); // Clears the screen

        print!("{} {} ", "trippagectl".cyan(), "➜".green());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.len() == 0 {
            let mut input = String::new();
            println!("You need to actually type something ツ");
            io::stdin().read_line(&mut input)?;
            continue;
        } else {
            let input = input.trim();
            match input {
                "q" | "Q" => break,
                "help" => {
                    if let Err(e) = help_message() {
                        eprintln!("Application error: {}", e);
                        process::exit(1);
                    }
                }

                "generate" => {
                    // Run function to a create random image, and if an error is output (propagated from function), run code with error message
                    println!("Enter the desired image length and height (separated by a space):");
                    let mut dimensions = String::new();
                    io::stdin().read_line(&mut dimensions)?;
                    let dimensions = dimensions
                        .match_indices(' ')
                        .nth(0)
                        .map(|(index, _)| dimensions.split_at(index))
                        .unwrap();
                    let input2 = dimensions.0.trim().to_string();
                    let input3 = dimensions.1.trim().to_string();
                    if let Err(e) = rand_image(input2, input3) {
                        eprintln!("Application\t error: {}", e);
                        process::exit(1);
                    }
                }

                "seed" => {
                    println!("Enter the desired seed to render:");
                    let mut input1 = String::new();
                    io::stdin().read_line(&mut input1)?;
                    println!("Enter the desired image length and height (separated by a space):");
                    let mut dimensions = String::new();
                    io::stdin().read_line(&mut dimensions)?;
                    let dimensions = dimensions
                        .match_indices(' ')
                        .nth(0)
                        .map(|(index, _)| dimensions.split_at(index))
                        .unwrap();
                    let input2 = dimensions.0.trim().to_string();
                    let input3 = dimensions.1.trim().to_string();

                    //Run function to create an image from a user-provided seed, and if an error is output (propagated from function), run code with error message
                    if let Err(e) = seed_image(input1.trim().to_string(), input2, input3) {
                        eprintln!("Application\t error: {}", e);
                        process::exit(1);
                    }
                }

                "load" => {
                    println!("Feed me:");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;

                    if let Err(e) = load_image(input) {
                        eprintln!("Application error: {}", e);
                        process::exit(1);
                    }
                }

                "export" => {
                    println!("Feed me:");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;

                    if let Err(e) = export_image(input) {
                        eprintln!("Application error: {}", e);
                        process::exit(1);
                    }
                }

                "gif" => {
                    println!("Feed me:");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;

                    if let Err(e) = export_gif(input) {
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

    print!("\x1b[2J\x1b[H");

    Ok(())
}
