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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let s = scan.token::<String>();
        let chars = s.chars().collect::<Vec<_>>();
        let idx = (chars.len() as f64 / 3.0).ceil() as usize;

        let s_prime = chars[0..idx].iter().collect::<String>();
        let rev_s_prime = s_prime.chars().rev().collect::<String>();

        let mut tail_s_prime = s_prime.clone();
        tail_s_prime.remove(0);
        let mut tail_rev_s_prime = rev_s_prime.clone();
        tail_rev_s_prime.remove(0);

        let cond1 = s == (s_prime.clone() + &rev_s_prime.clone() + &s_prime.clone());
        let cond2 = s == (s_prime.clone() + &tail_rev_s_prime.clone() + &s_prime.clone());
        let cond3 = s == (s_prime.clone() + &rev_s_prime.clone() + &tail_s_prime.clone());
        let cond4 = s == (s_prime.clone() + &tail_rev_s_prime.clone() + &tail_s_prime.clone());

        writeln!(
            out,
            "{}",
            if cond1 || cond2 || cond3 || cond4 {
                1
            } else {
                0
            }
        )
        .unwrap();
    }
}
