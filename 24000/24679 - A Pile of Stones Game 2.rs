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

    let t = scan.token::<usize>();

    for _ in 0..t {
        let (x, y, z) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        let mut arr = vec![x, y, z];
        arr.sort();

        let w = cmp::min(arr[2], arr[0] + arr[1] - 2);

        if (arr[0] == 0 && arr[1] % 2 == 1)
            || (arr[0] == 1 && arr[1] == arr[2] && arr[1] % 2 == 1)
            || (arr[0] > 1 && arr[0] % 2 == 0 && arr[1] % 2 == 0 && w % 2 == 0)
            || (arr[0] > 1 && (arr[0] + arr[1] + w) % 4 == 3)
        {
            writeln!(out, "B").unwrap();
        } else {
            writeln!(out, "R").unwrap();
        }
    }
}
