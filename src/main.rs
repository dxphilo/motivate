use rand::Rng;
use serde::Deserialize;
use serde_json::Value;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug, Deserialize)]
pub struct Quote {
    quote: String,
    author: String,
}

fn main() {
    motivate();
}

fn motivate() {
    if let Some(file_path) = get_data_path() {
        let rand_filename = gen_rand();
        let rand_json = format!("{}.json", rand_filename);
        let full_path = file_path.join(rand_json);
        read_data_file(full_path);
    }
}

fn get_data_path() -> Option<PathBuf> {
    if let Ok(path_dir) = env::current_exe() {
        if path_dir == PathBuf::from("/opt/motivate/motivate") {
            Some(PathBuf::from("/opt/motivate/data/"))
        } else {
            Some(PathBuf::from("./data/"))
        }
    } else {
        eprintln!("Error getting current executable path");
        None
    }
}

fn read_data_file(path: PathBuf) {
    let file_contents_result = fs::read_to_string(path);

    // Check if reading the file was successful
    match file_contents_result {
        Ok(file_contents) => {
            let json_value: Value = serde_json::from_str(&file_contents).unwrap();
            get_quotes(json_value);
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}
// The function prints out the quote
pub fn get_quotes(json_value: Value) {
    if let Some(data_array) = json_value.get("data").and_then(Value::as_array) {
        let quotes: Result<Vec<Quote>, _> =
            serde_json::from_value(Value::Array(data_array.clone()));

        match quotes {
            Ok(quotes) => {
                let rand_num = rand::thread_rng().gen_range(0..=quotes.len() - 1);
                let rand_quote = &quotes[rand_num];
                print_formatted_quote(rand_quote);
            }
            Err(err) => {
                eprintln!("Error deserializing JSON: {}", err);
            }
        }
    } else {
        eprintln!("Error: 'data' field is missing or not an array");
    }
}

fn print_formatted_quote(quote: &Quote) {
    println!("\n");
    println!("{:^50}", format!("\"{}\"", quote.quote));

    println!("\n");
    println!("{:^50}", format!("-- {}", quote.author));
    println!("\n{:-^50}", "");
}

// This function will generate a random number between
// 001 and 180 for the file name to be read for the quotes
fn gen_rand() -> String {
    let random_num = rand::thread_rng().gen_range(1..=180);
    format!("{:03}", random_num)
}
