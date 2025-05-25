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

    let weight_final = scan.token::<i64>();
    let n = scan.token::<usize>();
    let mut exams = vec![(0, 0); n];
    let mut flag = true;

    for i in 0..n {
        exams[i] = ((10.0 * scan.token::<f64>()) as i64, scan.token::<i64>());

        if exams[i].0 < 58 {
            flag = false;
        }
    }

    if !flag {
        writeln!(out, "IMPOSSIBLE").unwrap();
        return;
    }

    let sum_weight = weight_final + exams.iter().map(|&(_, w)| w).sum::<i64>();
    let sum_grade_weight = exams.iter().map(|&(g, w)| g * w).sum::<i64>();
    let ret = (80 * sum_weight - sum_grade_weight + weight_final - 1) / weight_final;

    if ret > 100 {
        writeln!(out, "IMPOSSIBLE").unwrap();
    } else {
        writeln!(out, "{:.1}", ret.max(58) as f64 / 10.0).unwrap();
    }
}
