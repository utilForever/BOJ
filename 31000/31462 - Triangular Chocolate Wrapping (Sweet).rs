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
    let mut triangle = vec![vec!['#'; n]; n];

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            triangle[i][j] = c;
        }
    }

    let mut ret = true;

    for row in 0..n {
        for col in 0..=row {
            if triangle[row][col] == '#' {
                continue;
            }

            if triangle[row][col] == 'R' {
                if row + 1 < n {
                    if triangle[row + 1][col] != 'R' || triangle[row + 1][col + 1] != 'R' {
                        ret = false;
                        break;
                    }

                    triangle[row + 1][col] = '#';
                    triangle[row + 1][col + 1] = '#';
                } else {
                    ret = false;
                    break;
                }
            } else {
                if col + 1 <= row && row + 1 < n {
                    if triangle[row][col + 1] != 'B' || triangle[row + 1][col + 1] != 'B' {
                        ret = false;
                        break;
                    }

                    triangle[row][col + 1] = '#';
                    triangle[row + 1][col + 1] = '#';
                } else {
                    ret = false;
                    break;
                }
            }

            triangle[row][col] = '#';
        }

        if !ret {
            break;
        }
    }

    writeln!(out, "{}", if ret { 1 } else { 0 }).unwrap();
}
