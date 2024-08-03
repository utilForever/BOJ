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

    let time = scan.token::<String>();
    let time = time
        .split(':')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let (h, m) = (time[0], time[1]);
    let mut dist_min = i64::MAX;
    let mut ret = (0, 0);

    let positions: [(i64, i64); 10] = [
        (3, 1),
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 1),
        (1, 2),
        (2, 0),
        (2, 1),
        (2, 2),
    ];

    for i in (h..100).step_by(24) {
        for j in (m..100).step_by(60) {
            let nums = [i / 10, i % 10, j / 10, j % 10];
            let mut dist = 0;

            nums.windows(2).for_each(|x| {
                dist += (positions[x[0] as usize].0 - positions[x[1] as usize].0).abs();
                dist += (positions[x[0] as usize].1 - positions[x[1] as usize].1).abs();
            });

            if dist_min > dist {
                dist_min = dist;
                ret = (i, j);
            }
        }
    }

    writeln!(out, "{:02}:{:02}", ret.0, ret.1).unwrap();
}
