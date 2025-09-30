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
    let decompose = |n: i64| -> (i64, i64) {
        match n % 3 {
            0 => (n / 3, 0),
            1 => ((n - 4) / 3, 2),
            2 => ((n - 2) / 3, 1),
            _ => unreachable!(),
        }
    };

    for _ in 0..t {
        let (r, c) = (scan.token::<i64>(), scan.token::<i64>());

        if r == 1 || c == 1 {
            writeln!(out, "0").unwrap();
            continue;
        }

        if (r * c) % 6 != 0 {
            writeln!(out, "0").unwrap();
            continue;
        }

        let mut ret = Vec::with_capacity(r as usize * c as usize / 6);

        if r % 2 == 0 && c % 3 == 0 {
            let mut idx_r = 1;

            while idx_r + 1 <= r {
                let mut idx_c = 1;

                while idx_c + 2 <= c {
                    ret.push((1, idx_r, idx_c));
                    idx_c += 3;
                }

                idx_r += 2;
            }
        } else if r % 3 == 0 && c % 2 == 0 {
            let mut idx_r = 1;

            while idx_r + 2 <= r {
                let mut idx_c = 1;

                while idx_c + 1 <= c {
                    ret.push((2, idx_r, idx_c));
                    idx_c += 2;
                }

                idx_r += 3;
            }
        } else if r % 6 == 0 {
            let (a, b) = decompose(c);
            let mut base = 1;

            while base + 5 <= r {
                if a > 0 {
                    let mut offset = 0;

                    while offset <= 4 {
                        let idx_r = base + offset;
                        let mut idx_c = 1;

                        while idx_c + 2 <= 3 * a {
                            ret.push((1, idx_r, idx_c));
                            idx_c += 3;
                        }

                        offset += 2;
                    }
                }

                if b > 0 {
                    let mut offset = 0;

                    while offset <= 3 {
                        let idx_r = base + offset;
                        let mut idx_c = 3 * a + 1;

                        while idx_c + 1 <= c {
                            ret.push((2, idx_r, idx_c));
                            idx_c += 2;
                        }

                        offset += 3;
                    }
                }

                base += 6;
            }
        } else if c % 6 == 0 {
            let (a, b) = decompose(r);
            let mut base = 1;

            while base + 5 <= c {
                if a > 0 {
                    let mut idx_r = 1;

                    while idx_r + 2 <= 3 * a {
                        let mut idx_c = base;

                        while idx_c + 1 <= base + 5 {
                            ret.push((2, idx_r, idx_c));
                            idx_c += 2;
                        }

                        idx_r += 3;
                    }
                }

                if b > 0 {
                    let mut idx_r = 3 * a + 1;

                    while idx_r + 1 <= r {
                        let mut idx_c = base;

                        while idx_c + 2 <= base + 5 {
                            ret.push((1, idx_r, idx_c));
                            idx_c += 3;
                        }

                        idx_r += 2;
                    }
                }

                base += 6;
            }
        } else {
            unreachable!()
        }

        writeln!(out, "{}", ret.len()).unwrap();

        for (ty, r, c) in ret {
            writeln!(out, "{ty} {r} {c}").unwrap();
        }
    }
}
