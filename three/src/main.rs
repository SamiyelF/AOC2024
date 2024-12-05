use regex::Regex;
use std::fs;
fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    let getmuls = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    let funcs: Vec<&str> = getmuls
        .captures_iter(&input)
        .map(|captures| captures.get(0).unwrap().as_str())
        .collect();
    let getvals = Regex::new(r"[0-9]{1,3}").unwrap();
    let mut vals: Vec<(i128, bool)> = Vec::new();
    let mut go = true;
    for func in funcs {
        if func == "do()" {
            go = true;
        } else if func == "don't()" {
            go = false;
        } else {
            let numbers: Vec<i128> = getvals
                .find_iter(func)
                .map(|m| m.as_str().parse::<i128>().unwrap())
                .collect();
            for num in numbers {
                vals.push((num, go));
            }
        }
    }

    let groups: Vec<((i128, bool), (i128, bool))> =
        vals.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
    let mut mults: Vec<i128> = Vec::new();
    let mut domults: Vec<i128> = Vec::new();
    for pair in groups {
        if pair.0 .1 {
            domults.push(pair.0 .0 * pair.1 .0);
        }
        mults.push(pair.0 .0 * pair.1 .0);
    }
    println!("{:?}", mults.iter().sum::<i128>());
    println!("{:?}", domults.iter().sum::<i128>());
}
