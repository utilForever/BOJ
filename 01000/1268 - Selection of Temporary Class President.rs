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
    let mut table = vec![vec![0; 5]; n];

    for i in 0..n {
        for j in 0..5 {
            table[i][j] = scan.token::<i64>();
        }
    }

    let mut friends = vec![vec![false; n]; n];

    for i in 0..5 {
        let students = table.iter().map(|x| x[i]).collect::<Vec<i64>>();

        for (j, student) in students.iter().enumerate() {
            for (k, other) in students.iter().enumerate() {
                if j == k {
                    continue;
                }

                if student != other {
                    continue;
                }

                friends[j][k] = true;
            }
        }
    }

    let mut friends_max = 0;
    let mut ret = 1;

    for i in 0..n {
        let friends_cnt = friends[i].iter().filter(|x| **x).count();

        if friends_cnt > friends_max {
            friends_max = friends_cnt;
            ret = i + 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}