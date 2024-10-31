use io::Write;
use std::{collections::HashSet, io, str};

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

    loop {
        let k = scan.token::<i64>();

        if k == 0 {
            break;
        }

        let m = scan.token::<i64>();
        let mut courses = HashSet::new();

        for _ in 0..k {
            courses.insert(scan.token::<i64>());
        }

        let mut ret = true;

        for _ in 0..m {
            let (c, r) = (scan.token::<i64>(), scan.token::<i64>());
            let mut cnt = 0;

            for _ in 0..c {
                let course = scan.token::<i64>();

                if courses.contains(&course) {
                    cnt += 1;
                }
            }

            if cnt < r {
                ret = false;
            }
        }

        writeln!(out, "{}", if ret { "yes" } else { "no" }).unwrap();
    }
}
