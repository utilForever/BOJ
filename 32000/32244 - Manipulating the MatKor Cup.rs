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
    let mut source = vec![vec![0; n]; n];
    let mut target = vec![vec![0; n]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            source[i][j] = c.to_digit(10).unwrap() as i64;
        }
    }

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            target[i][j] = c.to_digit(10).unwrap() as i64;
        }
    }

    let mut twos = 0;

    for i in 0..n {
        for j in 0..n {
            twos += target[i][j] - source[i][j];
        }
    }

    twos = (twos * 2) % 4;
    twos = (twos + 4) % 4;

    let mut ret = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            ret[i][j] = twos;
        }
    }

    let mut vertical = vec![0; n];
    let mut horizontal = vec![0; n];

    for i in 0..n {
        for j in 0..n {
            let diff = target[i][j] - source[i][j];

            if n % 4 == 0 {
                vertical[j] -= diff;
                horizontal[i] -= diff;
            } else {
                vertical[j] += diff;
                horizontal[i] += diff;
            }

            ret[i][j] -= diff;
        }
    }

    for i in 0..n {
        for j in 0..n {
            ret[i][j] += vertical[j] + horizontal[i];
            ret[i][j] %= 4;
            ret[i][j] = (ret[i][j] + 4) % 4;
        }
    }

    writeln!(out, "{}", ret.iter().flatten().sum::<i64>()).unwrap();
}
