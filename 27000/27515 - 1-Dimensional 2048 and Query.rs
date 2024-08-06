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

    let q = scan.token::<i64>();
    let mut ret = 0;

    for _ in 0..q {
        let query = scan.token::<String>();
        let query = query.chars().collect::<Vec<_>>();

        let num = query[1..]
            .iter()
            .collect::<String>()
            .parse::<i64>()
            .unwrap();

        if query[0] == '+' {
            ret += num;
        } else {
            ret -= num;
        }

        if ret == 0 {
            writeln!(out, "0").unwrap();
            continue;
        }

        let mut num = ret;
        let mut bit = 0;
        let mut bit_max = 0;

        while num > 1 {
            num >>= 1;
            bit += 1;

            if num & 1 == 1 {
                bit_max = bit;
            }
        }

        writeln!(out, "{}", 2_i64.pow(bit_max as u32)).unwrap();
    }
}
