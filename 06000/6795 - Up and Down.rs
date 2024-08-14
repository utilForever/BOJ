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

    let (a, b, c, d, s) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut is_forward = true;
    let mut cnt_total = 0;
    let mut cnt_local = 0;
    let mut pos_nikky = 0;
    let mut pos_byron = 0;

    loop {
        pos_nikky += if is_forward { 1 } else { -1 };
        cnt_total += 1;
        cnt_local += 1;

        if cnt_total == s {
            break;
        }

        if is_forward && cnt_local == a {
            is_forward = false;
            cnt_local = 0;
        } else if !is_forward && cnt_local == b {
            is_forward = true;
            cnt_local = 0;
        }
    }

    is_forward = true;
    cnt_total = 0;
    cnt_local = 0;

    loop {
        pos_byron += if is_forward { 1 } else { -1 };
        cnt_total += 1;
        cnt_local += 1;

        if cnt_total == s {
            break;
        }

        if is_forward && cnt_local == c {
            is_forward = false;
            cnt_local = 0;
        } else if !is_forward && cnt_local == d {
            is_forward = true;
            cnt_local = 0;
        }
    }

    writeln!(
        out,
        "{}",
        if pos_nikky == pos_byron {
            "Tied"
        } else if pos_nikky > pos_byron {
            "Nikky"
        } else {
            "Byron"
        }
    )
    .unwrap();
}
