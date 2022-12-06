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
    let mut arr = vec![vec![0; 3]; n];

    for i in 0..n {
        arr[i][0] = scan.token::<i64>();
        arr[i][1] = scan.token::<i64>();
        arr[i][2] = scan.token::<i64>();
    }

    let mut ret = 0;

    for i in 1..=3 {
        let mut cur = i;
        let mut cnt = 0;

        for j in 0..n {
            if cur == arr[j][0] {
                cur = arr[j][1];
            } else if cur == arr[j][1] {
                cur = arr[j][0];
            }

            if cur == arr[j][2] {
                cnt += 1;
            }
        }

        ret = ret.max(cnt);
    }

    writeln!(out, "{ret}").unwrap();
}
