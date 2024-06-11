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

    // Blue: 0, Red: 1, Green: 2
    let p = scan.token::<f64>();
    let mut stones = vec![0; 100];

    let n = scan.token::<i64>();

    for _ in 0..n {
        let (pos, dir) = (scan.token::<usize>() - 1, scan.token::<char>());

        match dir {
            'L' => {
                if pos == 0 {
                    continue;
                }

                stones[..pos].iter_mut().for_each(|x| {
                    *x = (*x + 1) % 3;
                });
            }
            'R' => {
                if pos == 99 {
                    continue;
                }

                stones[pos + 1..].iter_mut().for_each(|x| {
                    *x = (*x + 1) % 3;
                });
            }
            _ => unreachable!(),
        }
    }

    let cnt_blue = stones.iter().filter(|&&x| x == 0).count();
    let cnt_red = stones.iter().filter(|&&x| x == 1).count();
    let cnt_green = stones.iter().filter(|&&x| x == 2).count();

    writeln!(out, "{:.2}", p * (cnt_blue as f64) / 100.0).unwrap();
    writeln!(out, "{:.2}", p * (cnt_red as f64) / 100.0).unwrap();
    writeln!(out, "{:.2}", p * (cnt_green as f64) / 100.0).unwrap();
}
