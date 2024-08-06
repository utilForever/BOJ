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
    let mut filenames = vec![Vec::new(); n];

    for i in 0..n {
        let filename = scan.token::<String>();
        let filename = filename.chars().collect::<Vec<_>>();
        filenames[i] = filename;
    }

    let mut ret = String::new();

    for i in 0..filenames[0].len() {
        let char = filenames[0][i];
        let mut is_same = true;

        for j in 1..n {
            if char != filenames[j][i] {
                is_same = false;
                break;
            }
        }

        ret.push(if is_same { char } else { '?' });
    }

    writeln!(out, "{ret}").unwrap();
}
