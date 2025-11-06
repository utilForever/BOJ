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

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (r, c, x) = (scan.token::<i64>(), scan.token::<usize>(), scan.token::<u128>());

        if r != 2 || c % 3 != 0 {
            writeln!(out, "-1").unwrap();
            continue;
        }

        if c / 3 <= 127 && x > 2u128.pow(c as u32 / 3) {
            writeln!(out, "-1").unwrap();
            continue;
        }
        
        let mut ret_top = String::with_capacity(c);
        let mut ret_bottom = String::with_capacity(c);
        let mut idx = x - 1;

        for i in 0..c / 3 {
            let remain = c / 3 - i - 1;
            let pow = if remain >= 128 { u128::MAX } else { 2u128.pow(remain as u32) };

            if idx < pow {
                ret_top.push_str("114");
                ret_bottom.push_str("144");
            } else {
                idx -= pow;
                ret_top.push_str("322");
                ret_bottom.push_str("332");
            }
        }

        writeln!(out, "{ret_top}").unwrap();
        writeln!(out, "{ret_bottom}").unwrap();
    }
}
