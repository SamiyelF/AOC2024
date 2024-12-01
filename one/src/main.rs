use reqwest;
fn get_input(url: &str, session: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session))
        .send()?.text()?;
    Ok(response)
}

fn main() {
    let url = "https://adventofcode.com/2024/day/1/input";
    let session = "";

    let (mut l, mut r): (Vec<i32>, Vec<i32>) = get_input(url,session).expect("No data found").lines().filter_map(|line| {
        let mut parts = line.split_whitespace();
        let left = parts.next()?.parse::<i32>().ok()?;
        let right = parts.next()?.parse::<i32>().ok()?;
        Some((left, right))
    }).unzip();
    l.sort();
    r.sort();

    let distances: i32 = l.iter().zip(r.iter()).map(|(a,b)| (a-b).abs()).sum();
    let similarity: i32 = l.iter().map(|&x| x*r.iter().filter(|&&n| n == x).count() as i32).sum();
    println!("Distance: {}",distances);
    println!("Similarity: {}", similarity);
}
