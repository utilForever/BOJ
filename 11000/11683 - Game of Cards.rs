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

    let (p, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut ret = 0;

    for _ in 0..p {
        let n = scan.token::<usize>();
        let mut cards = vec![0; n];

        for i in 0..n {
            cards[i] = scan.token::<usize>();
        }

        let mut grundy = vec![0; n + 1];

        for i in 1..=n {
            let max = k.min(i - 1);
            let mut visited = vec![false; k + 2];

            for j in 0..=max {
                let remain = i - j;
                let idx_top = remain - 1;

                if cards[idx_top] <= remain {
                    let remain = remain - cards[idx_top];

                    if grundy[remain] <= k + 1 {
                        visited[grundy[remain]] = true;
                    }
                }
            }

            let mut mex = 0;

            while mex < visited.len() && visited[mex] {
                mex += 1;
            }

            grundy[i] = mex;
        }

        ret ^= grundy[n];
    }

    writeln!(
        out,
        "{}",
        if ret != 0 {
            "Alice can win."
        } else {
            "Bob will win."
        }
    )
    .unwrap();
}
