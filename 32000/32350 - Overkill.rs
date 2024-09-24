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

    let (n, d, p) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut is_overkill = false;
    let mut damage_extra = 0;
    let mut ret = 0;

    for _ in 0..n {
        let mut health = scan.token::<i64>();

        if is_overkill {
            health -= damage_extra;
        }

        if health < 0 {
            if !is_overkill {
                is_overkill = true;
                damage_extra = health.abs() * p / 100;
            } else {
                is_overkill = false;
            }

            continue;
        } else if health > 0 {
            let q = health / d;
            ret += q;
            health -= q * d;

            if health > 0 {
                ret += 1;
                health -= d;
            }

            if health < 0 {
                is_overkill = true;
                damage_extra = health.abs() * p / 100;
            } else {
                is_overkill = false;
                damage_extra = 0;
            }
        } else {
            is_overkill = false;
            damage_extra = 0;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
