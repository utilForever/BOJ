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

    let mut nums = vec![0; 105];
    let mut pos = 0;
    let mut val = 0;

    for i in 1..=104 {
        if i % 2 == 1 {
            pos += 1;
            val += pos;
        } else {
            val = 2 * (pos + val);
            pos = 2 * pos + 1;
        }

        nums[i] = val;
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<i64>();
        let mut idx = 0;

        loop {
            if nums[idx] > n {
                writeln!(
                    out,
                    "{}",
                    if idx % 2 == 1 {
                        "eoaud0108"
                    } else {
                        "kidw0124"
                    }
                )
                .unwrap();
                break;
            }

            idx += 1;
        }
    }
}
