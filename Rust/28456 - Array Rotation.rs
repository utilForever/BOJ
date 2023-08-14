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
    let mut array = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            array[i][j] = scan.token::<i64>();
        }
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 1 {
            let row = scan.token::<usize>() - 1;
            let temp = array[row][n - 1];

            for i in (1..n).rev() {
                array[row][i] = array[row][i - 1];
            }

            array[row][0] = temp;
        } else {
            let mut arr_new = vec![vec![0; n]; n];

            for i in 0..n {
                for j in 0..n {
                    arr_new[j][n - i - 1] = array[i][j];
                }
            }

            array = arr_new;
        }
    }

    for i in 0..n {
        for j in 0..n {
            write!(out, "{} ", array[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
