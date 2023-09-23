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

    let (s, n) = (scan.token::<i64>(), scan.token::<usize>());
    let mut positions = vec![0; n];

    for i in 0..n {
        positions[i] = scan.token::<i64>();
    }

    let (k, l) = (scan.token::<i64>(), scan.token::<i64>());
    let mut ret_energy = i64::MAX;
    let mut ret_index = 0;

    for (idx, &position) in positions.iter().enumerate() {
        let mut curr_pos = s;
        let mut curr_energy = 0;

        if curr_pos > position {
            curr_pos -= 2 * k;

            if curr_pos > position {
                curr_energy += (curr_pos - position) * l;
            } else if curr_pos < position {
                curr_energy += position - curr_pos;
            }

            if curr_energy < ret_energy {
                ret_energy = curr_energy;
                ret_index = idx + 1;
            }
        } else {
            curr_pos += 2 * k;

            if curr_pos < position {
                curr_energy += (position - curr_pos) * l;
            } else if curr_pos > position {
                curr_energy += curr_pos - position;
            }

            if curr_energy < ret_energy {
                ret_energy = curr_energy;
                ret_index = idx + 1;
            }
        }
    }

    writeln!(out, "{ret_energy} {ret_index}").unwrap();
}
