use std::io;
use std::cmp::min;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn merge_sort(vec: &mut [u32]) -> u64 {
    let mut swap_count: u64 = 0;
    let mut curr_size: usize = 1;
    let mut left_start: usize;
    let n = vec.len();
    if n < 2 {
        return 0;
    }
    // Merge subarrays in bottom up manner.  First merge subarrays of
    // size 1 to create sorted subarrays of size 2, then merge subarrays
    // of size 2 to create sorted subarrays of size 4, and so on.
    while curr_size <= n - 1 {
        left_start = 0;
        while left_start < n - 1 {
            let mid = left_start + curr_size - 1;
            let right_end = min(left_start + 2 * curr_size - 1, n - 1);
            swap_count += _merge(vec, left_start, mid, right_end);
            left_start += 2 * curr_size;
        }
        curr_size *= 2; 
    }
    swap_count
}

fn _merge(vec: &mut [u32], left: usize, mid: usize, right: usize) -> u64 {
    let mut swap_count: u64 = 0;
    let size: usize = right - left + 1;
    let mut curl = left;
    let mut curr = mid + 1;
    let mut tmp: Vec<u32> = vec![0; size];
    for i in 0..size {
        if (curl <= mid) && (curr <= right) {
            if vec[curl] <= vec[curr] {
                tmp[i] = vec[curl];
                curl += 1;
            } else {
                tmp[i] = vec[curr];
                curr += 1;
                swap_count += (mid - curl) as u64 + 1;
            }
        } else if curl <= mid {
            tmp[i] = vec[curl];
            curl += 1;
        } else if curr <= right  {
            tmp[i] = vec[curr];
            curr += 1;
        } else {
            panic!("Why i'm here!");
        }
    }
    // This line is not supported by rust compiler on the stepik course site
    // vec[left..right + 1].copy_from_slice(&tmp);
    // Here is workaround..:
    for (i, j) in (left..right + 1).enumerate() {
        vec[j] = tmp[i];
    }
    swap_count
}

fn main() {

    // 1. Read the array
    let n: usize = read_line().parse().unwrap();
    let mut a_vec: Vec<u32> = vec![0; n as usize];
    for (i, token) in read_line().split_whitespace().enumerate() {
        a_vec[i] = token.parse().unwrap();    
    }

    // 2. Mergesort the array, calculate number of swaps
    let nswap = merge_sort(&mut a_vec);

    // 3. Write result
    println!("{}", nswap);
}
