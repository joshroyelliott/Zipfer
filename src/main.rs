#![allow(unused)]

use anyhow::{Context, Result};
use log::{error, info, trace, warn};
use std::collections::HashMap;
use std::io;
use std::time::Instant;

mod args;

use args::CliArgs;
use clap::Parser;

fn get_input(args: &CliArgs) -> Result<String> {
    let input = if let Some(filepath) = &args.path {
        let file = std::fs::read_to_string(&filepath)
            .with_context(|| format!("could not read file `{}`", filepath.display()))?;
        file
    } else {
        let stdin = io::read_to_string(io::stdin())?;
        stdin
    };
    Ok(input)
}

fn count_words(text: &str) -> Vec<(String, u32)> {
    let mut counts: HashMap<String, u32> = HashMap::new();

    // Split the text into words using whitespace and punctuation as delimiters
    let words = text
        .split(|c: char| !c.is_alphabetic())
        .filter(|s| !s.is_empty());

    let mut counter = 0;
    for word in words {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
        counter += 1;
    }
    // Convert the HashMap into a vector of tuples and return it
    let mut word_counts: Vec<(String, u32)> = counts.into_iter().collect();
    word_counts.sort_by(|a, b| b.1.cmp(&a.1)); // sort by count in descending order

    word_counts
}

fn main() -> Result<()> {
    env_logger::init();
    trace!("starting up");

    // Get the arguments passed
    let args = CliArgs::parse();
    trace!("passed args: {:?}", args);

    let start = Instant::now(); // start measuring time

    // Input is file if provided or stdin
    let input = match get_input(&args) {
        Ok(input) => {
            trace!("collected input");
            input
        }
        Err(error) => {
            error!("Error: {}", error);
            return Err(error);
        }
    };

    let counts = count_words(&input);

    let duration = start.elapsed();

    println!("{:<5}{:<15}{}", "No.", "Word", "Count");
    for (i, (word, count)) in counts.iter().enumerate().take(args.number) {
        println!("{:<5}{:<15}{}", i + 1, word, count);
    }

    Ok(())
}

// // #[test]
// fn count_some_words() {
//     let text = "The the the the quick quick quick brown brown fox.";
//     let word_counts = count_words(text);
//
//     for (word, count) in word_counts {
//         println!("{}: {}", word, count);
//     }
// }
