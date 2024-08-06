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
    let mut ret = 0;

    for _ in 0..n {
        let mut dices = [0; 4];

        for i in 0..4 {
            dices[i] = scan.token::<i64>();
        }

        dices.sort();

        let reward = if dices[0] == dices[1] && dices[1] == dices[2] && dices[2] == dices[3] {
            50000 + dices[0] * 5000
        } else if (dices[0] == dices[1] && dices[1] == dices[2])
            || (dices[1] == dices[2] && dices[2] == dices[3])
        {
            10000 + dices[1] * 1000
        } else if dices[0] == dices[1] && dices[2] == dices[3] {
            2000 + dices[0] * 500 + dices[2] * 500
        } else if dices[0] == dices[1] || dices[1] == dices[2] {
            1000 + dices[1] * 100
        } else if dices[2] == dices[3] {
            1000 + dices[2] * 100
        } else {
            dices[3] * 100
        };

        ret = ret.max(reward);
    }

    writeln!(out, "{ret}").unwrap();
}
