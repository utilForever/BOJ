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

    let (n, k, p) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut preferences = vec![vec![0; n + 1]; k + 1];

    for i in 1..=n {
        for j in 1..=k {
            let idx = scan.token::<usize>();
            preferences[idx][i] = j;
        }
    }

    let mut idx = (0..=k).collect::<Vec<usize>>();

    // NOTE: It produces an order in which problem i is more preferred than problem i + 1 (maybe indirectly)
    //       Even if i + 1 is more preferred than i in some other way,
    //       it doesn't really matter since there's a "plausible" chance i will be selected once again.
    // Reference: https://codeforces.com/blog/entry/65132?#comment-491317
    idx.sort_by(|&a, &b| {
        let mut cnt = 0;

        for i in 1..=n {
            if preferences[a][i] < preferences[b][i] {
                cnt += 1;
            }
        }

        let ret = cnt >= (n + 1) / 2;

        if ret {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Greater;
        }
    });

    for i in 1..=p {
        write!(out, "{} ", idx[i]).unwrap();
    }

    writeln!(out).unwrap();
}
