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

    let (n, m, a, b) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut time_table = vec![0; n];
    let mut subjects = vec![0; m + 1];

    for i in 0..n {
        time_table[i] = scan.token::<usize>();
    }

    for _ in 0..a {
        let idx = scan.token::<usize>();
        subjects[idx] = 1;
    }

    for _ in 0..b {
        let idx = scan.token::<usize>();
        subjects[idx] = -1;
    }

    let mut cnt = 0i64;
    let mut ret = 0;

    for i in 0..n {
        if subjects[time_table[i]] == 0 {
            if cnt.abs() >= 3 {
                ret += cnt;
            }

            cnt = 0;
        } else if subjects[time_table[i]] == 1 {
            if cnt <= 0 {
                if cnt <= -3 {
                    ret += cnt;
                }

                cnt = 1;
            } else {
                cnt += 1;
            }
        } else {
            if cnt >= 0 {
                if cnt >= 3 {
                    ret += cnt;
                }

                cnt = -1;
            } else {
                cnt -= 1;
            }
        }
    }

    if cnt.abs() >= 3 {
        ret += cnt;
    }

    writeln!(out, "{ret}").unwrap();
}
