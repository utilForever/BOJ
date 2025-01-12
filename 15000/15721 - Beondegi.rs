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

    let a = scan.token::<i64>();
    let t = scan.token::<i64>();
    let is_degi = if scan.token::<i64>() == 1 {
        true
    } else {
        false
    };
    let mut cnt_bbeon = 0;
    let mut cnt_degi = 0;
    let mut cnt_repeat = 1;
    let mut ret = -1;

    'outer: loop {
        for _ in 0..2 {
            cnt_bbeon += 1;
            ret = (ret + 1) % a;

            if !is_degi && cnt_bbeon == t {
                break 'outer;
            }

            cnt_degi += 1;
            ret = (ret + 1) % a;

            if is_degi && cnt_degi == t {
                break 'outer;
            }
        }

        for _ in 0..=cnt_repeat {
            cnt_bbeon += 1;
            ret = (ret + 1) % a;

            if !is_degi && cnt_bbeon == t {
                break 'outer;
            }
        }

        for _ in 0..=cnt_repeat {
            cnt_degi += 1;
            ret = (ret + 1) % a;

            if is_degi && cnt_degi == t {
                break 'outer;
            }
        }

        cnt_repeat += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
