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

    let mut is_prime = vec![true; n + 1];
    is_prime[1] = false;

    let mut i = 2;

    while i * i <= n {
        if !is_prime[i] {
            i += 1;
            continue;
        }

        for j in (i * i..=n).step_by(i) {
            is_prime[j] = false;
        }

        i += 1;
    }

    let mut ret = 1;

    for i in 2..=n {
        if is_prime[i] {
            let mut pow = i;

            while pow * i <= n {
                pow *= i;
            }

            ret = ret * pow % (1 << 32);
        }
    }

    writeln!(out, "{}", ret).unwrap();
}
