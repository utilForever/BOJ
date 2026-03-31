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

    let n = scan.token::<usize>();
    let mut stones = vec![0; n];

    for i in 0..n {
        stones[i] = scan.token::<i64>();
    }

    let mut cnt_different = 0;
    let mut cnt_same_zero = 0;

    for i in 0..n / 2 {
        if stones[i] != stones[n - 1 - i] {
            cnt_different += 1;
        } else if stones[i] == 0 {
            cnt_same_zero += 1;
        }
    }

    let cnt_middle_zero = if n % 2 == 1 && stones[n / 2] == 0 {
        1
    } else {
        0
    };
    let ret = if cnt_same_zero == 0 {
        cnt_different + cnt_middle_zero >= 3
    } else {
        cnt_different + cnt_middle_zero > 0
    };

    writeln!(out, "{}", if ret { "PONIX" } else { "DALGU" }).unwrap();
}
