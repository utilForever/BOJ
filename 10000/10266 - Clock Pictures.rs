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
    let mut clock1 = vec![0; n];
    let mut clock2 = vec![0; n];

    for i in 0..n {
        clock1[i] = scan.token::<usize>();
    }

    for i in 0..n {
        clock2[i] = scan.token::<usize>();
    }

    clock1.sort();
    clock2.sort();

    let mut converted_clock1 = vec![0; n * 2];
    let mut converted_clock2 = vec![0; n];

    converted_clock1[0] = 360_000 - (clock1[n - 1] - clock1[0]);
    converted_clock2[0] = 360_000 - (clock2[n - 1] - clock2[0]);

    for i in 1..n {
        converted_clock1[i] = clock1[i] - clock1[i - 1];
        converted_clock2[i] = clock2[i] - clock2[i - 1];
    }

    for i in 0..n {
        converted_clock1[n + i] = converted_clock1[i];
    }

    let mut cmp = 0;
    let mut fail = vec![0; n];

    for i in 1..converted_clock2.len() - 1 {
        while cmp > 0 && converted_clock2[cmp] != converted_clock2[i] {
            cmp = fail[cmp - 1];
        }

        if converted_clock2[cmp] == converted_clock2[i] {
            cmp += 1;
            fail[i] = cmp;
        }
    }

    let mut result = Vec::new();
    cmp = 0;

    for i in 0..converted_clock1.len() - 1 {
        while cmp > 0 && converted_clock1[i] != converted_clock2[cmp] {
            cmp = fail[cmp - 1];
        }

        if converted_clock1[i] == converted_clock2[cmp] {
            if cmp == converted_clock2.len() - 2 {
                result.push(i - cmp + 1);
                cmp = fail[cmp];
            } else {
                cmp += 1;
            }
        }
    }

    writeln!(
        out,
        "{}",
        if result.is_empty() {
            "impossible"
        } else {
            "possible"
        }
    )
    .unwrap();
}
