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

    for _ in 0..n {
        let mut shapes_a = [0; 4];
        let mut shapes_b = [0; 4];

        let a = scan.token::<i64>();

        for _ in 0..a {
            let shape = scan.token::<usize>() - 1;
            shapes_a[3 - shape] += 1;
        }

        let b = scan.token::<i64>();

        for _ in 0..b {
            let shape = scan.token::<usize>() - 1;
            shapes_b[3 - shape] += 1;
        }

        writeln!(
            out,
            "{}",
            match shapes_a.cmp(&shapes_b) {
                std::cmp::Ordering::Less => "B",
                std::cmp::Ordering::Equal => "D",
                std::cmp::Ordering::Greater => "A",
            }
        )
        .unwrap();
    }
}
