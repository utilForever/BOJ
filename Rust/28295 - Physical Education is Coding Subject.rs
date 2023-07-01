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

    // 0: N, 1: E, 2: S, 3: W
    let mut ret = 0;

    for _ in 0..10 {
        let t = scan.token::<i64>();

        match t {
            1 => match ret {
                0 => {
                    ret = 1;
                }
                1 => {
                    ret = 2;
                }
                2 => {
                    ret = 3;
                }
                3 => {
                    ret = 0;
                }
                _ => unreachable!(),
            },
            2 => match ret {
                0 => {
                    ret = 2;
                }
                1 => {
                    ret = 3;
                }
                2 => {
                    ret = 0;
                }
                3 => {
                    ret = 1;
                }
                _ => unreachable!(),
            },
            3 => match ret {
                0 => {
                    ret = 3;
                }
                1 => {
                    ret = 0;
                }
                2 => {
                    ret = 1;
                }
                3 => {
                    ret = 2;
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    writeln!(
        out,
        "{}",
        match ret {
            0 => "N",
            1 => "E",
            2 => "S",
            3 => "W",
            _ => unreachable!(),
        }
    )
    .unwrap();
}
