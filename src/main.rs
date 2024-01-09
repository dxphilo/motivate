use rand::Rng;
use serde::Deserialize;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Quote {
    quote: String,
    author: String,
}

fn main() {
    let argument = env::args().nth(1);

    if let Some(arg) = argument {
        if arg == "motivate" {
            motivate()
        }
    }
}

fn motivate() {
    let rand_filename = gen_rand();
    let file_name = format!("./data/{}.json", rand_filename);

    let quotes_path = Path::new(&file_name);

    // Read file contents as a string
    let file_contents_result = fs::read_to_string(&quotes_path);

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
                println!("\n");
                println!("{: ^50}", rand_quote.quote);
                println!("\n");
                println!("{:-^50}", rand_quote.author);
                println!("\n");
            }
            Err(err) => {
                eprintln!("Error deserializing JSON: {}", err);
            }
        }
    } else {
        eprintln!("Error: 'data' field is missing or not an array");
    }
}

// This function will generate a random number between
// 001 and 180 for the file name to be read for the quotes
fn gen_rand() -> String {
    let random_num = rand::thread_rng().gen_range(1..=180);
    return format!("{:03}", random_num);
}
