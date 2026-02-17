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

    loop {
        let line = scan.line().trim().to_string();

        if line.is_empty() {
            break;
        }

        let words = line.split_whitespace().collect::<Vec<_>>();
        let n = words[0].parse::<usize>().unwrap();
        let mut points = vec![(0.0, 0.0); n];

        for i in 0..n {
            points[i] = (
                words[2 * i + 1].parse::<f64>().unwrap(),
                words[2 * i + 2].parse::<f64>().unwrap(),
            );
        }

        write!(out, "{n}").unwrap();

        for i in 0..n {
            let mid_x = (points[i].0 + points[(i + 1) % n].0) / 2.0;
            let mid_y = (points[i].1 + points[(i + 1) % n].1) / 2.0;

            write!(out, " {:.6} {:.6}", mid_x, mid_y).unwrap();
        }

        writeln!(out).unwrap();
    }
}
