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

    let n = scan.token::<i64>();

    let mut ret_min = n;
    let mut min_diff = 0;

    while ret_min % 100 != 99 {
        if ret_min == 0 {
            min_diff = i64::MAX;
            break;
        }

        ret_min -= 1;
        min_diff += 1;
    }

    let mut ret_max = n;
    let mut max_diff = 0;

    while ret_max % 100 != 99 {
        ret_max += 1;
        max_diff += 1;
    }

    writeln!(
        out,
        "{}",
        if max_diff > min_diff {
            ret_min
        } else {
            ret_max
        }
    )
    .unwrap();
}
