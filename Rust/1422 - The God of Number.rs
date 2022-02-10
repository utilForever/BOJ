use std::io;

fn input_integers() -> Vec<u128> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<u128> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let nums = input_integers();
    let (k, n) = (nums[0] as usize, nums[1] as usize);

    let mut raw_nums = vec![0; k];
    let mut max = 0;
    for i in 0..k {
        let num = input_integers()[0];
        raw_nums[i] = num;

        if num > max {
            max = num;
        }
    }

    let mut nums = Vec::new();
    for i in 0..k {
        nums.push(raw_nums[i].to_string());
    }

    for _ in 0..(n - k) {
        nums.push(max.to_string());
    }

    for i in 0..(n - 1) {
        let mut idx = i;

        for j in (i + 1)..n {
            let mut str1 = nums[j].clone();
            str1.push_str(&nums[idx]);
            let mut str2 = nums[idx].clone();
            str2.push_str(&nums[j]);

            if str1.parse::<u128>().unwrap() > str2.parse::<u128>().unwrap() {
                idx = j;
            }
        }

        let temp = nums[idx].clone();
        nums[idx] = nums[i].clone();
        nums[i] = temp;
    }

    let mut ret = String::new();
    for num in nums.iter() {
        ret.push_str(&num);
    }

    println!("{}", ret);
}
