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
        let (n, p) = (scan.token::<i64>(), scan.token::<String>());
        let p = p.chars().collect::<Vec<_>>();
        let p_sum = p.iter().map(|&c| c as i64 - '0' as i64).sum::<i64>();
        let mut cnt = 0;
        let mut is_satisfied = false;

        for i in 0..n {
            let num = p[i as usize] as i64 - '0' as i64;

            for j in (1..=9).rev() {
                if num == j || (p_sum - num + j) % 3 != 0 {
                    continue;
                }

                for k in 0..n {
                    if k == i {
                        write!(out, "{j}").unwrap();
                    } else {
                        write!(out, "{}", p[k as usize]).unwrap();
                    }
                }

                writeln!(out, " 3").unwrap();

                cnt += 1;

                if cnt == n {
                    is_satisfied = true;
                    break;
                }
            }

            if is_satisfied {
                break;
            }
        }
    }
}
