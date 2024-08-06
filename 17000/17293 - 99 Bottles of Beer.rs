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

    let n = scan.token::<i64>();

    for i in (2..=n).rev() {
        writeln!(
            out,
            "{} bottles of beer on the wall, {} bottles of beer.",
            i, i
        )
        .unwrap();
        writeln!(
            out,
            "Take one down and pass it around, {} {} of beer on the wall.",
            i - 1,
            if i > 2 { "bottles" } else { "bottle" }
        )
        .unwrap();
        writeln!(out).unwrap();
    }

    writeln!(out, "1 bottle of beer on the wall, 1 bottle of beer.").unwrap();
    writeln!(
        out,
        "Take one down and pass it around, no more bottles of beer on the wall."
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(
        out,
        "No more bottles of beer on the wall, no more bottles of beer."
    )
    .unwrap();
    writeln!(
        out,
        "Go to the store and buy some more, {} {} of beer on the wall.",
        n,
        if n > 1 { "bottles" } else { "bottle" }
    )
    .unwrap();
}
