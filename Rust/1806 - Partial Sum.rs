use std::io;

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
    let nums = input_integers();
    let (n, s) = (nums[0], nums[1]);

    let arr = input_integers();

    let (mut left, mut right) = (0_i32, 0_i32);
    let (mut sum, mut len, mut ans) = (0, 0, 1_000_000_000);

    loop {
        if sum >= s {
            if len < ans {
                ans = len;
            }
            sum -= arr[left as usize];
            left += 1;
            len -= 1;
        } else {
            if right == n {
                break;
            }

            sum += arr[right as usize];
            right += 1;
            len += 1;
        }
    }

    println!("{}", if ans == 1_000_000_000 { 0 } else { ans });
}
