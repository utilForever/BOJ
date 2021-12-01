use io::Write;
use std::cmp::Ordering;
use std::io;

pub fn next_permutation(nums: &mut Vec<i32>) -> bool {
    let last_ascending = match nums.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => {
            nums.reverse();
            return false;
        }
    };

    let swap_with = nums[last_ascending + 1..]
        .binary_search_by(|n| i32::cmp(&nums[last_ascending], n).then(Ordering::Less))
        .unwrap_err();
    nums.swap(last_ascending, last_ascending + swap_with);
    nums[last_ascending + 1..].reverse();

    true
}

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

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let n = input_integers()[0];

    let mut nums = vec![n; 0];

    for i in 1..=n {
        nums.push(i);
        write!(out, "{} ", i).unwrap();
    }

    writeln!(out, "").unwrap();

    loop {
        if !next_permutation(&mut nums) {
            break;
        }

        for num in nums.iter() {
            write!(out, "{} ", num).unwrap();
        }

        writeln!(out, "").unwrap();
    }
}
