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
    let mut arr = vec![0; n];

    for i in 0..n {
        arr[i] = scan.token::<i64>();
    }

    if n == 1 {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut is_ascending = true;

    for i in 0..n - 1 {
        if arr[i] > arr[i + 1] {
            is_ascending = false;
            break;
        }
    }

    if is_ascending {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut ret = 0;
    let mut idx = 0;
    let mut is_ascending = true;

    while idx < n - 1 {
        if is_ascending && arr[idx] > arr[idx + 1] {
            ret += 1;
            is_ascending = false;
        } else if !is_ascending && arr[idx] < arr[idx + 1] {
            ret += 1;
            is_ascending = true;
        }

        idx += 1;
    }

    writeln!(out, "{}", (ret as f64).log2() as i64 + 1).unwrap();
}
