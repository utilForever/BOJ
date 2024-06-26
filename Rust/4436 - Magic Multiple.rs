use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        let n = s.parse::<i64>().unwrap();
        let mut nums = [false; 10];
        let mut k = 0;

        loop {
            let mut val = n * (k + 1);
            k += 1;

            while val > 0 {
                let digit = val % 10;
                nums[digit as usize] = true;
                val /= 10;
            }

            if nums.iter().all(|&x| x) {
                writeln!(out, "{k}").unwrap();
                break;
            }
        }
    }
}
