extern crate rand;
use std::io;
use rand::Rng;


fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    // read numbers `n1` and `n2` in the first line. Generate two string with lengths `n1` and `n2`
    let line = read_line();
    let mut iter = line.split_whitespace();
    let n1: usize = iter.next().unwrap().parse().unwrap();
    let n2: usize = iter.next().unwrap().parse().unwrap();
    let mut sentence1 = String::new();
    let mut sentence2 = String::new();
    sentence1.reserve(n1 + 1);
    sentence2.reserve(n1 + 1);
    for _ in 0..n1 {
        sentence1.push(rand::thread_rng().gen_range('a' as u8, 'z' as u8) as char);
    }
    for _ in 0..n2 {
        sentence2.push(rand::thread_rng().gen_range('a' as u8, 'z' as u8) as char);
    }
    println!("{}", sentence1);
    println!("{}", sentence2);
}
