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

    let n = scan.token::<usize>();
    let mut votes = vec![0; n];
    let mut cnt_koo = 0;

    for i in 0..n {
        votes[i] = scan.token::<i64>();

        if votes[i] == 1 {
            cnt_koo += 1;
        }
    }

    if cnt_koo >= (n + 1) / 2 {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut cnt_koo_left = 0;
    let mut cnt_koo_right = 0;

    for i in 0..n {
        if votes[i] == 1 {
            cnt_koo_left += 1;
        }

        if votes[n - 1 - i] == 1 {
            cnt_koo_right += 1;
        }

        if cnt_koo_left >= (i + 2) / 2 || cnt_koo_right >= (i + 2) / 2 {
            writeln!(out, "1").unwrap();
            return;
        }
    }

    writeln!(out, "2").unwrap();
}
