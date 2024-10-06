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

    let (n, _) = (scan.token::<i64>(), scan.token::<usize>());
    let mut waves = Vec::new();

    for _ in 0..n {
        let cmd = scan.token::<char>();

        if cmd == '!' {
            let (p, l, a) = (
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            );
            waves.push((p, l, a));
        } else {
            let pos = scan.token::<i64>();
            let mut ret = 0;

            for &(p, l, a) in waves.iter() {
                if pos >= p && pos <= p + l - 1 {
                    ret += if (pos - p) % 4 == 0 {
                        a
                    } else if (pos - p) % 4 == 2 {
                        -a
                    } else {
                        0
                    };
                }
            }

            writeln!(out, "{ret}").unwrap();
        }
    }
}
