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

    let mut waters = [(0, 0); 3];
    for i in 0..3 {
        waters[i] = (scan.token::<i64>(), i + 1);
    }

    let mut ret = Vec::new();

    loop {
        waters.sort();

        if waters[0].0 == 0 {
            break;
        }

        let mut q = waters[1].0 / waters[0].0;
        let r = waters[1].0 % waters[0].0;

        if r <= waters[0].0 / 2 {
            while q > 0 {
                if q % 2 == 1 {
                    ret.push((waters[1].1, waters[0].1));
                    waters[1].0 -= waters[0].0;
                    waters[0].0 *= 2;
                } else {
                    ret.push((waters[2].1, waters[0].1));
                    waters[2].0 -= waters[0].0;
                    waters[0].0 *= 2;
                }

                q >>= 1;
            }
        } else {
            q += 1;

            while q > 1 {
                if q % 2 == 1 {
                    ret.push((waters[1].1, waters[0].1));
                    waters[1].0 -= waters[0].0;
                    waters[0].0 *= 2;
                } else {
                    ret.push((waters[2].1, waters[0].1));
                    waters[2].0 -= waters[0].0;
                    waters[0].0 *= 2;
                }

                q >>= 1;
            }

            ret.push((waters[0].1, waters[1].1));
            waters[0].0 -= waters[1].0;
            waters[1].0 *= 2;
        }
    }

    writeln!(out, "{}", ret.len()).unwrap();
    for (a, b) in ret {
        writeln!(out, "{} {}", a, b).unwrap();
    }
}
