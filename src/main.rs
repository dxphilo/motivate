use include_dir::{include_dir, Dir};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;

static DATA_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/data");

#[derive(Debug, Deserialize)]
struct Quote {
    quote: String,
    author: String,
}

#[derive(Debug, Deserialize)]
struct QuotesFile {
    data: Vec<Quote>,
}

fn main() {
    if let Err(e) = motivate() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn motivate() -> Result<(), Box<dyn std::error::Error>> {
    let files: Vec<_> = DATA_DIR.files().collect();

    if files.is_empty() {
        return Err("No quote files found in embedded data".into());
    }

    let file = files
        .choose(&mut thread_rng())
        .expect("files vector is non-empty");

    let contents = file
        .contents_utf8()
        .ok_or("Embedded file is not valid UTF-8")?;

    let quotes_file: QuotesFile = serde_json::from_str(contents)?;

    let quotes = quotes_file.data;

    if quotes.is_empty() {
        return Err("Selected quote file contains no quotes".into());
    }

    let quote = quotes
        .choose(&mut thread_rng())
        .expect("quotes vector is non-empty");

    print_formatted_quote(quote);

    Ok(())
}

fn print_formatted_quote(quote: &Quote) {
    println!("\n");
    println!("{:^60}", format!("\"{}\"", quote.quote));
    println!("\n");
    println!("{:^60}", format!("-- {}", quote.author));
    println!("\n{:-^60}", "");
}