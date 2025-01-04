use std::fs::read_to_string;

fn check_safe(line_string: &str) -> i32 {
    let tmp_chars: Vec<&str> = line_string.split(" ").collect();
    let vec_ints: Vec<i16> = tmp_chars
        .iter()
        .map(|v| v.parse::<i16>().unwrap())
        .collect();

    let mut safe_check = is_safe(&vec_ints);
    let mut iter_count = 0;
    while (safe_check == 0) && (iter_count < vec_ints.len()) {
        let mut x: Vec<i16> = vec_ints.clone();
        x.remove(iter_count);
        safe_check = is_safe(&x);
        iter_count += 1;
    }
    return safe_check;
}

fn is_safe(vec_int: &Vec<i16>) -> i32 {
    let mut order = 0;
    let mut idx = 0;
    for idx in 0..vec_int.len() - 1 {
        let diff = vec_int[idx] - vec_int[idx + 1];
        if diff == 0 {
            return 0;
        }
        if (diff > 3) | (diff < -3) {
            return 0;
        }
        if order == 0 {
            order = diff.signum();
            continue;
        }
        if diff.signum() != order {
            return 0;
        }
    }
    return 1;
}
fn main() {
    let mut safe_counter: i32 = 0;

    for line in read_to_string("../data/input.txt").unwrap().lines() {
        safe_counter += check_safe(&line.to_string());
    }

    println!("Number of Safe reports: {}", safe_counter);
}
