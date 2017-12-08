use std::io;
use std::cmp::min;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn get_levenshtein_distance(str1: &str, str2: &str) -> u32 {
    let n = str1.len() + 1;
    let m = str2.len() + 1;
    // compute 2D indexes into flat 1D index
    let ind = |i, j| i * m + j;
    let mut vec_d: Vec<u32> = vec![0; n * m];
    for i in 0..n {
        vec_d[ind(i, 0)] = i as u32;
    }
    for j in 0..m {
        vec_d[ind(0, j)] = j as u32;
    }

    for (i, c1) in str1.chars().enumerate() {
        for (j, c2) in str2.chars().enumerate() {
            let c =  if c1 == c2 {0} else {1};
            vec_d[ind(i + 1, j + 1)] = min( min( vec_d[ind(i, j + 1)] + 1
                                               , vec_d[ind(i + 1, j)] + 1
                                               )
                                          , vec_d[ind(i, j)] + c
                                          );
        }
    }
  return vec_d[ind(n - 1, m - 1)];
}

fn main() {
    // 1. Read two strings
    let str1 = read_line();
    let str2 = read_line();

    // 2. Calculate Levenshtein distance between strings
    let dist = get_levenshtein_distance(&str1, &str2);

    // 3. Write result
    println!("{}", dist);
}

