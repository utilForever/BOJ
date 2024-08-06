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

    let (n, a, b) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut deadlines = vec![0; n];

    for i in 0..n {
        deadlines[i] = scan.token::<i64>();
    }

    deadlines.sort();

    // No sleep
    let time_task = a;
    let mut time_curr = 0;
    let mut cnt = 0;

    for &deadline in deadlines.iter() {
        if time_curr + time_task <= deadline {
            time_curr += time_task;
            cnt += 1;
        }
    }

    let mut ret = cnt;

    // Sleep
    for x in 1..a {
        let time_sleep = b * x;

        for pos in 0..n {
            let mut time_task = a;
            let mut time_curr = 0;
            let mut cnt = 0;

            for i in 0..n {
                if i == pos {
                    time_task = a - x;
                    time_curr += time_sleep;
                }

                if time_curr + time_task <= deadlines[i] {
                    time_curr += time_task;
                    cnt += 1;
                }
            }

            ret = ret.max(cnt);
        }
    }

    writeln!(out, "{ret}").unwrap();
}
