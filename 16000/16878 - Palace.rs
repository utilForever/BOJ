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

    let t = scan.token();

    let mut num_cases = vec![0; 10_000_001];

    num_cases[0] = 1;
    num_cases[1] = 1;
    num_cases[2] = 0;
    num_cases[3] = 0;
    num_cases[4] = 2;

    for i in 5..10_000_001 as i64 {
        num_cases[i as usize] = (1_000_000_007 * 10
            + ((i + 1) * num_cases[i as usize - 1]) % 1_000_000_007
            - ((i - 2) * num_cases[i as usize - 2]) % 1_000_000_007
            - ((i - 5) * num_cases[i as usize - 3]) % 1_000_000_007
            + ((i - 3) * num_cases[i as usize - 4]) % 1_000_000_007)
            % 1_000_000_007;
    }

    for _ in 0..t {
        let n: usize = scan.token();
        writeln!(out, "{}", num_cases[n]).unwrap();
    }
}
