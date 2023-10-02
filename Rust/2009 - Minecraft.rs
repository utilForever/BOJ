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
    let mut h = vec![vec!['0'; n]; n];
    let mut r = vec![vec!['0'; n]; n];
    let mut c = vec![vec!['0'; n]; n];

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, ch) in s.chars().enumerate() {
            h[i][j] = ch;
        }
    }

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, ch) in s.chars().enumerate() {
            r[i][j] = ch;
        }
    }

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, ch) in s.chars().enumerate() {
            c[i][j] = ch;
        }
    }

    let mut ret = vec![vec![vec!['0'; n]; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                ret[i][j][k] = h[j][k].min(r[i][k]).min(c[i][j]);
            }
        }
    }

    let mut h_restore = vec![vec!['0'; n]; n];
    let mut r_restore = vec![vec!['0'; n]; n];
    let mut c_restore = vec![vec!['0'; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                h_restore[j][k] = h_restore[j][k].max(ret[i][j][k]);
                r_restore[i][k] = r_restore[i][k].max(ret[i][j][k]);
                c_restore[i][j] = c_restore[i][j].max(ret[i][j][k]);
            }
        }
    }

    if h != h_restore || r != r_restore || c != c_restore {
        writeln!(out, "NO").unwrap();
        return;
    }

    writeln!(out, "YES").unwrap();

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                write!(out, "{}", ret[i][j][k]).unwrap();
            }

            writeln!(out).unwrap();
        }
    }
}
