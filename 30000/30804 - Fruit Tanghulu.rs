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
    let mut fruits = vec![0; n];
    let mut cnt = vec![0; 10];

    for i in 0..n {
        fruits[i] = scan.token::<usize>();
    }

    let mut cnt_type = 0;
    let mut left = 0;
    let mut right = 0;
    let mut ret = 0;

    while left < n && right < n {
        cnt[fruits[right]] += 1;

        if cnt[fruits[right]] == 1 {
            cnt_type += 1;
        }

        while cnt_type > 2 {
            cnt[fruits[left]] -= 1;

            if cnt[fruits[left]] == 0 {
                cnt_type -= 1;
            }

            left += 1;
        }

        ret = ret.max(right - left + 1);
        right += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
