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

    loop {
        let (nx, ny, w) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<f64>(),
        );

        if nx == 0 && ny == 0 && w == 0.0 {
            break;
        }

        let mut horizontals = vec![0.0; nx];
        let mut verticals = vec![0.0; ny];

        for i in 0..nx {
            horizontals[i] = scan.token::<f64>();
        }

        for i in 0..ny {
            verticals[i] = scan.token::<f64>();
        }

        horizontals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        verticals.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut ret = true;

        for i in 0..horizontals.len() {
            if i == 0 {
                if horizontals[i] - w / 2.0 > 0.0 {
                    ret = false;
                    break;
                }
            }

            if i == horizontals.len() - 1 {
                if horizontals[i] + w / 2.0 < 75.0 {
                    ret = false;
                    break;
                }
            }

            if i > 0 {
                if horizontals[i] - w / 2.0 > horizontals[i - 1] + w / 2.0 {
                    ret = false;
                    break;
                }
            }
        }

        for i in 0..verticals.len() {
            if i == 0 {
                if verticals[i] - w / 2.0 > 0.0 {
                    ret = false;
                    break;
                }
            }

            if i == verticals.len() - 1 {
                if verticals[i] + w / 2.0 < 100.0 {
                    ret = false;
                    break;
                }
            }

            if i > 0 {
                if verticals[i] - w / 2.0 > verticals[i - 1] + w / 2.0 {
                    ret = false;
                    break;
                }
            }
        }

        writeln!(out, "{}", if ret { "YES" } else { "NO" }).unwrap();
    }
}
