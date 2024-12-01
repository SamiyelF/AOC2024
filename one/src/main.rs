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
    let url = "https://adventofcode.com/2024/day/1/input";
    let session = "im not telling :3";
    let contents: String;
    match get_input(url, session) {
        Ok(page) => contents=page,
        Err(_e) => contents="".to_string(),
    };
    let mut l: Vec<i32> = Vec::new();
    let mut r: Vec<i32> = Vec::new();
    let lines:Vec<String> = contents.lines().map(String::from).collect();
    for mut line in lines.clone() {
        if let Some(space_idx) = line.find(' ') {
            let right = line.split_off(space_idx);
            l.push(line.parse::<i32>().unwrap_or(0));
            r.push(right.trim().parse::<i32>().unwrap_or(0));
        }
    }
    l.sort();
    r.sort();
    let mut distances = 0;
    for (i,j) in l.iter().zip(r.iter()) {
        distances += (i-j).abs();
    }
    println!("Distance: {}",distances);
    let mut similarity = 0;
    for i in l{
        let count = r.iter().filter(|&n| *n == i).count() as i32;
        similarity += i * count;
    }
    println!("Similarity: {}", similarity);
}
/*explanation:
    First, we get the contents of the webpage.
    we split each index at the lines (23), and then split each list at the spaces (24-30)
    we sort the list, and add the distances.
    similarity is self explanitory
    easy peasy!
*/
