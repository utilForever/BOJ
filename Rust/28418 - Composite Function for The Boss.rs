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

    let (f_a, f_b, f_c) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let (g_a, g_b) = (scan.token::<i64>(), scan.token::<i64>());

    let p_a = f_a * g_a * g_a;
    let p_b = 2 * f_a * g_a * g_b + f_b * g_a;
    let p_c = f_a * g_b * g_b + f_b * g_b + f_c;
    let q_a = f_a * g_a;
    let q_b = f_b * g_a;
    let q_c = f_c * g_a + g_b;

    let ret_a = p_a - q_a;
    let ret_b = p_b - q_b;
    let ret_c = p_c - q_c;
    let determinant = ret_b * ret_b - 4 * ret_a * ret_c;

    writeln!(
        out,
        "{}",
        if ret_a == 0 {
            if ret_b == 0 {
                if ret_c == 0 {
                    "Nice"
                } else {
                    "Head on"
                }
            } else {
                "Remember my character"
            }
        } else {
            if determinant > 0 {
                "Go ahead"
            } else if determinant == 0 {
                "Remember my character"
            } else {
                "Head on"
            }
        }
    )
    .unwrap();
}
