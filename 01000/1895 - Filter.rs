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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (r, c) = (scan.token::<usize>(), scan.token::<usize>());
    let mut image = vec![vec![0; c]; r];

    for i in 0..r {
        for j in 0..c {
            image[i][j] = scan.token::<i64>();
        }
    }

    let t = scan.token::<i64>();
    let mut filtered = vec![vec![0; c - 2]; r - 2];

    for i in 0..r - 2 {
        for j in 0..c - 2 {
            let mut subimage = vec![0; 9];

            for k in 0..3 {
                for l in 0..3 {
                    subimage[k * 3 + l] = image[i + k][j + l];
                }
            }

            subimage.sort();

            filtered[i][j] = subimage[4];
        }
    }

    writeln!(
        out,
        "{}",
        filtered.iter().flatten().filter(|&&x| x >= t).count()
    )
    .unwrap();
}
