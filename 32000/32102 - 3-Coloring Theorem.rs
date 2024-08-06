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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let (r, g, b) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );

    let total = (n * m) as i64;
    let max = r.max(g).max(b);

    if (total % 2 == 0 && max > total / 2) || (total % 2 == 1 && max > total / 2 + 1) {
        writeln!(out, "NO").unwrap();
        return;
    }

    let mut colors = vec![(r, 'R'), (g, 'G'), (b, 'B')];
    colors.sort_by(|a, b| b.0.cmp(&a.0));

    let mut grid = vec![vec![' '; m]; n];

    writeln!(out, "YES").unwrap();

    if n >= m {
        for i in 0..n {
            for j in 0..m {
                if (i + j) % 2 == 1 {
                    continue;
                }
    
                grid[i][j] = colors[0].1;
                colors[0].0 -= 1;
    
                if colors[0].0 == 0 {
                    colors.remove(0);

                    if colors.len() == 2 {
                        colors.swap(0, 1);
                    }
                }
            }
        }
    
        for i in 0..n {
            for j in 0..m {
                if (i + j) % 2 == 0 {
                    continue;
                }
    
                grid[i][j] = colors[0].1;
                colors[0].0 -= 1;
    
                if colors[0].0 == 0 {
                    colors.remove(0);

                    if colors.len() == 2 {
                        colors.swap(0, 1);
                    }
                }
            }
        }
    } else {
        for i in 0..m {
            for j in 0..n {
                if (i + j) % 2 == 1 {
                    continue;
                }
    
                grid[j][i] = colors[0].1;
                colors[0].0 -= 1;
    
                if colors[0].0 == 0 {
                    colors.remove(0);

                    if colors.len() == 2 {
                        colors.swap(0, 1);
                    }
                }
            }
        }
    
        for i in 0..m {
            for j in 0..n {
                if (i + j) % 2 == 0 {
                    continue;
                }
    
                grid[j][i] = colors[0].1;
                colors[0].0 -= 1;
    
                if colors[0].0 == 0 {
                    colors.remove(0);

                    if colors.len() == 2 {
                        colors.swap(0, 1);
                    }
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            write!(out, "{}", grid[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
