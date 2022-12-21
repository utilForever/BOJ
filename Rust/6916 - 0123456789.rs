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

    let num = scan.token::<i64>();

    writeln!(
        out,
        "{}",
        match num {
            0 => {
                " * * *\n*     *\n*     *\n*     *\n\n*     *\n*     *\n*     *\n * * *"
            }
            1 => {
                "\n      *\n      *\n      *\n\n      *\n      *\n      *\n"
            }
            2 => {
                " * * *\n      *\n      *\n      *\n * * *\n*\n*\n*\n * * *"
            }
            3 => {
                " * * *\n      *\n      *\n      *\n * * *\n      *\n      *\n      *\n * * *"
            }
            4 => {
                "\n*     *\n*     *\n*     *\n * * *\n      *\n      *\n      *\n"
            }
            5 => {
                " * * *\n*\n*\n*\n * * *\n      *\n      *\n      *\n * * *"
            }
            6 => {
                " * * *\n*\n*\n*\n * * *\n*     *\n*     *\n*     *\n * * *"
            }
            7 => {
                " * * *\n      *\n      *\n      *\n\n      *\n      *\n      *\n"
            }
            8 => {
                " * * *\n*     *\n*     *\n*     *\n * * *\n*     *\n*     *\n*     *\n * * *"
            }
            9 => {
                " * * *\n*     *\n*     *\n*     *\n * * *\n      *\n      *\n      *\n * * *"
            }
            _ => unreachable!(),
        }
    )
    .unwrap();
}
