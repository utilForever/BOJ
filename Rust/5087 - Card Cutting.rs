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

    let mut ret_cheryl = 0;
    let mut ret_tania = 0;

    loop {
        let s = scan.token::<String>();

        if s == "#" {
            break;
        } else if s == "*" {
            writeln!(
                out,
                "{}",
                if ret_cheryl > ret_tania {
                    "Cheryl"
                } else if ret_cheryl < ret_tania {
                    "Tania"
                } else {
                    "Draw"
                }
            )
            .unwrap();

            ret_cheryl = 0;
            ret_tania = 0;
        } else {
            match s.as_str() {
                "A" | "3" | "5" | "7" | "9" => {
                    ret_cheryl += 1;
                }
                "2" | "4" | "6" | "8" | "10" => {
                    ret_tania += 1;
                }
                _ => unreachable!(),
            }
        }
    }
}
