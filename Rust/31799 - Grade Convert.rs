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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut ret = vec![' '; n];

    let s = scan.token::<String>();
    let mut grades = Vec::new();
    let mut prev = String::new();

    for c in s.chars() {
        if c == 'A' || c == 'B' || c == 'C' {
            if !prev.is_empty() {
                grades.push(prev.clone());
                prev.clear();
            }
        }

        prev.push(c);
    }

    grades.push(prev);

    let mut prev = String::new();
    let mut idx = 0;

    for grade in grades {
        if grade == "C+" || grade == "C0" || grade == "C" || grade == "C-" {
            ret[idx] = 'B';
        } else if grade == "B0" || grade == "B" || grade == "B-" {
            if prev.is_empty() || prev == "C+" || prev == "C0" || prev == "C" || prev == "C-" {
                ret[idx] = 'D';
            } else {
                ret[idx] = 'B';
            }
        } else if grade == "A-" || grade == "B+" {
            if prev.is_empty()
                || prev == "B0"
                || prev == "B"
                || prev == "B-"
                || prev == "C+"
                || prev == "C0"
                || prev == "C"
                || prev == "C-"
            {
                ret[idx] = 'P';
            } else {
                ret[idx] = 'D';
            }
        } else if grade == "A0" || grade == "A" {
            if !prev.is_empty() && (prev == "A+" || prev == "A0" || prev == "A") {
                ret[idx] = 'P';
            } else {
                ret[idx] = 'E';
            }
        } else {
            ret[idx] = 'E';
        }

        prev = grade;
        idx += 1;
    }

    for val in ret {
        write!(out, "{val}").unwrap();
    }

    writeln!(out).unwrap();
}
