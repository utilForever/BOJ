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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut nums = vec![0; n];

        for i in 0..n {
            nums[i] = scan.token::<i64>();
        }

        let diff = nums[1] - nums[0];
        let mut is_same = true;

        for i in 2..n {
            if nums[i] - nums[i - 1] != diff {
                is_same = false;
                break;
            }
        }

        if is_same {
            write!(out, "The next 5 numbers after [").unwrap();

            for i in 0..n {
                write!(out, "{}", nums[i]).unwrap();

                if i != n - 1 {
                    write!(out, ", ").unwrap();
                }
            }

            write!(out, "] are: [").unwrap();

            for i in 1..=5 {
                write!(out, "{}", nums[n - 1] + i * diff).unwrap();

                if i != 5 {
                    write!(out, ", ").unwrap();
                }
            }

            writeln!(out, "]").unwrap();
        } else {
            write!(out, "The sequence [").unwrap();

            for i in 0..n {
                write!(out, "{}", nums[i]).unwrap();

                if i != n - 1 {
                    write!(out, ", ").unwrap();
                }
            }

            writeln!(out, "] is not an arithmetic sequence.").unwrap();
        }
    }
}
