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

    let (min, max) = (scan.token::<u64>(), scan.token::<u64>());
    let mut checked = vec![false; 1_000_001];
    let mut is_square = vec![false; 1_000_001];
    let mut num_not_square = max - min + 1;
    let mut i = 2;

    loop {
        if i * i > max {
            break;
        }

        if checked[i as usize] {
            i += 1;
            continue;
        }

        let mut j = i;

        loop {
            if j * j > max {
                break;
            }

            checked[j as usize] = true;

            j += i;
        }

        let square = i * i;
        let mut j = ((min - 1) / square + 1) * square;

        loop {
            if j > max {
                break;
            }

            if !is_square[(j - min) as usize] {
                is_square[(j - min) as usize] = true;
                num_not_square -= 1;
            }

            j += square;
        }
    }

    writeln!(out, "{}", num_not_square).unwrap();
}
