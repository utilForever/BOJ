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

    let time_curr = scan.token::<String>();
    let time_lecture = scan.token::<String>();
    let (t, k) = (scan.token::<i64>(), scan.token::<i64>());

    let (h1, m1, s1) = {
        let v: Vec<i64> = time_curr
            .split(':')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        (v[0], v[1], v[2])
    };
    let (h2, m2, s2) = {
        let v: Vec<i64> = time_lecture
            .split(':')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        (v[0], v[1], v[2])
    };
    let time_curr = h1 * 3600 + m1 * 60 + s1;
    let time_lecture = h2 * 3600 + m2 * 60 + s2;

    writeln!(
        out,
        "{}",
        if 100 * time_curr + (100 - k) * t <= 100 * time_lecture {
            1
        } else {
            0
        }
    )
    .unwrap();
}
