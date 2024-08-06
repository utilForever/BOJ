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

    let (a, b) = (scan.token::<i64>(), scan.token::<i64>());
    let (c, d) = (scan.token::<i64>(), scan.token::<i64>());
    let k = scan.token::<i64>();

    let mut pos_manga = a;
    let mut pos_doldol = a + c;

    if k == 0 {
        let cnt_manga = (pos_manga + b - 1) / b;
        let cnt_doldol = (pos_doldol + d - 1) / d;

        writeln!(
            out,
            "{}",
            if cnt_manga < cnt_doldol {
                cnt_manga
            } else {
                -1
            }
        )
        .unwrap();
        return;
    }

    let mut speed = b;
    let mut ret = 0;

    while pos_manga > 0 && pos_manga <= pos_doldol && speed > 0 {
        pos_manga -= speed;
        pos_doldol -= d;
        speed -= k;
        ret += 1;
    }

    writeln!(
        out,
        "{}",
        if pos_manga <= 0 && pos_doldol > 0 {
            ret
        } else {
            -1
        }
    )
    .unwrap();
}
