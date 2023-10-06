use regex::Regex;
use std::io;


fn main() {
    let mut triangle_sides = String::new();
    println!("Insert 3 whole numbers separated by ,");
    io::stdin()
        .read_line(&mut triangle_sides)
        .expect("something broke i can't read from terminal");
    // question!
    // this is simple enough to be compile time C can handle it (even when it needs whole statemachine)... why i cant use const?
    let matcher: Regex =
        Regex::new(r"\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*").expect("rex no workie ");
    let mut captures = matcher.captures(&triangle_sides);
    while captures.is_none() {
        println!("you screw up try again");
        println!("Insert 3 whole numbers separated by ,");
        io::stdin()
            .read_line(&mut triangle_sides)
            .expect("something broke i can't read from terminal");
        captures = matcher.captures(&triangle_sides);
    }
    let (_, [a_str, b_str, c_str]) = captures.unwrap().extract();
    let a = a_str.parse::<u64>().expect("broke");
    let b = b_str.parse::<u64>().expect("broke");
    let c = c_str.parse::<u64>().expect("broke");
    if a * a == (b * b + c * c) {
        println!("triangle is right angled")
    } else if a == b && a == c {
        println!("triangle is equilen")
    } else if a == b || b == c || c == a {
        println!("triangle has 2 sides same len")
    } else {
        println!("triangle is general triangle")
    }
}
