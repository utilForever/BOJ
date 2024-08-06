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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, e) = (scan.token::<usize>(), scan.token::<i64>());
        let mut people = vec![-1; n];

        for i in 0..e {
            let idx = scan.token::<usize>() - 1;
            people[idx] = i;
        }

        let mut ret = None;

        for i in 0..n {
            if people[i] == -1 {
                ret = Some(i + 1);
            }
        }

        if ret.is_some() {
            writeln!(out, "{}", ret.unwrap()).unwrap();
            continue;
        }

        let max = people.iter().min().unwrap();

        writeln!(
            out,
            "{}",
            people.iter().position(|&x| x == *max).unwrap() + 1
        )
        .unwrap();
    }
}
