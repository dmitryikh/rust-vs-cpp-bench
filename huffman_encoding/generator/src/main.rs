extern crate rand;
use std::io;
use rand::Rng;


fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let n: usize = read_line().parse().unwrap();    
    let mut sentence = String::new();
    sentence.reserve(n + 1);
    for _ in 0..n {
        sentence.push(rand::thread_rng().gen_range('a' as u8, 'z' as u8) as char);
    }
    println!("{}", sentence);
}
