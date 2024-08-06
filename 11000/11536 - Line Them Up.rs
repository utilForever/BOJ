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
    let mut names = vec![String::new(); n];
    let mut names_asceding = vec![String::new(); n];
    let mut names_descending = vec![String::new(); n];

    for i in 0..n {
        names[i] = scan.token::<String>();
        names_asceding[i] = names[i].clone();
        names_descending[i] = names[i].clone();
    }

    names_asceding.sort();
    names_descending.sort_by(|a, b| b.cmp(a));

    writeln!(
        out,
        "{}",
        if names == names_asceding {
            "INCREASING"
        } else if names == names_descending {
            "DECREASING"
        } else {
            "NEITHER"
        }
    )
    .unwrap();
}
