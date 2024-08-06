use io::Write;
use std::{collections::HashSet, io, str};

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

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut cards = vec![String::new(); n];

    for i in 0..n {
        cards[i] = scan.token::<String>();
    }

    let mut set = HashSet::new();

    if k == 2 {
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                let val = cards[i].clone() + &cards[j];
                set.insert(val);
            }
        }
    } else if k == 3 {
        for i in 0..n {
            for j in 0..n {
                for l in 0..n {
                    if i == j || i == l || j == l {
                        continue;
                    }

                    let val = cards[i].clone() + &cards[j] + &cards[l];
                    set.insert(val);
                }
            }
        }
    } else if k == 4 {
        for i in 0..n {
            for j in 0..n {
                for l in 0..n {
                    for m in 0..n {
                        if i == j || i == l || i == m || j == l || j == m || l == m {
                            continue;
                        }

                        let val = cards[i].clone() + &cards[j] + &cards[l] + &cards[m];
                        set.insert(val);
                    }
                }
            }
        }
    }

    writeln!(out, "{}", set.len()).unwrap();
}
