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
    let mut grades = vec![(' ', ' '); n];

    for i in 0..n {
        let subject = scan.token::<String>().chars().collect::<Vec<_>>();
        grades[i] = (subject[0], subject[1]);
    }

    let mut grade_sum = 0.0;

    for i in 0..n {
        grade_sum += match grades[i].0 {
            'A' => 4.0,
            'B' => 3.0,
            'C' => 2.0,
            'D' => 1.0,
            'E' => 0.0,
            _ => unreachable!(),
        };
    }

    let mut ret = grade_sum / n as f64;

    for i in 0..n {
        ret += match (grades[i].0, grades[i].1) {
            ('A', '1') | ('B', '1') | ('C', '1') => 0.05,
            ('A', '2') | ('B', '2') | ('C', '2') => 0.025,
            _ => 0.0,
        }
    }

    writeln!(out, "{:.9}", ret).unwrap();
}
