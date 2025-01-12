use std::io::{self, BufRead};
use std::collections::HashMap;

fn get_smallest_substring(str1: &str, str2: &str) -> String {
    let mut letter_count = HashMap::new();
    for c in str2.chars() {
        *letter_count.entry(c).or_insert(0) += 1;
    }

    let mut left = 0;
    let mut window_country = HashMap::new();
    let mut right = 0;
    let mut required = letter_count.len();
    let mut min_start = 0;
    let mut min_len = usize::MAX;

    while right < str1.len() {
        let a = str1.chars().nth(right).unwrap();
        if  letter_count.contains_key(&a) {
            *window_country.entry(a).or_insert(0) += 1;
            if window_country[&a] == letter_count[&a] {
                required -= 1;
            }
        }
        while required == 0 {
            let temp = right - left + 1;
            let c = str1.chars().nth(left).unwrap();
            if letter_count.contains_key(&c) {
                *window_country.entry(c).or_insert(0) -= 1;
                if window_country[&c] < letter_count[&c] {
                    if min_len > temp {
                        min_start = left;
                        min_len = right - left + 1
                    }
                    required += 1;
                }
            }
            left += 1;
        }
        right += 1;
    }
    if min_len == usize::MAX {
        "".to_string()
    } else {
        str1[min_start..min_start+min_len].to_string()
    }

}

fn main() {
    let mut str1 = String::new();
    io::stdin().lock().read_line(&mut str1).expect("Failed to read line");

    let mut str2 = String::new();
    io::stdin().lock().read_line(&mut str2).expect("Failed to read line");

    // Call the function to get the smallest substring
    let result = get_smallest_substring(&str1.trim(), &str2.trim());

    // Print the result or a message if no valid substring is found
    if !result.is_empty() {
        println!("{}", result);
    } else {
        println!("No valid substring found.");
    }
}
