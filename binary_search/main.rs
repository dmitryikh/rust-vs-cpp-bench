use std::io;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

// read number of elements, read elements into vector
fn read_vec_w_num() -> Vec<u32> {
    let line = read_line();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut vec = vec![0; n];
    for i in 0..n {
        vec[i] = iter.next().unwrap().parse().unwrap();
    }
    vec
}

// return position of the element if found
fn binary_search(vec: &[u32], value: u32) -> Option<usize> {
    let mut l: i32 = 0;
    let mut r: i32 = vec.len() as i32 - 1;
    while  l <= r {
        let i = ((l + r) / 2) as usize;
        if vec[i] == value {
          return Some(i);
        } else if vec[i] > value {
          r = i as i32 - 1;
        } else if vec[i] < value {
          l = i as i32 + 1;
        } 
    }
    None
}

fn main() {

    // 1. Read the array, and values for search
    let mut a_vec = read_vec_w_num();
    let b_vec = read_vec_w_num();

    // 2. Sort the array
    a_vec.sort();

    // 3. Search values, write results
    for b in b_vec {
        print!("{} ", binary_search(&a_vec, b).map_or(-1, |i| i as i32 + 1));
    }
}
