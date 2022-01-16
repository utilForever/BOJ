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

fn promising(visit: &[i32; 9], num: usize, num_count: usize) -> bool {
    visit[num] < num_count as i32
}

fn check(
    out: &mut BufWriter<StdoutLock>,
    nums: &mut Vec<usize>,
    num_count: &[usize; 10000],
    arr: &mut [i32; 9],
    visit: &mut [i32; 9],
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
            if promising(&visit, i, num_count[nums[i - 1]]) {
                if arr.iter().max().unwrap() > &(nums[i - 1] as i32) {
                    continue;
                }

                visit[i] += 1;
                arr[num] = nums[i - 1] as i32;

                check(out, nums, num_count, arr, visit, num + 1, n, m);

                arr[num] = 0;
                visit[i] -= 1;
            }
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
    let mut visit = [0; 9];

    nums.sort();

    for num in nums.iter() {
        num_count[*num] += 1;
    }

    nums.dedup();

    check(&mut out, &mut nums, &mut num_count, &mut arr, &mut visit, 1, n, m);
}
