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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let s = scan.token::<String>();
        let s = s.chars().collect::<Vec<_>>();

        let mut idx = 0;
        let mut cnt_not = 0;
        let mut cnt_factorial = 0;

        while idx < s.len() && s[idx] == '!' {
            cnt_not += 1;
            idx += 1;
        }

        let mut ret = s[idx].to_digit(10).unwrap();
        idx += 1;

        while idx < s.len() {
            cnt_factorial += 1;
            idx += 1;
        }

        if cnt_factorial > 0 {
            ret = 1;
        }

        cnt_not = cnt_not % 2;
        ret ^= cnt_not;

        writeln!(out, "{ret}").unwrap();
    }
}
