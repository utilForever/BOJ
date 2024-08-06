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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut commands = vec![0; n];
    let mut dices = vec![0; m];

    for i in 0..n {
        commands[i] = scan.token::<i64>();
    }

    for i in 0..m {
        dices[i] = scan.token::<i64>();
    }

    let mut pos = 1;
    let mut ret = 0;

    for dice in dices {
        pos += dice;
        ret += 1;

        if pos >= n as i64 {
            break;
        }

        pos += commands[pos as usize - 1];

        if pos >= n as i64 {
            break;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
