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

    let n = scan.token::<i64>();
    let before = scan.token::<String>();
    let after = scan.token::<String>();
    let before = before.chars().collect::<Vec<_>>();
    let after = after.chars().collect::<Vec<_>>();

    if n % 2 == 0 {
        writeln!(
            out,
            "{}",
            if before == after {
                "Deletion succeeded"
            } else {
                "Deletion failed"
            }
        )
        .unwrap();
    } else {
        let mut ret = true;

        for (bit_before, bit_after) in before.iter().zip(after.iter()) {
            if bit_before == bit_after {
                ret = false;
                break;
            }
        }

        writeln!(
            out,
            "{}",
            if ret {
                "Deletion succeeded"
            } else {
                "Deletion failed"
            }
        )
        .unwrap();
    }
}
