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

    let h_start = scan.token::<i64>();
    let _ = scan.token::<String>();
    let m_start = scan.token::<i64>();
    let _ = scan.token::<String>();
    let s_start = scan.token::<i64>();

    let h_end = scan.token::<i64>();
    let _ = scan.token::<String>();
    let m_end = scan.token::<i64>();
    let _ = scan.token::<String>();
    let s_end = scan.token::<i64>();

    let time_start = h_start * 3600 + m_start * 60 + s_start;
    let time_end = h_end * 3600 + m_end * 60 + s_end;

    writeln!(
        out,
        "{}",
        if time_start > time_end {
            24 * 3600 - time_start + time_end
        } else {
            time_end - time_start
        }
    )
    .unwrap();
}
