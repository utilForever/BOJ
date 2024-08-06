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

fn promising(visit: &[bool; 9], num: usize) -> bool {
    !visit[num]
}

fn check(
    out: &mut BufWriter<StdoutLock>,
    nums: &mut Vec<usize>,
    arr: &mut [i32; 9],
    visit: &mut [bool; 9],
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
        for i in 1..=n {
            if promising(&visit, i) {
                visit[i] = true;
                arr[num] = nums[i - 1] as i32;

                check(out, nums, arr, visit, num + 1, n, m);

                arr[num] = 0;
                visit[i] = false;
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
    let mut arr = [0; 9];
    let mut visit = [false; 9];

    nums.sort();

    check(&mut out, &mut nums, &mut arr, &mut visit, 1, n, m);
}
