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

    let (n, mut a, mut b) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut subjects = vec![(0, 0); 10];

    for i in 0..10 {
        subjects[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    for i in 0..8 - n {
        a += subjects[i].0 * 3;
        b += subjects[i].0 * 3;

        let remain = (6 - subjects[i].0).min(subjects[i].1);
        b += remain * 3;
    }

    writeln!(
        out,
        "{}",
        if a >= 66 && b >= 130 {
            "Nice"
        } else {
            "Nae ga wae"
        }
    )
    .unwrap();
}
