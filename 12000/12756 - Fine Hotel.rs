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

    let (atk_a, mut hp_a) = (scan.token::<i64>(), scan.token::<i64>());
    let (atk_b, mut hp_b) = (scan.token::<i64>(), scan.token::<i64>());

    loop {
        hp_a -= atk_b;
        hp_b -= atk_a;

        if hp_a <= 0 || hp_b <= 0 {
            break;
        }
    }

    writeln!(
        out,
        "{}",
        if hp_a <= 0 && hp_b <= 0 {
            "DRAW"
        } else if hp_b <= 0 {
            "PLAYER A"
        } else {
            "PLAYER B"
        }
    )
    .unwrap();
}
