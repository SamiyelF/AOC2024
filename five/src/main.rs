use std::fs;
fn insert(input: &mut Vec<i32>, find: i32, place: i32, dir: bool) {
    if let Some(pos) = input.iter().position(|x| *x == find) {
        let insert_pos = if dir { pos + 1 } else { pos };
        input.insert(insert_pos, place);
    }
    input.dedup();
}
fn is_in_relative_order(subset: &[i32], order: &[i32]) -> bool {
    let mut positions: Vec<i32> = Vec::new();
    for i in subset {
        positions.push(order.iter().position(|x| x == i).unwrap() as i32);
    }
    return positions.is_sorted();
}
fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut sort_rules: Vec<(i32, i32)> = Vec::new();
    let mut set2: bool = false;
    let mut requests: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        if line.is_empty() {
            set2 = true;
            continue;
        }
        if !set2 {
            let mut split = line.split('|');
            let l = split.next().unwrap().parse().unwrap();
            let r = split.next().unwrap().parse().unwrap();
            sort_rules.push((l, r));
        } else {
            let request: Vec<i32> = line.split(',').map(|x| x.parse().unwrap()).collect();
            requests.push(request);
        }
    }
    let mut sortorder: Vec<i32> = Vec::new();
    sortorder.push(sort_rules.first().unwrap().0);
    sortorder.push(sort_rules.first().unwrap().1);
    sort_rules.remove(0);
    for rule in sort_rules.clone() {
        insert(&mut sortorder, rule.0, rule.1, true);
    }
    let mut inordercount = 0;
    for request in requests {
        if is_in_relative_order(&request, &sortorder) {
            inordercount += 1;
        }
    }
    println!("{}", inordercount);
}
