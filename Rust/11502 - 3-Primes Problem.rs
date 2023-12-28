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

    let mut is_prime = vec![false; 1001];
    is_prime[1] = true;

    for i in 2..=1000 {
        if is_prime[i] {
            continue;
        }

        for j in (i * i..=1000).step_by(i) {
            is_prime[j] = true;
        }
    }

    is_prime.iter_mut().for_each(|x| *x = !*x);

    let t = scan.token::<i64>();

    for _ in 0..t {
        let k = scan.token::<usize>();
        let mut check = false;

        'outer: for a in 1..=998 {
            for b in 1..=998 {
                if a + b >= k {
                    break;
                }

                if is_prime[a] && is_prime[b] && is_prime[k - (a + b)] {
                    check = true;

                    writeln!(out, "{a} {b} {}", k - (a + b)).unwrap();

                    break 'outer;
                }
            }
        }

        if !check {
            writeln!(out, "0").unwrap();
        }
    }
}
