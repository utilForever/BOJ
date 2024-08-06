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
        let (b, n) = (scan.token::<i64>(), scan.token::<u32>());

        if b == 0 && n == 0 {
            break;
        }

        let mut val_old = 1;

        for i in 1_i64..=1000000_i64 {
            let val = i.pow(n);

            if val >= b {
                writeln!(
                    out,
                    "{}",
                    match (val - b).abs().cmp(&(b - val_old).abs()) {
                        std::cmp::Ordering::Less => i,
                        std::cmp::Ordering::Equal => (i - 1).max(1),
                        std::cmp::Ordering::Greater => (i - 1).max(1),
                    }
                )
                .unwrap();

                break;
            }

            val_old = val;
        }
    }
}
