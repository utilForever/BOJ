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

    let t = scan.token::<i64>();

    for i in 1..=t {
        let n = scan.token::<usize>();
        let mut marbles = vec![0; n + 1];

        for j in 1..=n {
            marbles[j] = scan.token::<i64>();
        }

        let mut children = vec![Vec::new(); n + 1];

        for j in 1..=n {
            let circle = scan.token::<usize>();

            if circle > 0 {
                children[circle].push(j);
            }
        }

        let mut grundy = vec![0; n + 1];
        let mut mex_marked = vec![0; n + 1];
        let mut mex_idx = 1;

        for j in (1..=n).rev() {
            mex_idx += 1;

            for &child in children[j].iter() {
                mex_marked[grundy[child]] = mex_idx;
            }

            let mut g = 0;

            loop {
                if mex_marked[g] != mex_idx {
                    break;
                }

                g += 1;
            }

            grundy[j] = g;
        }

        let mut ret = 0;

        for j in 1..=n {
            if marbles[j] % 2 == 1 {
                ret ^= grundy[j];
            }
        }

        writeln!(
            out,
            "Case #{i}: {}",
            if ret != 0 { "first" } else { "second" }
        )
        .unwrap();
    }
}
