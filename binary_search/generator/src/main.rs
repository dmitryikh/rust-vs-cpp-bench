extern crate rand;
use std::io;
use rand::Rng;


fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let line = read_line();
    let mut iter = line.split_whitespace();
    // A size
    let n: usize = iter.next().unwrap().parse().unwrap();
    // B size
    let k: usize = iter.next().unwrap().parse().unwrap();
    // Uniform probabily that B[i] is in A
    let pr: f64 = iter.next().unwrap().parse().unwrap();

    let mut a_vec: Vec<u32> = vec![0; n];
    let mut b_vec: Vec<u32> = vec![0; k];
    for a in &mut a_vec {
        *a = rand::thread_rng().gen_range(1, 1_000_000_000);
    }

    a_vec.sort();

    for b in &mut b_vec {
        let chance: f64 = rand::thread_rng().gen_range(0.0, 1.0);
        *b = if chance <= pr {
                a_vec[rand::thread_rng().gen_range(0, n)]
            } else {
                rand::thread_rng().gen_range(1, 1_000_000_000)
            }
    }
    print!("{}", n);
    for a in a_vec {
        print!(" {}", a);
    }
    println!("");

    print!("{}", k);
    for b in b_vec {
        print!(" {}", b);
    }
    println!("");
}
