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
    let balls = scan.token::<String>();
    let mut balls = balls.chars().collect::<Vec<_>>();

    // Case 1: Red Left
    let pos = balls.iter().position(|&x| x == 'B');
    let ret1 = match pos {
        Some(pos) => balls[pos..].iter().filter(|&x| *x == 'R').count(),
        None => 0,
    };

    // Case 2: Blue Left
    let pos = balls.iter().position(|&x| x == 'R');
    let ret2 = match pos {
        Some(pos) => balls[pos..].iter().filter(|&x| *x == 'B').count(),
        None => 0,
    };

    balls.reverse();

    // Case 3: Red Right
    let pos = balls.iter().position(|&x| x == 'B');
    let ret3 = match pos {
        Some(pos) => balls[pos..].iter().filter(|&x| *x == 'R').count(),
        None => ret1,
    };

    // Case 4: Blue Right
    let pos = balls.iter().position(|&x| x == 'R');
    let ret4 = match pos {
        Some(pos) => balls[pos..].iter().filter(|&x| *x == 'B').count(),
        None => ret2,
    };

    writeln!(out, "{}", ret1.min(ret2).min(ret3).min(ret4)).unwrap();
}
