use std::process::exit;
use std::env;
use std::collections::HashMap;
use base64::{Engine as _, engine::general_purpose};
use sha256::digest;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()!=3 {
        println!("YOU USED IT BAD\nUsage is <mode> <your text in quotes if it has spaces>");
        exit(1);
    } 

    // yes i can use switch statement but i wanted to try to create constant LUT for str->function this is as close i could get
    // in other program i would create helper function to help me add it or staticaly initiate it (of course with checking that i did not overide it)
    //this should be able to be done staticaly since after initialization is is just array of str hashes and pointer to function so just array
    // how to do it during compile time?
    let mut map: HashMap<&str, Box<dyn Fn(String) -> String>> = HashMap::new();
    map.insert("lowercase",Box::new(|x:String|x.to_lowercase()));
    map.insert("uppercase", Box::new(|x:String|x.to_uppercase()));
    map.insert("no-spaces", Box::new(|x:String| x.chars().filter(|&c| !c.is_whitespace()).collect()));
    map.insert("slugify",Box::new(|x:String| slug::slugify(x)));
    map.insert("base64", Box::new(|x:String| general_purpose::STANDARD_NO_PAD.encode(x.as_bytes())));
    map.insert("sha256", Box::new(|x:String| digest(x)));

    let mode:String=args[1].to_lowercase();
    if !map.contains_key(&mode.as_str()){
        println!("Unknown mode {mode}");
        exit(1)
    }
    
    let txt=&args[2];
    let me_out=map.get(&mode.as_str()).unwrap()(txt.to_string());
    println!("{}",me_out);

 }