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
    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();
    let mut ret = [false; 13];

    for i in 0..n {
        for j in i..n {
            let symbol = s[i..=j].iter().collect::<String>();

            match symbol.as_str() {
                "I" => ret[1] = true,
                "II" => ret[2] = true,
                "III" => ret[3] = true,
                "IV" => ret[4] = true,
                "V" => ret[5] = true,
                "VI" => ret[6] = true,
                "VII" => ret[7] = true,
                "VIII" => ret[8] = true,
                "IX" => ret[9] = true,
                "X" => ret[10] = true,
                "XI" => ret[11] = true,
                "XII" => ret[12] = true,
                _ => (),
            }
        }
    }

    for i in 1..=12 {
        if ret[i] {
            write!(out, "{i} ").unwrap();
        }
    }

    writeln!(out).unwrap();
}
