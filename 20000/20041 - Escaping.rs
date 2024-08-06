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

    let n = scan.token::<usize>();
    let mut polices = vec![(0, 0); n];

    for i in 0..n {
        polices[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let thief = (scan.token::<i64>(), scan.token::<i64>());
    let mut can_catch = [false, false, false, false];

    for police in polices.iter() {
        let diff = ((thief.0 - police.0).abs(), (thief.1 - police.1).abs());

        if diff.0 <= diff.1 {
            if thief.1 < police.1 {
                can_catch[0] = true;
            } else {
                can_catch[1] = true;
            }
        }

        if diff.0 >= diff.1 {
            if thief.0 < police.0 {
                can_catch[2] = true;
            } else {
                can_catch[3] = true;
            }
        }
    }

    writeln!(
        out,
        "{}",
        if can_catch.iter().all(|&x| x) {
            "NO"
        } else {
            "YES"
        }
    )
    .unwrap();
}
