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

    let (a, b) = (scan.token::<i64>(), scan.token::<i64>());
    let add = a + b;
    let sub = a - b;
    let mul = a * b;
    let max = add.max(sub).max(mul);

    if (max == add && max == sub) || (max == add && max == mul) || (max == sub && max == mul) {
        writeln!(out, "NIE").unwrap();
        return;
    }

    let ret_a = if a >= 0 {
        format!("{a}")
    } else {
        format!("({a})")
    };
    let ret_b = if b >= 0 {
        format!("{b}")
    } else {
        format!("({b})")
    };
    let ret_max = if max >= 0 {
        format!("{max}")
    } else {
        format!("({max})")
    };

    writeln!(
        out,
        "{ret_a}{}{ret_b}={ret_max}",
        if max == add {
            "+"
        } else if max == sub {
            "-"
        } else {
            "*"
        }
    )
    .unwrap();
}
