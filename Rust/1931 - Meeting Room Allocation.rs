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
    let mut meetings = vec![(0, 0); n];

    for i in 0..n {
        let (start, finish) = (scan.token::<i32>(), scan.token::<i32>());
        meetings[i] = (start, finish);
    }

    meetings.sort_by(|a, b| {
        if a.1 == b.1 {
            return a.0.cmp(&b.0);
        }
        a.1.cmp(&b.1)
    });

    let mut cnt = 0;
    let mut last = -1;

    for meeting in meetings.iter() {
        let (start, finish) = meeting;

        if *start >= last {
            cnt += 1;
            last = *finish;
        }
    }

    writeln!(out, "{}", cnt).unwrap();
}
