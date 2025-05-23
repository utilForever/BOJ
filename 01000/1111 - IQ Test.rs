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

    let n = scan.token::<usize>();
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    if n == 1 {
        writeln!(out, "A").unwrap();
        return;
    }

    if n == 2 {
        if nums[0] == nums[1] {
            writeln!(out, "{}", nums[0]).unwrap();
        } else {
            writeln!(out, "A").unwrap();
        }
        return;
    }

    if nums[0] == nums[1] {
        let all_same = nums.iter().all(|&x| x == nums[0]);

        if all_same {
            writeln!(out, "{}", nums[0]).unwrap();
        } else {
            writeln!(out, "B").unwrap();
        }
    } else {
        let diff1 = nums[1] - nums[0];
        let diff2 = nums[2] - nums[1];

        if diff2 % diff1 != 0 {
            writeln!(out, "B").unwrap();
        } else {
            let a = diff2 / diff1;
            let b = nums[1] - a * nums[0];
            let check = (1..n).all(|idx| {
                let expected = a * nums[idx - 1] + b;
                expected == nums[idx]
            });

            if check {
                writeln!(out, "{}", nums[n - 1] * a + b).unwrap();
            } else {
                writeln!(out, "B").unwrap();
            }
        }
    }
}
