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
    lis_idx: &mut Vec<i32>,
    track: &mut Vec<i32>,
    bound: i32,
) {
    lis.push(-1_000_000_007);
    lis_idx.push(0);
    track.push(0);

    for i in 1..=bound {
        let index = binary_search(&lis, sequence[i as usize]);

        if index == -1 {
            lis.push(sequence[i as usize]);
            track[i as usize] = *lis_idx.last().unwrap();
            lis_idx.push(i);
        } else if sequence[i as usize] < lis[index as usize] {
            lis[index as usize] = cmp::min(lis[index as usize], sequence[i as usize]);
            lis_idx[index as usize] = i as i32;
            track[i as usize] = lis_idx[index as usize - 1];
        }
    }
}

fn main() {
    let n = input_integers()[0];

    let mut sequence = input_integers();
    let mut lis = Vec::new();
    let mut lis_idx = Vec::new();
    let mut track = vec![0; 1_000_000];

    sequence.insert(0, 0);

    process_lis(&sequence, &mut lis, &mut lis_idx, &mut track, n);
    println!("{}", lis.len() - 1);

    let mut cur = *lis_idx.last().unwrap();
    let mut answer = Vec::new();

    while cur != 0 {
        answer.push(sequence[cur as usize]);
        cur = track[cur as usize];
    }

    for ans in answer.iter().rev() {
        print!("{} ", ans);
    }

    println!();
}
