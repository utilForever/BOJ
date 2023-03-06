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

    let n = scan.token::<i64>();

    for _ in 0..n {
        let m = scan.token::<usize>();
        let mut grid = vec![vec![' '; m]; m];
        let mut ship = (0, 0);

        for i in 0..m {
            let line = scan.token::<String>();

            for (j, c) in line.chars().enumerate() {
                grid[i][j] = c;

                if c == 's' {
                    ship = (i, j);
                }
            }
        }

        let mut ret_location = (0, 0);
        let mut ret_dist = f64::MAX;

        for i in 0..m {
            for j in 0..m {
                if grid[i][j] == 'p' {
                    let dist = (((i as i64 - ship.0 as i64).pow(2)
                        + (j as i64 - ship.1 as i64).pow(2))
                        as f64)
                        .sqrt();

                    if dist < ret_dist {
                        ret_dist = dist;
                        ret_location = (i, j);
                    }
                }
            }
        }

        writeln!(
            out,
            "({},{}):({},{}):{:.2}",
            ship.0, ship.1, ret_location.0, ret_location.1, ret_dist
        )
        .unwrap();
    }
}
