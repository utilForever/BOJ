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

    let mut print = |a: usize, b: usize| {
        writeln!(out, "{a} {b}").unwrap();
    };

    print(1, 3);
    print(2, 3);
    print(3, 4);
    print(4, 5);

    match n % 6 {
        0 => print(5, 6),
        1 => {
            print(4, 6);
            print(6, 7);
        }
        2 => {
            print(5, 6);
            print(5, 7);
            print(7, 8);
        }
        3 => {
            print(1, 6);
            print(2, 7);
            print(3, 8);
            print(8, 9);
        }
        4 => {
            print(5, 6);
            print(4, 7);
            print(7, 8);
            print(4, 9);
            print(9, 10);
        }
        5 => {
            print(1, 6);
            print(3, 7);
            print(7, 8);
            print(8, 9);
            print(9, 10);
            print(10, 11);
        }
        _ => unreachable!(),
    }

    let mut idx = n % 6 + 6;

    while idx < n {
        print(idx, idx + 1);
        print(idx, idx + 3);
        print(idx, idx + 5);
        print(idx + 1, idx + 2);
        print(idx + 3, idx + 4);
        print(idx + 5, idx + 6);

        idx += 6;
    }
}
