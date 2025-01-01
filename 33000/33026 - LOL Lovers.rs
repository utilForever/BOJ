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
    let s = scan.token::<String>();

    let cnt_l = s.matches('L').count();
    let cnt_o = s.matches('O').count();

    let mut prefix_l = vec![0; n + 1];
    let mut prefix_o = vec![0; n + 1];

    for (i, ch) in s.chars().enumerate() {
        prefix_l[i + 1] = prefix_l[i] + if ch == 'L' { 1 } else { 0 };
        prefix_o[i + 1] = prefix_o[i] + if ch == 'O' { 1 } else { 0 };
    }

    for k in 1..n {
        let left_l = prefix_l[k];
        let left_o = prefix_o[k];
        let right_l = cnt_l - left_l;
        let right_o = cnt_o - left_o;

        if left_l != right_l && left_o != right_o {
            writeln!(out, "{k}").unwrap();
            return;
        }
    }

    writeln!(out, "-1").unwrap();
}
