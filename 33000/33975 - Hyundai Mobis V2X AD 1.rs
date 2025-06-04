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
    let route1 = scan.token::<String>().chars().collect::<Vec<_>>();
    let route2 = scan.token::<String>().chars().collect::<Vec<_>>();

    let mut cnt_right1 = 0;
    let mut cnt_right2 = 0;
    let mut ret_left = 0i64;
    let mut ret_right = 0i64;

    for i in 0..2 * n - 1 {
        if route1[i] == 'R' {
            cnt_right1 += 1;
        }

        if route2[i] == 'R' {
            cnt_right2 += 1;
        }

        let diff = cnt_right1 - cnt_right2;

        if diff >= 0 {
            ret_left += diff + 1;
        }

        if diff <= 0 {
            ret_right += -diff + 1;
        }
    }

    writeln!(out, "{}", ret_left.min(ret_right)).unwrap();
}
