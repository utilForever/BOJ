use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut nums = String::new();
    io::stdin().read_line(&mut nums).unwrap();
    let nums = nums
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let (k, _) = (nums[0] as i64, nums[1]);

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();
    let s = s.chars().collect::<Vec<_>>();

    for c in s {
        match c {
            'a'..='z' => {
                let c = (c as i64 - 'a' as i64 + k) % 26;
                write!(out, "{}", (c as u8 + 'a' as u8) as char).unwrap();
            }
            'A'..='Z' => {
                let c = (c as i64 - 'A' as i64 + k) % 26;
                write!(out, "{}", (c as u8 + 'A' as u8) as char).unwrap();
            }
            _ => write!(out, "{c}").unwrap(),
        }
    }

    writeln!(out).unwrap();
}
