use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut nums = Vec::new();

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        nums.push(s);
    }

    for i in 0..nums.len() - 1 {
        let mut idx = i;

        for j in i + 1..nums.len() {
            let mut str1 = nums[j].clone();
            str1.push_str(&nums[idx]);

            let mut str2 = nums[idx].clone();
            str2.push_str(&nums[j]);

            if str1 > str2 {
                idx = j;
            }
        }

        nums.swap(i, idx);
    }

    let mut ret = String::new();

    for num in nums.iter() {
        ret.push_str(&num);
    }

    writeln!(
        out,
        "{}",
        if nums.iter().filter(|s| s == &"0").count() == nums.len() {
            "0".to_string()
        } else {
            ret
        }
    )
    .unwrap();
}
