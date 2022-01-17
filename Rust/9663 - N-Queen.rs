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

fn promising(queen_idx: i32, num: i32, n: i32, arr: &[i32; 16]) -> bool {
    for i in 1..=n {
        if arr[i as usize] == 0 {
            break;
        }

        if arr[i as usize] == num || (num - arr[i as usize]).abs() == (queen_idx - i).abs() {
            return false;
        }
    }

    true
}

fn check(num: i32, n: i32, arr: &mut [i32; 16], ans: &mut i32) {
    if num == n + 1 {
        *ans += 1;
    } else {
        for i in 1..=n {
            if promising(num, i, n, arr) {
                arr[num as usize] = i;
                check(num + 1, n, arr, ans);
                arr[num as usize] = 0;
            }
        }
    }
}

fn main() {
    let n = input_integers()[0];

    let mut arr = [0; 16];
    let mut ans = 0;

    check(1, n, &mut arr, &mut ans);

    println!("{}", ans);
}
