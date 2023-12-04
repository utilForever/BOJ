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

    let mut fibonacci = [0_i64; 80];
    fibonacci[1] = 1;
    fibonacci[2] = 1;

    for i in 3..80 {
        fibonacci[i] = fibonacci[i - 1] + fibonacci[i - 2];
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (k, x) = (scan.token::<i64>(), scan.token::<i64>());

        if k == 1 {
            writeln!(
                out,
                "{}",
                if fibonacci.iter().position(|&v| v == x).is_some() {
                    "YES"
                } else {
                    "NO"
                }
            )
            .unwrap();
        } else if k == 2 {
            let mut is_found = false;

            for i in 1..80 {
                for j in 1..80 {
                    if fibonacci[i] + fibonacci[j] == x {
                        is_found = true;
                        break;
                    }
                }
            }

            writeln!(out, "{}", if is_found { "YES" } else { "NO" }).unwrap();
        } else {
            let mut is_found = false;

            for i in 1..80 {
                for j in 1..80 {
                    for k in 1..80 {
                        if fibonacci[i] + fibonacci[j] + fibonacci[k] == x {
                            is_found = true;
                            break;
                        }
                    }
                }
            }

            writeln!(out, "{}", if is_found { "YES" } else { "NO" }).unwrap();
        }
    }
}
