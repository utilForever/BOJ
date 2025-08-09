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

    let n = scan.token::<usize>();
    let mut nums = vec![0; n + 1];

    for i in 1..=n {
        nums[i] = scan.token::<i64>();
    }

    // b[1] = a[1]
    // b[2] = a[1] + a[2]
    // b[3] = a[3]
    // b[4] = a[1] + a[2] + a[3] + a[4]
    // b[5] = a[5]
    // b[6] = a[5] + a[6]
    // b[7] = a[7]
    // b[8] = a[1] + a[2] + a[3] + a[4] + a[5] + a[6] + a[7] + a[8]
    // ...

    for i in 1..=n {
        if i % 2 == 1 {
            continue;
        }

        let mut pos = i;
        let mut offset = 1;
        let mut sum = 0;

        while pos % 2 == 0 {
            sum += nums[i - offset];
            offset *= 2;
            pos /= 2;
        }

        nums[i - 1] -= sum;
    }

    for i in 1..=n {
        write!(out, "{} ", nums[i]).unwrap();
    }

    writeln!(out).unwrap();
}
