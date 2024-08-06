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

    let mut gears = vec![vec![0; 8]; 4];

    for i in 0..4 {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            gears[i][j] = c.to_digit(10).unwrap();
        }
    }

    let k = scan.token::<i64>();
    let mut ret = 0;

    for _ in 0..k {
        let (idx_rotate, dir_rotate) = (scan.token::<usize>() - 1, scan.token::<i64>());
        let mut gears_rotate = vec![0; 4];

        gears_rotate[idx_rotate] = dir_rotate;

        // Check left
        for i in (0..idx_rotate).rev() {
            if gears[i][2] == gears[i + 1][6] {
                break;
            }

            gears_rotate[i] = -gears_rotate[i + 1];
        }

        // Check right
        for i in idx_rotate + 1..4 {
            if gears[i - 1][2] == gears[i][6] {
                break;
            }

            gears_rotate[i] = -gears_rotate[i - 1];
        }

        // Process rotate
        for i in 0..4 {
            if gears_rotate[i] == 1 {
                // Rotate clockwise
                let tmp = gears[i][7];
                gears[i].remove(7);
                gears[i].insert(0, tmp);
            } else if gears_rotate[i] == -1 {
                // Rotate counter-clockwise
                let tmp = gears[i][0];
                gears[i].remove(0);
                gears[i].push(tmp);
            }
        }
    }

    // Calculate score
    for i in 0..4 {
        if gears[i][0] == 1 {
            ret += 1 << i;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
