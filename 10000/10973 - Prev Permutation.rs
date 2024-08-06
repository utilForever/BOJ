use std::cmp::Ordering;
use std::io;

pub fn prev_permutation(nums: &mut Vec<i32>) -> bool {
    let first_ascending = match nums.windows(2).rposition(|w| w[0] > w[1]) {
        Some(i) => i,
        None => {
            return false;
        }
    };

    let swap_with = nums[first_ascending + 1..]
        .binary_search_by(|n| i32::cmp(n, &nums[first_ascending]).then(Ordering::Greater))
        .unwrap_err();
    nums.swap(first_ascending, first_ascending + swap_with);
    nums[first_ascending + 1..].reverse();

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
    let _ = input_integers();
    let mut nums = input_integers();

    let result = prev_permutation(&mut nums);

    if result {
        for num in nums {
            print!("{} ", num);
        }
    } else {
        println!("-1");
    }
}
