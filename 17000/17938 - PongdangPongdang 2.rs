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
    let (mut p, t) = (scan.token::<usize>(), scan.token::<usize>());

    p -= 1;

    let left_pos = p * 2;
    let right_pos = p * 2 + 1;

    let mut hands = 1;
    let mut is_increase = true;
    let mut cur_idx = 0;

    for _ in 1..t {
        cur_idx = (cur_idx + hands) % (n * 2);

        if is_increase {
            if hands == n * 2 {
                hands -= 1;
                is_increase = false;
            } else {
                hands += 1;
            }
        } else {
            if hands == 1 {
                hands += 1;
                is_increase = true;
            } else {
                hands -= 1;
            }
        }
    }

    for i in 0..hands {
        let idx = (i + cur_idx) % (n * 2);

        if idx == left_pos || idx == right_pos {
            writeln!(out, "Dehet YeonJwaJe ^~^").unwrap();
            return;
        }
    }

    writeln!(out, "Hing...NoJam").unwrap();
}
