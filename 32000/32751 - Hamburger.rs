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
    let (a, b, c, d) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let hamburger = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut cnt = [0, 0, 0, 0];

    for i in 0..n {
        match hamburger[i] {
            'a' => cnt[0] += 1,
            'b' => cnt[1] += 1,
            'c' => cnt[2] += 1,
            'd' => cnt[3] += 1,
            _ => unreachable!(),
        }
    }

    // if cnt[0] > a or cnt[1] > b or cnt[2] > c or cnt[3] > d, always "No"
    if cnt[0] > a || cnt[1] > b || cnt[2] > c || cnt[3] > d {
        writeln!(out, "No").unwrap();
        return;
    }

    // if n == 1, check if hamburger[0] == 'a' or not
    if n == 1 {
        writeln!(out, "{}", if hamburger[0] == 'a' { "Yes" } else { "No" }).unwrap();
        return;
    }

    // if n == 2, always "No"
    if n == 2 {
        writeln!(out, "No").unwrap();
        return;
    }

    // if n >= 3, check hamburger[0] and hamburger[n - 1] are 'a' or not
    if hamburger[0] != 'a' || hamburger[n - 1] != 'a' {
        writeln!(out, "No").unwrap();
        return;
    }

    let mut check = true;

    hamburger.windows(2).for_each(|w| {
        if w[0] == w[1] {
            check = false;
        }
    });

    writeln!(out, "{}", if check { "Yes" } else { "No" }).unwrap();
}
