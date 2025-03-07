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

    let n = scan.token::<i64>();
    let mut ret = 0;

    for _ in 0..n {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
        let dist = x * x + y * y;

        ret += match dist {
            ..=100 => 10,
            101..=900 => 9,
            901..=2500 => 8,
            2501..=4900 => 7,
            4901..=8100 => 6,
            8101..=12100 => 5,
            12101..=16900 => 4,
            16901..=22500 => 3,
            22501..=28900 => 2,
            28901..=36100 => 1,
            _ => 0,
        }
    }

    writeln!(out, "{ret}").unwrap();
}
