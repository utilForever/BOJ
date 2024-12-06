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

    let (rh, rv, sh, sv) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let n = scan.token::<i64>();
    let mut ret = i64::MAX;

    for _ in 0..n {
        let (rhi, rvi, shi, svi, price) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        let cnt_horizontal = ((rh + rhi - 1) / rhi).max((sh + shi - 1) / shi);
        let cnt_vertical = ((rv + rvi - 1) / rvi).max((sv + svi - 1) / svi);

        ret = ret.min(cnt_horizontal * cnt_vertical * price);

        let cnt_horizontal_rotated = ((rh + rvi - 1) / rvi).max((sh + svi - 1) / svi);
        let cnt_vertical_rotated = ((rv + rhi - 1) / rhi).max((sv + shi - 1) / shi);

        ret = ret.min(cnt_horizontal_rotated * cnt_vertical_rotated * price);
    }

    writeln!(out, "{ret}").unwrap();
}
