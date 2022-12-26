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

    let n = scan.token::<usize>();
    let mut babels = vec![0; n + 2];

    babels[0] = 10;
    babels[n + 1] = 10;

    for i in 1..=n {
        babels[i] = scan.token::<i64>();
        babels[i] /= 2;
    }

    for i in (1..=n + 1).rev() {
        babels[i] -= babels[i - 1];
    }

    let mut stack = Vec::new();
    let mut babel_sum = 0;
    let mut babel_cnt = 0;

    for i in 1..=n + 1 {
        if babels[i] > 0 {
            stack.push(babels[i]);
        } else {
            babels[i] *= -1;

            while babels[i] > 0 {
                let weight_top = *stack.last().unwrap();
                let weight_min = weight_top.min(babels[i]);

                stack.pop();

                babel_sum += 2 * weight_min;
                babel_cnt += weight_min / 20;
                
                if weight_min % 20 != 0 {
                    babel_cnt += 1;
                }
                
                if weight_top > weight_min {
                    stack.push(weight_top - weight_min);
                }

                babels[i] -= weight_min;
            }
        }
    }

    writeln!(out, "{} {}", babel_sum * 2, babel_cnt * 4).unwrap();
}
