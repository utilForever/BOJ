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

    let (b, c, d) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut burgers = vec![0; b];
    let mut sides = vec![0; c];
    let mut drinks = vec![0; d];

    for i in 0..b {
        burgers[i] = scan.token::<i64>();
    }

    for i in 0..c {
        sides[i] = scan.token::<i64>();
    }

    for i in 0..d {
        drinks[i] = scan.token::<i64>();
    }

    writeln!(
        out,
        "{}",
        burgers.iter().sum::<i64>() + sides.iter().sum::<i64>() + drinks.iter().sum::<i64>()
    )
    .unwrap();

    burgers.sort_by(|a, b| b.cmp(a));
    sides.sort_by(|a, b| b.cmp(a));
    drinks.sort_by(|a, b| b.cmp(a));

    let cnt = b.min(c).min(d);
    let mut ret = 0;

    for i in 0..b {
        ret += if i < cnt {
            burgers[i] * 9 / 10
        } else {
            burgers[i]
        };
    }

    for i in 0..c {
        ret += if i < cnt { sides[i] * 9 / 10 } else { sides[i] };
    }

    for i in 0..d {
        ret += if i < cnt {
            drinks[i] * 9 / 10
        } else {
            drinks[i]
        };
    }

    writeln!(out, "{ret}").unwrap();
}
