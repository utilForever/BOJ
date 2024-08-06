use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut is_unique = vec![false; 5001];

    for i in 1..=9 {
        is_unique[i] = true;
    }

    for i in 10..=5000 {
        let mut n = i;
        let mut nums = vec![0; 10];

        while n > 0 {
            nums[n % 10] += 1;
            n /= 10;
        }

        if nums.iter().filter(|&x| *x > 1).count() == 0 {
            is_unique[i] = true;
        }
    }

    let mut prefix_sum = vec![0; 5001];

    for i in 1..=5000 {
        prefix_sum[i] = prefix_sum[i - 1] + if is_unique[i] { 1 } else { 0 };
    }

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        let (a, b) = s.split_at(s.find(' ').unwrap());
        let (a, b) = (
            a.parse::<usize>().unwrap(),
            b.trim().parse::<usize>().unwrap(),
        );

        writeln!(out, "{}", prefix_sum[b] - prefix_sum[a - 1]).unwrap();
    }
}
