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

    let n = scan.token::<usize>();
    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut ret = 0;

    for i in 0..=n - 3 {
        for j in i + 3..=n {
            let s = &s[i..j];
            let mut check = true;
            let mut cnt_n = 0;

            for (k, &c) in s.iter().enumerate() {
                if k == 0 && c != 'A' {
                    check = false;
                    break;
                }

                if k == s.len() - 1 && c != 'A' {
                    check = false;
                    break;
                }

                if k != 0 && k != s.len() - 1 && c == 'A' {
                    check = false;
                    break;
                }

                if c == 'N' {
                    cnt_n += 1;
                }
            }

            if check && cnt_n == 1 {
                ret += 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}