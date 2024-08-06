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
    let problem = match n {
        1 => 12,
        2 | 3 => 11,
        4 => 10,
        5 | 6 | 7 => 9,
        8 => 8,
        9 => 7,
        10 | 11 => 6,
        _ => unreachable!(),
    };
    let penalty = match n {
        1 => 1600,
        2 => 894,
        3 => 1327,
        4 => 1311,
        5 => 1004,
        6 => 1178,
        7 => 1357,
        8 => 837,
        9 => 1055,
        10 => 556,
        11 => 773,
        _ => unreachable!(),
    };

    writeln!(out, "{problem} {penalty}").unwrap();
}
