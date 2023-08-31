use io::Write;
use std::{collections::HashSet, io, str};

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

    let mut squares = [[0; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            squares[i][j] = scan.token::<i64>();
        }
    }

    let mut sums = HashSet::new();

    for i in 0..4 {
        sums.insert(squares[i].iter().sum::<i64>());
    }

    for i in 0..4 {
        sums.insert(squares.iter().map(|x| x[i]).sum::<i64>());
    }

    writeln!(
        out,
        "{}",
        if sums.len() == 1 {
            "magic"
        } else {
            "not magic"
        }
    )
    .unwrap();
}
