use std::{cmp, io};

fn input_integers() -> Vec<i32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn binary_search(lis: &Vec<i32>, value: i32) -> i32 {
    let mut left = 0;
    let mut right = lis.len() - 1;
    let mut mid;

    while left <= right {
        mid = (left + right) / 2;
        let lis_mid_left = if mid > 0 { lis[mid - 1] } else { 0 };

        if (lis_mid_left < value && value < lis[mid]) || lis[mid] == value {
            return mid as i32;
        } else if lis[mid] < value {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    -1
}

fn process_lis(
    sequence: &Vec<i32>,
    lis: &mut Vec<i32>,
    bound: i32,
) {
    lis.push(-1_000_000_007);

    for i in 1..=bound {
        let index = binary_search(&lis, sequence[i as usize]);

        if index == -1 {
            lis.push(sequence[i as usize]);
        } else if sequence[i as usize] < lis[index as usize] {
            lis[index as usize] = cmp::min(lis[index as usize], sequence[i as usize]);
        }
    }
}

fn main() {
    let n = input_integers()[0];

    let a = input_integers();
    let b = input_integers();

    let mut temp = vec![0; n as usize];
    let mut c = vec![0; n as usize];

    for i in 0..n {
        temp[b[i as usize] as usize - 1] = i;
    }

    for i in 0..n {
        c[i as usize] = temp[a[i as usize] as usize - 1];
    }

    let mut lis = Vec::new();
    
    c.insert(0, 0);

    process_lis(&c, &mut lis, n);
    println!("{}", lis.len() - 1);
}
