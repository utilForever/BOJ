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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (mut n, mut m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut table = vec![vec![' '; m]; n];

    for i in 0..n {
        let line = scan.line().trim().to_string();

        for (j, c) in line.chars().enumerate() {
            table[i][j] = c;
        }
    }

    if n == 1 {
        let mut table_new = vec![vec![' '; 1]; m];

        for i in 0..m {
            table_new[i][0] = table[0][i];
        }

        table = table_new;
        std::mem::swap(&mut n, &mut m);
    }

    let mut ret = Vec::new();

    for j in 0..m {
        let mut i = 0;

        while i < n {
            if table[i][j] == 'O' {
                i += 1;
                continue;
            }

            if i + 1 < n && table[i + 1][j] == 'X' {
                ret.push(format!("2 {} {}", (i + 1) + n * j, (i + 2) + n * j));
                i += 2;
            } else {
                ret.push(format!("1 {}", (i + 1) + n * j));
                i += 1;
            }
        }
    }

    writeln!(out, "{}", ret.len()).unwrap();

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
