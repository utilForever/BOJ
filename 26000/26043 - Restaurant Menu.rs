use io::Write;
use std::{collections::VecDeque, io, str};

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

    let n = scan.token::<i64>();
    let mut students = VecDeque::new();
    let mut students_match = Vec::new();
    let mut students_not_match = Vec::new();

    for _ in 0..n {
        let a = scan.token::<i64>();

        if a == 1 {
            let (b, c) = (scan.token::<i64>(), scan.token::<i64>());
            students.push_back((b, c));
        } else if a == 2 {
            let b = scan.token::<i64>();

            if students.front().unwrap().1 == b {
                students_match.push(students.pop_front().unwrap().0);
            } else {
                students_not_match.push(students.pop_front().unwrap().0);
            }
        }
    }

    let mut students = students.iter().collect::<Vec<_>>();
    students.sort();
    students_match.sort();
    students_not_match.sort();

    if students_match.is_empty() {
        writeln!(out, "None").unwrap();
    } else {
        for student in students_match.iter() {
            write!(out, "{student} ").unwrap();
        }

        writeln!(out).unwrap();
    }

    if students_not_match.is_empty() {
        writeln!(out, "None").unwrap();
    } else {
        for student in students_not_match.iter() {
            write!(out, "{student} ").unwrap();
        }

        writeln!(out).unwrap();
    }

    if students.is_empty() {
        writeln!(out, "None").unwrap();
    } else {
        for student in students.iter() {
            write!(out, "{} ", student.0).unwrap();
        }

        writeln!(out).unwrap();
    }
}
