use std::io;
use rand::Rng;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}


fn calculate_area_and_perimeter(dimensions: (u32, u32)) -> (u32, u32) {
    let min_len = {
        if (dimensions.0 > dimensions.1) {dimensions.1} 
        else {dimensions.0}
    }
    let r = (min_len as f32 / 2.0) as f32;
    {3.14*r*r, 2*3.14*r}
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut n = parse_input!(input_line, i32);
    
    while {
        if n == 0 {break;}
        n -= 1;
        
        let mut rng = rand::thread_rng();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();
        
        let val: Vec<&str> = line.split_whitespace().collect();
        let a = val[0].parse::<u32>().unwrap();
        let b = val[1].parse::<u32>().unwrap();

        let res: (u32, u32) = calculate_area_and_perimeter((a,b));
        println!("{} {}", res.0, res.1);
    }
}
