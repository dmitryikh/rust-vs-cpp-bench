extern crate rand;
use std::io;
use rand::Rng;


fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    // n - number of elements to generate
    let n: usize = read_line().parse().unwrap();

    let mut a_vec: Vec<u32> = vec![0; n];
    for a in &mut a_vec {
        *a = rand::thread_rng().gen_range(1, 1_000_000_000);
    }
    println!("{}", n);
    for a in a_vec {
        print!("{} ", a);
    }
    println!("");
}
