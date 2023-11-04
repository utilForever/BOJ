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
    let mut problems = vec![0; n];
    let mut has_streak_freeze = true;
    let mut date_use = 0;
    let mut cnt = 0;
    let mut ret = 0;

    for i in 0..n {
        problems[i] = scan.token::<i64>();
    }

    for i in 0..n {
        if i - date_use == 2 {
            has_streak_freeze = true;
        }

        if problems[i] > 0 {
            cnt += 1;
        } else {
            if has_streak_freeze {
                has_streak_freeze = false;
                date_use = i;
            } else {
                ret = ret.max(cnt);
                cnt = 0;
            }
        }
    }

    ret = ret.max(cnt);

    writeln!(out, "{ret}").unwrap();
}
