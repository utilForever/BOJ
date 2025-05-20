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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let s = scan.token::<String>();
        let mut prefix_sum = Vec::with_capacity(2 * n + 1);
        let mut prev = 0;

        prefix_sum.push(0);

        for c in s.chars() {
            if c == '(' {
                prefix_sum.push(prev + 1);
                prev += 1;
            } else {
                prefix_sum.push(prev - 1);
                prev -= 1;
            }
        }

        if *prefix_sum.iter().min().unwrap() >= 0 {
            writeln!(out, "0").unwrap();
            continue;
        }

        let mut left = 0;
        let mut right = 0;

        for i in 0..=2 * n {
            if prefix_sum[i] < 0 {
                left = i;
                break;
            }
        }

        for i in (0..=2 * n).rev() {
            if prefix_sum[i] < 0 {
                right = i;
                break;
            }
        }

        let (mut before_max, mut before_pos) = (prefix_sum[0], 0);

        for i in 0..left {
            if prefix_sum[i] > before_max {
                before_max = prefix_sum[i];
                before_pos = i;
            }
        }

        let (mut after_max, mut after_pos) = (prefix_sum[right], right);

        for i in right..=2 * n {
            if prefix_sum[i] > after_max {
                after_max = prefix_sum[i];
                after_pos = i;
            }
        }

        let (mut global_max, mut global_pos) = (prefix_sum[0], 0);

        for i in 0..=2 * n {
            if prefix_sum[i] > global_max {
                global_max = prefix_sum[i];
                global_pos = i;
            }
        }

        if before_max + after_max >= global_max {
            writeln!(out, "1").unwrap();
            writeln!(out, "{} {}", before_pos + 1, after_pos).unwrap();
        } else {
            writeln!(out, "2").unwrap();
            writeln!(out, "1 {}", global_pos).unwrap();
            writeln!(out, "{} {}", global_pos + 1, 2 * n).unwrap();
        }
    }
}
