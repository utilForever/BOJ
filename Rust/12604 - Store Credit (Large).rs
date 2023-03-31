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

    for idx in 1..=n {
        let c = scan.token::<i64>();
        let i = scan.token::<usize>();
        let mut items = vec![0; i];

        for j in 0..i {
            items[j] = scan.token::<i64>();
        }

        let mut found = false;
        let mut ret = (0, 0);

        for j in 0..i {
            for k in j + 1..i {
                if items[j] + items[k] == c {
                    ret = (j + 1, k + 1);
                    found = true;
                    break;
                }
            }

            if found {
                break;
            }
        }

        writeln!(out, "Case #{idx}: {} {}", ret.0, ret.1).unwrap();
    }
}
