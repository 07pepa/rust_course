use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;

use std::path::PathBuf;
use std::process::exit;

use base64::{engine::general_purpose, Engine as _};
use ctrlc;
use flume::{Receiver, Sender};
use polars::prelude::CsvReader;
use polars::prelude::SerReader;
use sha256::digest;

fn path_exist(path: &str) -> bool {
    match fs::metadata(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn csv_to_str(file: PathBuf) -> Result<String, Box<dyn Error>> {
    return Ok(CsvReader::from_path(file)?
        .has_header(true)
        .with_delimiter(b'|')
        .finish()?
        .to_string());
}

fn load_csv(csv_data: String) -> Result<String, Box<dyn Error>> {
    // reqirements does not say if must interpret is as path always     (and i already have code)
    if !path_exist(&csv_data) {
        return Err("Path does not exist!".into());
    }

    return csv_to_str((&csv_data).into());
}

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = flume::unbounded();
    //JUST FYI  reqirements  say one thread read from stdin so i will only read from std in  (to kill 2 birds with one stone)
    // later it says If you're looking for an additional challenge, implement a mechanism where the program enters the interactive mode only when there are no CLI arguments provided. If arguments are given, the program should operate in the previous way, processing the command directly.*/
    //technicaly this mean that arg functionality does not have to be preserved and from usual linux notion (one way to operate things)
    //and you set modes with cli switches or get path but you redirect  input usual to std::in .. so to keep this notion i removed parameters
    // and continualy reading from std in but waiting for line to finish and then pass it to another thread

    let suplier_reader = std::thread::spawn(move || loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            match tx.send(input) {
                Err(_) => break,
                Ok(_) => continue,
            }
        } else {
            break;
        }
    });

    let consumer = std::thread::spawn(move || {
        // hehe map keys set is basicly a enum in my case and its easy to extend
        let functions: HashMap<&str, Box<dyn Fn(String) -> Result<String, Box<dyn Error>>>> = {
            let mut map: HashMap<&str, Box<dyn Fn(String) -> Result<String, Box<dyn Error>>>> =
                HashMap::new();
            map.insert("lowercase", Box::new(|x: String| Ok(x.to_lowercase())));
            map.insert("uppercase", Box::new(|x: String| Ok(x.to_uppercase())));
            map.insert(
                "no-spaces",
                Box::new(|x: String| Ok(x.chars().filter(|&c| !c.is_whitespace()).collect())),
            );
            map.insert("slugify", Box::new(|x: String| Ok(slug::slugify(x))));
            map.insert(
                "base64",
                Box::new(|x: String| Ok(general_purpose::STANDARD_NO_PAD.encode(x.as_bytes()))),
            );
            map.insert("sha256", Box::new(|x: String| Ok(digest(x))));
            map.insert("csv", Box::new(|x: String| load_csv(x)));
            map
        };

        loop {
            match rx.recv() {
                Ok(input) => {
                    let parts: Vec<&str> = input.splitn(2, ' ').collect();
                    if parts.len() != 2 {
                        let crlf_fix = input.replace("\n", "");
                        eprintln!("unknown input '{crlf_fix}'");
                        continue;
                    }
                    let mode: String = parts[0].to_string().to_lowercase();
                    if !functions.contains_key(mode.as_str()) {
                        eprintln!("Unknown mode {}", parts[0].to_string());
                        continue;
                    }

                    let result: Result<String, Box<dyn Error>> =
                        functions.get(mode.as_str()).unwrap()(parts[1].to_string()); // this cant fail or panic unlsess someone removes contains key code
                    match result {
                        Ok(v) => println!("{v}"),
                        Err(e) => println!("Error occured: {e:?}"),
                    }
                }
                Err(_) => break, // this requires unbounded queue
            }
        }
    });

    let _ = ctrlc::set_handler(move || {
        exit(0) // yes i know this is ugly hack but makes program not panic
                //and OS must free our memory when we die so any mess consumer and suplier will do will be freed there
                // ao its safe
    });

    suplier_reader.join().expect("Read thread panicked");
    consumer.join().expect("compute thread panicked");
    // this means we have main thread and 2 others
}
