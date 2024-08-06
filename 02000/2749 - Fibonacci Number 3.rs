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

static MOD: i64 = 1_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    if n == 1 || n == 2 {
        writeln!(out, "1").unwrap();
        return;
    }

    let mut start = 1;
    let mut end = 1;
    let mut period = 0;

    loop {
        let temp = (start + end) % MOD;

        start = end;
        end = temp;
        period += 1;

        if start == 1 && end == 1 {
            break;
        }
    }

    let mut ret = vec![0_i64; period + 1];
    ret[1] = 1;
    ret[2] = 1;

    for i in 3..=period {
        ret[i] = (ret[i - 1] + ret[i - 2]) % MOD;
    }

    writeln!(out, "{}", ret[n % period]).unwrap();
}
