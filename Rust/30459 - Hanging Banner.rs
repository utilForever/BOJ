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

    let (n, m, r) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut positions = vec![0; n];
    let mut lengths = vec![0; m];

    for i in 0..n {
        positions[i] = scan.token::<i64>();
    }

    for i in 0..m {
        lengths[i] = scan.token::<i64>();
    }

    positions.sort();
    lengths.sort();

    let mut ret = None;

    for i in 0..n - 1 {
        for j in i + 1..n {
            let target = 2 * r / (positions[j] - positions[i]);
            let mut index = lengths.partition_point(|&x| x <= target);

            if index == 0 {
                continue;
            }

            index -= 1;

            ret = match ret {
                None => Some(lengths[index] * (positions[j] - positions[i])),
                Some(val) => Some(val.max(lengths[index] * (positions[j] - positions[i]))),
            };
        }
    }

    writeln!(
        out,
        "{}",
        match ret {
            None => "-1".to_string(),
            Some(val) =>
                if val % 2 == 0 {
                    (val / 2).to_string() + ".0"
                } else {
                    (val / 2).to_string() + ".5"
                },
        }
    )
    .unwrap();
}
