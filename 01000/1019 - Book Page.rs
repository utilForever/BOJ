use std::io;

fn input_integers() -> Vec<i64> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i64> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn calc_nums(nums: &mut Vec<i64>, mut n: i64, cnt: i64) {
    while n > 0 {
        nums[(n % 10) as usize] += cnt;
        n /= 10;
    }
}

fn main() {
    let mut end = input_integers()[0];

    let mut nums = vec![0; 10];
    let mut start = 1;
    let mut multiplier = 1;

    while start <= end {
        while start % 10 != 0 && start <= end {
            calc_nums(&mut nums, start, multiplier);
            start += 1;
        }

        if start > end {
            break;
        }

        while end % 10 != 9 && start <= end {
            calc_nums(&mut nums, end, multiplier);
            end -= 1;
        }

        let cnt = (end / 10) - (start / 10) + 1;
        for i in 0..10 {
            nums[i] += cnt * multiplier;
        }

        start /= 10;
        end /= 10;
        multiplier *= 10;
    }

    for i in 0..10 {
        print!("{} ", nums[i]);
    }

    println!();
}
