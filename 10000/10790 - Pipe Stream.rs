use io::Write;
use std::{cmp, io, str};

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

    let c = scan.token::<usize>();

    for _ in 0..c {
        let (l, v1, v2, t, s) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        let mut interval1 = (v2 - v1 + t - 1) / t;
        let v_min = v2 - interval1 * t;
        let mut intervals = 1;
        let mut knocks = 0;

        while intervals > 0 && interval1 > intervals {
            knocks += 1;

            let v_max = l / (s * knocks);
            let interval2 = cmp::min(interval1, (v_max - v_min + t) / t);

            intervals -= interval1 - interval2;
            intervals *= 2;
            interval1 = interval2;
        }

        writeln!(
            out,
            "{}",
            if intervals <= 0 {
                "impossible".to_string()
            } else {
                knocks.to_string()
            }
        )
        .unwrap();
    }
}
