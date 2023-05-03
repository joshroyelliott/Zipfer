// TODO: add regex parsing
// TODO: add character sequence parsing

#![allow(unused)]

use anyhow::{anyhow, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info, trace, warn};
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::io;
use std::process;
use std::time::Instant;

mod args;

use args::CliArgs;
use clap::Parser;
use clap_verbosity_flag::Verbosity;

fn get_input(args: &CliArgs) -> Result<String> {
    // Input is file if provided or stdin
    let input = if let Some(filepath) = &args.path {
        std::fs::read_to_string(&filepath)?
    } else {
        io::read_to_string(io::stdin())?
    };
    Ok(input)
}

fn tokenize_input(text: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // Split the text into words using whitespace and punctuation as delimiters
    let tokens: Vec<(String)> = text
        .split(|c: char| !c.is_alphabetic())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    Ok(tokens)
}

fn count_words(text: &str, args: &CliArgs) -> Result<Vec<(String, u32)>, Box<dyn Error>> {
    let start = Instant::now(); // start measuring time

    // Get tokens
    let tokens = tokenize_input(text)?;

    let mut total = 0;
    let mut counts: HashMap<String, u32> = HashMap::new();
    // Accumulation of counts from tokens

    // Create a progress bar if the verbose flag is set
    let progress_bar = if args.verbose.log_level_filter() >= log::LevelFilter::Info {
        let progress_bar = ProgressBar::new(tokens.len() as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} {bar:40.cyan/blue} {pos}/{len} {elapsed_precise}")?
                .progress_chars("=> "),
        );
        Some(progress_bar)
    } else {
        None
    };

    for token in &tokens {
        *counts.entry(token.to_lowercase()).or_insert(0) += 1;

        // Update progress bar if it exists
        if let Some(pb) = &progress_bar {
            pb.inc(1);
        }
    }

    // Convert the HashMap into a vector of tuples and return it
    let mut word_counts: Vec<(String, u32)> = counts.into_iter().collect();
    word_counts.sort_by(|a, b| b.1.cmp(&a.1)); // sort by count in descending order

    let duration = start.elapsed();

    // Print verbose output if the verbose flag is set
    if let Some(pb) = progress_bar {
        pb.finish_with_message(format!(
            "Finished counting {} tokens in {:.2}s",
            tokens.len(),
            duration.as_secs_f32()
        ));
    }
    if args.verbose.log_level_filter() >= log::LevelFilter::Info {
        println!("Total tokens: {}", tokens.len());
        println!("Time taken: {:.2}s", duration.as_secs_f32());
    }

    Ok(word_counts)
}

fn main() -> Result<()> {
    env_logger::init();
    trace!("starting up");

    // Get the arguments passed
    let args = CliArgs::parse();
    trace!("passed args: {:?}", args);

    // Get input from args
    let input = get_input(&args)?;
    // Count words based on args
    let counts = count_words(&input, &args).unwrap();

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
