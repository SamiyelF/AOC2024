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
    let url = "https://adventofcode.com/2024/day/2/input";
    let session = "";

    let input: String = get_input(url, session).expect("No data found");
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|s| s.to_string())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect::<Vec<i32>>()
        })
        .collect();
    let mut difs: Vec<Vec<i32>> = Vec::new();
    for report in reports.clone() {
        let mut prev = 0;
        let mut current: Vec<i32> = Vec::new();
        for (i, val) in report.into_iter().enumerate() {
            if i == 0 {
                prev = val;
                continue;
            }
            current.push(val - prev);
            prev = val;
        }
        difs.push(current);
    }
    let mut valid = 0;
    for base in reports {
        print!("Checking sequence: {:?}...", base);

        if is_valid_sequence(&base) {
            println!("valid");
            valid += 1;
            continue;
        }

        // Try removing each number one at a time
        for i in 0..base.len() {
            let mut modified = base.clone();
            modified.remove(i);
            print!("removing {} at {}... ", base[i], i);
            if is_valid_sequence(&modified) {
                println!("validate by removing {} at position {}", base[i], i);
                println!("Modified sequence: {:?}", modified);
                valid += 1;
                println!("valid");
                break;
            }
        }
        println!("invalid\n");
    }
    println!("\n{} valid reports", valid);
}
fn is_valid_sequence(base: &[i32]) -> bool {
    if base.len() <= 1 {
        println!("empty");
        return false;
    }

    // Calculate differences
    let dif: Vec<i32> = base.windows(2).map(|w| w[1] - w[0]).collect();

    // Check if sorted
    let mut sorted = base.to_vec();
    sorted.sort();
    if sorted != base && sorted.iter().cloned().rev().collect::<Vec<_>>() != base {
        println!("out of order");
        return false;
    }

    // Check changes
    let max_change = dif.iter().map(|x| x.abs()).max().unwrap_or(0);
    let min_change = dif.iter().map(|x| x.abs()).min().unwrap_or(0);

    if max_change <= 3 && min_change >= 1 {
        return true;
    }
    println!("bad rate of change");
    return false;
}
