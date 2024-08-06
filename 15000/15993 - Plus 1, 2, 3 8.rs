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

    let t = scan.token::<usize>();

    let mut cnt = vec![vec![0_i64; 2]; 100_001];
    cnt[1][0] = 1;
    cnt[1][1] = 0;
    cnt[2][0] = 1;
    cnt[2][1] = 1;
    cnt[3][0] = 2;
    cnt[3][1] = 2;

    for i in 4..=100_000 {
        cnt[i][0] = (cnt[i - 1][1] + cnt[i - 2][1] + cnt[i - 3][1]) % 1_000_000_009;
        cnt[i][1] = (cnt[i - 1][0] + cnt[i - 2][0] + cnt[i - 3][0]) % 1_000_000_009;
    }

    for _ in 0..t {
        let n = scan.token::<usize>();

        writeln!(out, "{} {}", cnt[n][0], cnt[n][1]).unwrap();
    }
}
