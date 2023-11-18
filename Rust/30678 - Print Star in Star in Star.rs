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

fn fill_stars(ret: &mut Vec<Vec<char>>, y: usize, x: usize, size_orig: usize, size: usize) {
    if y >= size_orig || x >= size_orig {
        return;
    }

    if size == 0 {
        ret[y][x] = '*';
        return;
    }

    fill_stars(ret, y, x + size * 2, size_orig, size / 5);
    fill_stars(ret, y + size, x + size * 2, size_orig, size / 5);
    fill_stars(ret, y + size * 2, x, size_orig, size / 5);
    fill_stars(ret, y + size * 2, x + size, size_orig, size / 5);
    fill_stars(ret, y + size * 2, x + size * 2, size_orig, size / 5);
    fill_stars(ret, y + size * 2, x + size * 3, size_orig, size / 5);
    fill_stars(ret, y + size * 2, x + size * 4, size_orig, size / 5);
    fill_stars(ret, y + size * 3, x + size, size_orig, size / 5);
    fill_stars(ret, y + size * 3, x + size * 2, size_orig, size / 5);
    fill_stars(ret, y + size * 3, x + size * 3, size_orig, size / 5);
    fill_stars(ret, y + size * 4, x + size, size_orig, size / 5);
    fill_stars(ret, y + size * 4, x + size * 3, size_orig, size / 5);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let size = 5_i64.pow(n as u32) as usize;
    let mut ret = vec![vec![' '; size]; size];

    fill_stars(&mut ret, 0, 0, size, size / 5);

    for i in 0..size {
        for j in 0..size {
            write!(out, "{}", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
