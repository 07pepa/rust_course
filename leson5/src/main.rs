use std::process::exit;
use std::env;
use std::collections::HashMap;
use base64::{Engine as _, engine::general_purpose};
use sha256::digest;
use std::error::Error;

use std::fs::File;
use std::io::Write;
use polars::prelude::CsvReader;
use tempfile::Builder;
use polars::prelude::SerReader;
use std::fs;
use std::path::PathBuf;



fn path_exist(path: &str) -> bool {
    match fs::metadata(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn csv_to_str(file:PathBuf)->Result<String, Box<dyn Error>>{
    return Ok(CsvReader::from_path(file)?.has_header(true).with_delimiter(b'|').finish()?.to_string());
}


fn load_csv(csv_data:String) -> Result<String, Box<dyn Error>>{
    // polars can handle any length of values and headers.... should yield a bonus point 
    // since polars do read entire input after csv param  technicaly you should be able to put multiline in "" in cmd and i am reading file if it exist should be enought to pass
    // another option would be to take it from std in but that would greatly differ from other "modes"
    if !path_exist(&csv_data){
        let temp_dir = Builder::new().tempdir()?;
        let temp_file: PathBuf = temp_dir.path().join("usr_input.csv").into();// yes i runned out of time
        let mut file = File::create(&temp_file)?;
        println!("{}",csv_data);
        file.write_all(csv_data.as_bytes())?;
        return csv_to_str(temp_file)
    }

    return csv_to_str((&csv_data).into())
 

}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()!=3 {
        println!("YOU USED IT BAD\nUsage is <mode> <your text in quotes if it has spaces>");
        exit(1);
    } 


    // i know this code is fugly ... fix is to create  some inner helper function to only accept str and fun
    let mut map: HashMap<&str, Box<dyn Fn(String) -> Result<String, Box<dyn Error>>>> = HashMap::new();
    map.insert("lowercase",Box::new(|x:String|Ok(x.to_lowercase())));
    map.insert("uppercase", Box::new(|x:String|Ok(x.to_uppercase())));
    map.insert("no-spaces", Box::new(|x:String|Ok( x.chars().filter(|&c| !c.is_whitespace()).collect())));
    map.insert("slugify",Box::new(|x:String| Ok(slug::slugify(x))));
    map.insert("base64", Box::new(|x:String| Ok(general_purpose::STANDARD_NO_PAD.encode(x.as_bytes()))));
    map.insert("sha256", Box::new(|x:String| Ok(digest(x))));
    map.insert("csv",Box::new(|x:String| load_csv(x)));

    

    let mode:String=args[1].to_lowercase();
    if !map.contains_key(&mode.as_str()){
        eprintln!("Unknown mode {mode}");
        exit(1)
    }
    
    let txt=&args[2];
    let result:Result<String, Box<dyn Error>>=map.get(&mode.as_str()).unwrap()(txt.to_string());// this cant fail or panic unlsess someone removes contains key code
    match result {
        Ok(v) => println!("{v}"),
        Err(e) => println!("Error occured:{e:?}"),
    }
    
 }