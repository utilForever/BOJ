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

    let (mut flour, mut chocolate, mut egg, mut buffer) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let (need_flour, need_chocolate, need_egg, need_buffer) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut cnt_cookie = 0;

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (cmd, val) = (scan.token::<i64>(), scan.token::<i64>());

        match cmd {
            1 => {
                if flour >= val * need_flour
                    && chocolate >= val * need_chocolate
                    && egg >= val * need_egg
                    && buffer >= val * need_buffer
                {
                    flour -= val * need_flour;
                    chocolate -= val * need_chocolate;
                    egg -= val * need_egg;
                    buffer -= val * need_buffer;
                    cnt_cookie += val;

                    writeln!(out, "{cnt_cookie}").unwrap();
                } else {
                    writeln!(out, "Hello, siumii").unwrap();
                }
            }
            2 => {
                flour += val;
                writeln!(out, "{flour}").unwrap();
            }
            3 => {
                chocolate += val;
                writeln!(out, "{chocolate}").unwrap();
            }
            4 => {
                egg += val;
                writeln!(out, "{egg}").unwrap();
            }
            5 => {
                buffer += val;
                writeln!(out, "{buffer}").unwrap();
            }
            _ => unreachable!(),
        }
    }
}
