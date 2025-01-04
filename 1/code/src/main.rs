use std::fs::read_to_string;

fn main() {
    let mut list_1: Vec<i64> = Vec::new();
    let mut list_2: Vec<i64> = Vec::new();

    let mut hamming_distance: i64 = 0;
    let mut similarity_score: i64 = 0;

    for line in read_to_string("../data/id_comparisons.txt")
        .unwrap()
        .lines()
    {
        let temp_str: String = line.to_string();
        let temp_str2: Vec<&str> = temp_str.split("   ").collect();

        list_1.push(temp_str2[0].parse::<i64>().unwrap());
        list_2.push(temp_str2[1].parse::<i64>().unwrap());
    }

    list_1.sort();
    list_2.sort();

    for i in 0..list_1.len() {
        hamming_distance += (list_1[i] - list_2[i]).abs();
    }

    println!("");
    println!(
        "The hamming distance of the two sorted list are: {}",
        hamming_distance
    );

    // O(n**2) solution
    for v in &list_1 {
        similarity_score += v * list_2.iter().filter(|&&s| s == *v).count() as i64;
    }

    // O(n) solution
    let list_len = list_1.len();
    let mut p_left = 0;
    let mut p_right = 0;
    let mut similarity_score2: i64 = 0;
    while (p_left < list_len) & (p_right < list_len) {
        if list_1[p_left] == list_2[p_right] {
            similarity_score2 += list_1[p_left];
            p_right += 1;
        } else if list_1[p_left] < list_2[p_right] {
            p_left += 1;
        } else {
            p_right += 1;
        }
    }

    println!(
        "The 'similarity score' for the list is: {}",
        similarity_score
    );
}
