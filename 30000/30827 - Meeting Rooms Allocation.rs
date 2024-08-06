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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut meetings = vec![(0, 0); n];
    let mut rooms = vec![0; k];

    for i in 0..n {
        meetings[i] = (scan.token::<usize>(), scan.token::<usize>());
    }

    meetings.sort_by(|a, b| a.1.cmp(&b.1));

    let mut ret = 0;

    for (start, end) in meetings {
        rooms.sort();

        for room in rooms.iter_mut().rev() {
            if *room < start {
                *room = end;
                ret += 1;
                break;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
