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

    let mut n = scan.token::<i64>();

    if n == 0 {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut is_negative = false;
    let mut ret = String::new();

    if n < 0 {
        is_negative = true;
        n = -n;
    }

    while n > 0 {
        let val = n % 3;

        ret.push(match val {
            0 => '0',
            1 => {
                if is_negative {
                    'T'
                } else {
                    '1'
                }
            }
            2 => {
                if is_negative {
                    '1'
                } else {
                    'T'
                }
            }
            _ => unreachable!(),
        });

        n /= 3;

        if val == 2 {
            n += 1;
        }
    }

    ret = ret.chars().rev().collect::<String>();

    writeln!(out, "{ret}").unwrap();
}
