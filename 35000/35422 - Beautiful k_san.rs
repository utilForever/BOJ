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
    let mut heights = vec![0; n + 1];

    for i in 0..=n {
        heights[i] = scan.token::<i64>();
    }

    if n % 2 == 1 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut base = 0;
    let mut offset = 0;
    let mut height_curr = 0;

    let mut cnt_negative = 0;
    let mut cnt_zero = 0;
    let mut cnt_positive = 0;

    for i in 1..=n {
        let slope = heights[i] - heights[i - 1];

        base += if slope == 0 { 1 } else { slope.abs() - 1 };
        height_curr -= 1;

        if slope > 0 {
            offset += 1;
            cnt_negative += 1;
        } else if slope == 0 {
            cnt_zero += 1;
        } else {
            cnt_positive += 1;
        }

        if height_curr < 0 {
            if cnt_negative > 0 {
                offset -= 1;
                cnt_negative -= 1;
            } else if cnt_zero > 0 {
                cnt_zero -= 1;
            } else if cnt_positive > 0 {
                offset += 1;
                cnt_positive -= 1;
            }

            height_curr += 2;
        }
    }

    writeln!(out, "{}", base / 2 + offset).unwrap();
}
