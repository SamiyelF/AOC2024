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

fn main() {
    let url = "https://adventofcode.com/2024/day/3/input";
    let session = "";

    let input: String = get_input(url, session).expect("No data found");
    println!("{}", input);
}
