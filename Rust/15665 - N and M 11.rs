use io::Write;
use std::io::{self, BufWriter, StdoutLock};

fn input_integers() -> Vec<usize> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<usize> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn check(
    out: &mut BufWriter<StdoutLock>,
    nums: &mut Vec<usize>,
    num_count: &[usize; 10000],
    arr: &mut [i32; 9],
    num: usize,
    n: usize,
    m: usize,
) {
    if num == m + 1 {
        for i in 1..=m {
            write!(out, "{} ", arr[i]).unwrap();
        }

        writeln!(out).unwrap();
    } else {
        for i in 1..=nums.len() {
            arr[num] = nums[i - 1] as i32;
            check(out, nums, num_count, arr, num + 1, n, m);
            arr[num] = 0;
        }
    }
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let nums = input_integers();
    let (n, m) = (nums[0], nums[1]);

    let mut nums = input_integers();
    let mut num_count = [0_usize; 10000];
    let mut arr = [0; 9];

    nums.sort();

    for num in nums.iter() {
        num_count[*num] += 1;
    }

    nums.dedup();

    check(&mut out, &mut nums, &mut num_count, &mut arr,  1, n, m);
}
