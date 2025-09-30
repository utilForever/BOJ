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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();
    let decompose = |mut n: i64| -> Vec<i64> {
        let mut ret = Vec::new();

        if n % 3 == 1 {
            ret.push(2);
            ret.push(2);
            n -= 4;
        } else if n % 3 == 2 {
            ret.push(2);
            n -= 2;
        }

        while n > 0 {
            ret.push(3);
            n -= 3;
        }

        ret
    };

    for _ in 0..t {
        let (r, c) = (scan.token::<i64>(), scan.token::<i64>());

        if r == 1 || c == 1 {
            writeln!(out, "0").unwrap();
            continue;
        }

        let mut ret = Vec::new();

        if r % 2 == 0 && c % 2 == 0 {
            let mut idx_r = 1;

            while idx_r <= r {
                let mut idx_c = 1;

                while idx_c <= c {
                    ret.push((1, idx_r, idx_c));
                    idx_c += 2;
                }

                idx_r += 2;
            }
        } else if r % 3 == 0 && c % 3 == 0 {
            let mut idx_r = 1;

            while idx_r <= r {
                let mut idx_c = 1;

                while idx_c <= c {
                    ret.push((2, idx_r, idx_c));
                    idx_c += 3;
                }

                idx_r += 3;
            }
        } else if r % 6 == 0 {
            let parts = decompose(c);
            let mut idx_c = 1;

            for part in parts {
                if part == 2 {
                    let mut idx_r = 1;

                    while idx_r <= r {
                        ret.push((1, idx_r, idx_c));
                        idx_r += 2;
                    }
                } else {
                    let mut idx_r = 1;

                    while idx_r <= r {
                        ret.push((2, idx_r, idx_c));
                        idx_r += 3;
                    }
                }

                idx_c += part;
            }
        } else if c % 6 == 0 {
            let parts = decompose(r);
            let mut idx_r = 1;

            for part in parts {
                if part == 2 {
                    let mut idx_c = 1;

                    while idx_c <= c {
                        ret.push((1, idx_r, idx_c));
                        idx_c += 2;
                    }
                } else {
                    let mut idx_c = 1;

                    while idx_c <= c {
                        ret.push((2, idx_r, idx_c));
                        idx_c += 3;
                    }
                }

                idx_r += part;
            }
        } else {
            writeln!(out, "0").unwrap();
            continue;
        }

        writeln!(out, "{}", ret.len()).unwrap();

        for (ty, r, c) in ret {
            writeln!(out, "{ty} {r} {c}").unwrap();
        }
    }
}
