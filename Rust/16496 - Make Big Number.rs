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

fn input_strings() -> Vec<String> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<String> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let n = input_integers()[0] as usize;
    let mut nums = input_strings();

    for i in 0..(n - 1) {
        let mut idx = i;

        for j in (i + 1)..n {
            let mut str1 = nums[j].clone();
            str1.push_str(&nums[idx]);
            let mut str2 = nums[idx].clone();
            str2.push_str(&nums[j]);

            if str1.parse::<i64>().unwrap() > str2.parse::<i64>().unwrap() {
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

    if nums.iter().filter(|s| s == &"0").count() == n {
        println!("0");
    } else {
        println!("{}", ret);
    }
}
