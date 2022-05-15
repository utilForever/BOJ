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

fn can_health(week: &Vec<usize>, idx: usize, sum: usize) -> bool {
    if sum == 4 {
        return true;
    }

    if idx >= 7 {
        return false;
    }

    let num = week[idx];

    for i in 0..=num {
        if can_health(&week, idx + 1, (sum + idx * i) % 7) {
            return true;
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut week = vec![0_usize; 7];

    for _ in 0..n {
        let work = scan.token::<usize>();
        week[work % 7] += 1;
    }

    writeln!(
        out,
        "{}",
        if can_health(&week, 1, 0) { "YES" } else { "NO" }
    )
    .unwrap();
}
