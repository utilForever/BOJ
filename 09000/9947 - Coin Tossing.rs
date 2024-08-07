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

    loop {
        let (name1, name2) = (scan.token::<String>(), scan.token::<String>());

        if name1 == "#" && name2 == "#" {
            break;
        }

        let n = scan.token::<i64>();
        let (mut score1, mut score2) = (0, 0);

        for _ in 0..n {
            let (coin1, coin2) = (scan.token::<char>(), scan.token::<char>());

            if coin1 == coin2 {
                score1 += 1;
            } else {
                score2 += 1;
            }
        }

        writeln!(out, "{name1} {score1} {name2} {score2}").unwrap();
    }
}
