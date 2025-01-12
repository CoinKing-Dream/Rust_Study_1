use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let s = input_line.trim_matches('\n').to_string();
    let val: Vec<&str> = s.split_whitespace().rev().collect();
    println!("{}", val.join(" "));
}
