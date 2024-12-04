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
    let url = "https://adventofcode.com/2024/day/2/input";
    let session = "";

    let input: String = get_input(url,session).expect("No data found");
    let reports: Vec<Vec<i32>> = input.lines().map(|s| s.to_string())
        .map(|line| line.split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect::<Vec<i32>>())
        .collect();
    let mut difs: Vec<Vec<i32>> = Vec::new();
    for report in reports.clone() {
        let mut prev = 0;
        let mut current: Vec<i32> = Vec::new();
        for (i,val) in report.into_iter().enumerate() {
            if i == 0 {
                prev = val;
                continue;
            }
            current.push(val-prev);
            prev = val;
        }
        difs.push(current);
    }
    let mut valid = 0;
    for (dif,base) in difs.clone().into_iter().zip(reports) {
        let mut sorted = base.clone();
        sorted.sort();
        if sorted != base && sorted.iter().rev().cloned().collect::<Vec<i32>>() != base {
            println!("{:?} is not in order", base);
            continue;
        } else if !(dif.iter().map(|x| x.abs()).max().unwrap_or(0) <= 3) {
            println!("{:?} changes too fast",dif);
            continue;
        } else if !(dif.iter().map(|x| x.abs()).min().unwrap_or(0) >= 1) {
            println!("{:?} changes too slow",dif);
            continue;
        } else {
            println!("{:?} is valid", dif);
            valid += 1;
        }
    }
    println!("{} valid reports",valid);
}
