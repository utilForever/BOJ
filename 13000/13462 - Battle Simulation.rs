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

    let moves = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut idx = 0;
    let mut ret = String::new();

    while idx < moves.len() {
        if idx + 2 < moves.len()
            && moves[idx] != moves[idx + 1]
            && moves[idx] != moves[idx + 2]
            && moves[idx + 1] != moves[idx + 2]
        {
            ret.push('C');
            idx += 3;
        } else {
            if moves[idx] == 'R' {
                ret.push('S');
            } else if moves[idx] == 'B' {
                ret.push('K');
            } else {
                ret.push('H');
            }

            idx += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
