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

fn process_backtrack(answers: &[i64; 10], submits: &mut [i64; 10], ret: &mut i64, idx: usize) {
    if idx == 10 {
        let mut cnt = 0;

        for i in 0..10 {
            if submits[i] == answers[i] {
                cnt += 1;
            }
        }

        if cnt >= 5 {
            *ret += 1;
        }

        return;
    }

    for i in 1..=5 {
        if idx >= 2 && submits[idx - 1] == i as i64 && submits[idx - 2] == i as i64 {
            continue;
        }

        submits[idx] = i as i64;
        process_backtrack(answers, submits, ret, idx + 1);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut answers = [0; 10];

    for i in 0..10 {
        answers[i] = scan.token::<i64>();
    }

    let mut submits = [0; 10];
    let mut ret = 0;

    process_backtrack(&answers, &mut submits, &mut ret, 0);

    writeln!(out, "{ret}").unwrap();
}
