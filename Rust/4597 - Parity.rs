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

    loop {
        let s = scan.token::<String>();

        if s == "#" {
            break;
        }

        let mut s = s.chars().collect::<Vec<_>>();
        let len = s.len();
        let mut cnt_one = 0;

        for i in 0..len - 1 {
            if s[i] == '1' {
                cnt_one += 1;
            }
        }

        s[len - 1] = if s[len - 1] == 'e' {
            if cnt_one % 2 == 0 {
                '0'
            } else {
                '1'
            }
        } else {
            if cnt_one % 2 == 0 {
                '1'
            } else {
                '0'
            }
        };

        writeln!(out, "{}", s.iter().collect::<String>()).unwrap();
    }
}
