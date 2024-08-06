use io::Write;
use std::{io, str};

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; 1001];

    for _ in 0..n {
        let num = scan.token::<usize>();
        nums[num] += 1;
    }

    for i in 0..=1000 {
        if nums[i] == 0 {
            continue;
        }

        // If nums[i] and nums[i + 1] are both positive,
        // find the next smallest number that is not 0 from i + 2
        if i + 1 <= 1000 && nums[i + 1] > 0 {
            let mut found = false;

            for j in i + 2..=1000 {
                if nums[j] == 0 {
                    continue;
                }

                for _ in 0..nums[i] {
                    write!(out, "{i} ").unwrap();
                }

                // Only use one number of nums[j]
                // because we should print the earliest in lexigraphical order
                write!(out, "{j} ").unwrap();

                nums[j] -= 1;

                found = true;
                break;
            }

            // If next smallest number is found, continue to the next iteration
            if found {
                continue;
            }

            // If not found, swap all numbers of nums[i + 1] with nums[i]
            for _ in 0..nums[i + 1] {
                write!(out, "{} ", i + 1).unwrap();
            }

            for _ in 0..nums[i] {
                write!(out, "{i} ").unwrap();
            }

            nums[i + 1] = 0;
        } else {
            // If nums[i] is positive and nums[i + 1] is 0,
            // print all numbers of nums[i]
            for _ in 0..nums[i] {
                write!(out, "{i} ").unwrap();
            }
        }
    }

    writeln!(out).unwrap();
}
