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

    let (n, m) = (scan.token::<i64>(), scan.token::<i64>());

    writeln!(
        out,
        "{}",
        if n % 2 == 1 && m % 2 == 1 {
            n * m - 1
        } else {
            n * m
        }
    )
    .unwrap();

    let mut idx_x = 1;
    let mut idx_y = 1;

    if n % 2 == 0 {
        while idx_y < m {
            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_y += 1;
        }

        while idx_x < n - 1 {
            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_x += 1;

            while idx_y > 2 {
                writeln!(out, "{idx_x} {idx_y}").unwrap();
                idx_y -= 1;
            }

            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_x += 1;

            while idx_y < m {
                writeln!(out, "{idx_x} {idx_y}").unwrap();
                idx_y += 1;
            }
        }

        writeln!(out, "{idx_x} {idx_y}").unwrap();
        idx_x += 1;

        while idx_y > 1 {
            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_y -= 1;
        }

        while idx_x > 1 {
            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_x -= 1;
        }
    } else if m % 2 == 0 {
        while idx_x < n {
            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_x += 1;
        }

        while idx_y < m - 1 {
            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_y += 1;

            while idx_x > 2 {
                writeln!(out, "{idx_x} {idx_y}").unwrap();
                idx_x -= 1;
            }

            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_y += 1;

            while idx_x < n {
                writeln!(out, "{idx_x} {idx_y}").unwrap();
                idx_x += 1;
            }
        }

        writeln!(out, "{idx_x} {idx_y}").unwrap();
        idx_y += 1;

        while idx_x > 1 {
            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_x -= 1;
        }

        while idx_y > 1 {
            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_y -= 1;
        }
    } else {
        while idx_y < m {
            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_y += 1;
        }

        while idx_x < n - 2 {
            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_x += 1;

            while idx_y > 2 {
                writeln!(out, "{idx_x} {idx_y}").unwrap();
                idx_y -= 1;
            }

            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_x += 1;

            while idx_y < m {
                writeln!(out, "{idx_x} {idx_y}").unwrap();
                idx_y += 1;
            }
        }

        writeln!(out, "{idx_x} {idx_y}").unwrap();
        idx_x += 1;

        while idx_y > 1 {
            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_y -= 1;

            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_x += 1;

            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_y -= 1;

            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_x -= 1;
        }

        while idx_x > 1 {
            writeln!(out, "{idx_x} {idx_y}").unwrap();
            idx_x -= 1;
        }
    }
}
