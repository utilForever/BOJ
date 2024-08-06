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

    for i in 1..=t {
        let (mut a, mut b) = (scan.token::<i64>(), scan.token::<i64>());

        if b > a {
            std::mem::swap(&mut a, &mut b);
        }

        if a == 6 && b == 5 {
            writeln!(out, "Case {i}: Sheesh Beesh").unwrap();
        } else if a == b {
            writeln!(
                out,
                "Case {i}: {}",
                match a {
                    1 => "Habb Yakk",
                    2 => "Dobara",
                    3 => "Dousa",
                    4 => "Dorgy",
                    5 => "Dabash",
                    6 => "Dosh",
                    _ => unreachable!(),
                }
            )
            .unwrap();
        } else {
            writeln!(
                out,
                "Case {i}: {} {}",
                match a {
                    1 => "Yakk",
                    2 => "Doh",
                    3 => "Seh",
                    4 => "Ghar",
                    5 => "Bang",
                    6 => "Sheesh",
                    _ => unreachable!(),
                },
                match b {
                    1 => "Yakk",
                    2 => "Doh",
                    3 => "Seh",
                    4 => "Ghar",
                    5 => "Bang",
                    6 => "Sheesh",
                    _ => unreachable!(),
                }
            )
            .unwrap();
        }
    }
}
