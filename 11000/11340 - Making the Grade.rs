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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (project, term_paper, midterm_exam) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        // project * 0.15 + term_paper * 0.2 + midterm_exam * 0.25 + final_exam * 0.4 >= 90
        // final_exam * 0.4 >= 90 - (project * 0.15 + term_paper * 0.2 + midterm_exam * 0.25)
        // final_exam >= (90 - (project * 0.15 + term_paper * 0.2 + midterm_exam * 0.25)) / 0.4
        let val = 9000 - (project * 15 + term_paper * 20 + midterm_exam * 25);
        let final_exam = if val % 40 == 0 {
            (val / 40).max(0)
        } else {
            (val / 40 + 1).max(0)
        };

        if final_exam > 100 {
            writeln!(out, "impossible").unwrap();
            continue;
        }

        writeln!(out, "{final_exam}").unwrap();
    }
}
