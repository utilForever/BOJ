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
    let mut coins_max = vec![0; n];
    let mut coins_min = vec![0; n];

    for i in 0..n {
        coins_max[i] = scan.token::<i64>();

        if coins_max[i] < 0 {
            coins_max[i] *= -1;
        }
    }

    for i in 0..n {
        coins_min[i] = scan.token::<i64>();

        if coins_min[i] > 0 {
            coins_min[i] *= -1;
        }
    }

    writeln!(
        out,
        "{}",
        coins_max.iter().sum::<i64>() - coins_min.iter().sum::<i64>()
    )
    .unwrap();
}
