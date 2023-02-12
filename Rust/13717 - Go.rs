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
    let mut ret_evolve = 0;
    let mut ret_max = (String::new(), i64::MIN);

    for _ in 0..n {
        let p = scan.token::<String>();
        let (k, mut m) = (scan.token::<i64>(), scan.token::<i64>());
        let mut cnt_evolve = 0;
        
        while k <= m {
            cnt_evolve += m / k;
            m = m % k + (m / k) * 2;
        }

        if cnt_evolve > ret_max.1 {
            ret_max = (p, cnt_evolve);
        }

        ret_evolve += cnt_evolve;
    }

    writeln!(out, "{ret_evolve}").unwrap();
    writeln!(out, "{}", ret_max.0).unwrap();
}
