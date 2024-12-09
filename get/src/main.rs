use reqwest;
use std::{env::consts::OS, fs};
fn int_to_string(n: i32) -> String {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        21 => "twentyone",
        22 => "twentytwo",
        23 => "twentythree",
        24 => "twentyfour",
        25 => "twentyfive",
        _ => "NaN",
    }
    .to_string()
}
fn get_input(url: &str, session: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session))
        .send()?
        .text()?;
    Ok(response)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n = args
        .iter()
        .nth(1)
        .expect("what day is it?")
        .parse::<i32>()
        .expect("invalid number");
    println!("{}", int_to_string(n));
    let session = args.iter().nth(2).expect("what's your session id?");

    let home = format!("https://adventofcode.com/2024/day/{}", n);
    let input = format!("https://adventofcode.com/2024/day/{}/input", n);

    let description = get_input(&home, session).expect("No data found");
    let input = get_input(&input, session).expect("No data found");
    if OS == "linux" {
        std::env::set_current_dir("../").expect("failed to go up");
        std::process::Command::new("sh")
            .arg("-c")
            .arg("echo 'data recieved, making new project'")
            .output()
            .expect("Failed to execute command");
        std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("cargo new {}", int_to_string(n)))
            .output()
            .expect("failed to create project");

        fs::write(
            format!("{}/description.html", int_to_string(n)),
            description,
        )
        .expect("Unable to write file");
        fs::write(format!("{}/input", int_to_string(n)), input).expect("Unable to write file");
    }
}
