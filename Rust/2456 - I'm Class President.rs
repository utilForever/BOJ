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
    let mut votes = vec![vec![0; 3]; 3];

    for _ in 0..n {
        for j in 0..3 {
            let score = scan.token::<usize>();
            votes[j][score - 1] += 1;
        }
    }

    let sum_a = votes[0][0] + votes[0][1] * 2 + votes[0][2] * 3;
    let sum_b = votes[1][0] + votes[1][1] * 2 + votes[1][2] * 3;
    let sum_c = votes[2][0] + votes[2][1] * 2 + votes[2][2] * 3;

    if sum_a > sum_b && sum_a > sum_c {
        writeln!(out, "1 {sum_a}").unwrap();
    } else if sum_b > sum_a && sum_b > sum_c {
        writeln!(out, "2 {sum_b}").unwrap();
    } else if sum_c > sum_a && sum_c > sum_b {
        writeln!(out, "3 {sum_c}").unwrap();
    } else if sum_a == sum_b && sum_a > sum_c {
        if votes[0][2] > votes[1][2] {
            writeln!(out, "1 {sum_a}").unwrap();
        } else if votes[1][2] > votes[0][2] {
            writeln!(out, "2 {sum_b}").unwrap();
        } else {
            if votes[0][1] > votes[1][1] {
                writeln!(out, "1 {sum_a}").unwrap();
            } else if votes[1][1] > votes[0][1] {
                writeln!(out, "2 {sum_b}").unwrap();
            } else {
                writeln!(out, "0 {sum_a}").unwrap();
            }
        }
    } else if sum_a == sum_c && sum_a > sum_b {
        if votes[0][2] > votes[2][2] {
            writeln!(out, "1 {sum_a}").unwrap();
        } else if votes[2][2] > votes[0][2] {
            writeln!(out, "3 {sum_c}").unwrap();
        } else {
            if votes[0][1] > votes[2][1] {
                writeln!(out, "1 {sum_a}").unwrap();
            } else if votes[2][1] > votes[0][1] {
                writeln!(out, "3 {sum_c}").unwrap();
            } else {
                writeln!(out, "0 {sum_a}").unwrap();
            }
        }
    } else if sum_b == sum_c && sum_b > sum_a {
        if votes[1][2] > votes[2][2] {
            writeln!(out, "2 {sum_b}").unwrap();
        } else if votes[2][2] > votes[1][2] {
            writeln!(out, "3 {sum_c}").unwrap();
        } else {
            if votes[1][1] > votes[2][1] {
                writeln!(out, "2 {sum_b}").unwrap();
            } else if votes[2][1] > votes[1][1] {
                writeln!(out, "3 {sum_c}").unwrap();
            } else {
                writeln!(out, "0 {sum_b}").unwrap();
            }
        }
    } else {
        if votes[0][2] > votes[1][2] && votes[0][2] > votes[2][2] {
            writeln!(out, "1 {sum_a}").unwrap();
        } else if votes[1][2] > votes[0][2] && votes[1][2] > votes[2][2] {
            writeln!(out, "2 {sum_b}").unwrap();
        } else if votes[2][2] > votes[0][2] && votes[2][2] > votes[1][2] {
            writeln!(out, "3 {sum_c}").unwrap();
        } else if votes[0][2] == votes[1][2] && votes[0][2] > votes[2][2] {
            if votes[0][1] > votes[1][1] {
                writeln!(out, "1 {sum_a}").unwrap();
            } else if votes[1][1] > votes[0][1] {
                writeln!(out, "2 {sum_b}").unwrap();
            } else {
                writeln!(out, "0 {sum_a}").unwrap();
            }
        } else if votes[0][2] == votes[2][2] && votes[0][2] > votes[1][2] {
            if votes[0][1] > votes[2][1] {
                writeln!(out, "1 {sum_a}").unwrap();
            } else if votes[2][1] > votes[0][1] {
                writeln!(out, "3 {sum_c}").unwrap();
            } else {
                writeln!(out, "0 {sum_a}").unwrap();
            }
        } else if votes[1][2] == votes[2][2] && votes[1][2] > votes[0][2] {
            if votes[1][1] > votes[2][1] {
                writeln!(out, "2 {sum_b}").unwrap();
            } else if votes[2][1] > votes[1][1] {
                writeln!(out, "3 {sum_c}").unwrap();
            } else {
                writeln!(out, "0 {sum_b}").unwrap();
            }
        } else {
            if votes[0][1] > votes[1][1] && votes[0][1] > votes[2][1] {
                writeln!(out, "1 {sum_a}").unwrap();
            } else if votes[1][1] > votes[0][1] && votes[1][1] > votes[2][1] {
                writeln!(out, "2 {sum_b}").unwrap();
            } else if votes[2][1] > votes[0][1] && votes[2][1] > votes[1][1] {
                writeln!(out, "3 {sum_c}").unwrap();
            } else {
                writeln!(out, "0 {sum_a}").unwrap();
            }
        }
    }
}
