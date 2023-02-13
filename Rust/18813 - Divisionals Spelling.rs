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

    let (n, m) = (scan.token::<i64>(), scan.token::<usize>());
    let mut questions = vec![false; 26];
    let mut ret = 0;

    for i in 0..m {
        questions[i] = true;
    }

    for _ in 0..n {
        let question = scan.token::<String>();
        let mut alphabets = vec![0; 26];
        let mut is_duplicate = false;

        for c in question.chars() {
            let index = c as usize - 'A' as usize;
            alphabets[index] += 1;

            if alphabets[index] > 1 {
                is_duplicate = true;
                break;
            }
        }

        if is_duplicate {
            continue;
        }

        let mut is_valid = true;

        for c in question.chars() {
            let index = c as usize - 'A' as usize;

            if questions[index] == false {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
