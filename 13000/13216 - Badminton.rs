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

    let records = scan.token::<String>();
    let mut score_a = 0;
    let mut score_b = 0;
    let mut win_a = 0;
    let mut win_b = 0;

    for record in records.chars() {
        if record == 'A' {
            score_a += 1;
        } else if record == 'B' {
            score_b += 1;
        }

        if score_a == 21 || score_b == 21 {
            writeln!(out, "{score_a}-{score_b}").unwrap();

            if score_a == 21 {
                win_a += 1;
            } else {
                win_b += 1;
            }

            score_a = 0;
            score_b = 0;
        }
    }

    writeln!(out, "{}", if win_a > win_b { "A" } else { "B" }).unwrap();
}
