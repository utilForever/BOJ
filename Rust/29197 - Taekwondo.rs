use io::Write;
use std::{collections::BTreeSet, io, str};

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
    let mut points = BTreeSet::new();

    for _ in 0..n {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());

        if x == 0 && y > 0 {
            points.insert(String::from("y+"));
        } else if x == 0 && y < 0 {
            points.insert(String::from("y-"));
        } else if x > 0 && y == 0 {
            points.insert(String::from("x+"));
        } else if x < 0 && y == 0 {
            points.insert(String::from("x-"));
        } else {
            points.insert((x as f64).atan2(y as f64).to_string());
        }
    }

    writeln!(out, "{}", points.len()).unwrap();
}
