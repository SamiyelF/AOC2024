use regex::Regex;
use reqwest;
fn get_input(url: &str, session: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session))
        .send()?
        .text()?;
    Ok(response)
}
fn valid_char(c: char) -> bool {
    match c {
        '0'..='9' => true,
        'm' | 'u' | 'l' => true,
        '(' | ')' | ',' => true,
        _ => false,
    }
}
fn main() {
    let url = "https://adventofcode.com/2024/day/3/input";
    let session = "";

    let input: String = get_input(url, session).expect("No data found");
    let mut cleaned: String = input.clone();
    let mut removed = 0;
    for (i, c) in input.chars().enumerate() {
        if !valid_char(c) {
            cleaned.remove(i - removed);
            removed += 1;
        }
    }
    let getmuls = Regex::new(r"(?m)(mul\()[0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let funcs: Vec<&str> = getmuls
        .captures_iter(&cleaned)
        .map(|captures| captures.get(0).unwrap().as_str())
        .collect();
    let funcs_str = funcs.join(" ");
    let getvals = Regex::new(r"[0-9]{1,3}").unwrap();
    let vals: Vec<i128> = getvals
        .captures_iter(&funcs_str)
        .map(|captures| captures.get(0).unwrap().as_str().parse::<i128>().unwrap())
        .collect();

    let groups: Vec<(i128, i128)> = vals.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
    let mut mults: Vec<i128> = Vec::new();
    for pair in groups {
        mults.push(pair.0 * pair.1);
    }
    println!("{:?}", mults.iter().sum::<i128>());
}
