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
    let mut alrams = vec![0; n];

    for i in 0..n {
        let time = scan.token::<String>();
        let (h, m) = (
            time[0..1].parse::<i64>().unwrap(),
            time[2..4].parse::<i64>().unwrap(),
        );

        alrams[i] = h * 60 + m;
    }

    if n == 1 {
        writeln!(out, "2").unwrap();
        return;
    } else if n == 2 {
        writeln!(out, "{}", if alrams[1] - alrams[0] <= 10 { 1 } else { 2 }).unwrap();
        return;
    }

    let mut ret = 2;

    alrams.windows(3).for_each(|w| {
        if w[2] - w[0] <= 10 {
            ret = ret.min(0);
        } else if w[1] - w[0] <= 10 || w[2] - w[1] <= 10 {
            ret = ret.min(1);
        }
    });

    writeln!(out, "{ret}").unwrap();
}
