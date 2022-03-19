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

    let num_order = scan.token::<usize>();
    let mut arr = vec![0; 21];

    for _ in 0..num_order {
        let order = scan.token::<String>();

        match order.as_str() {
            "add" => {
                let num = scan.token::<usize>();
                arr[num] = 1;
            }
            "remove" => {
                let num = scan.token::<usize>();
                arr[num] = 0;
            }
            "check" => {
                let num = scan.token::<usize>();
                if arr[num] == 1 {
                    writeln!(out, "1").unwrap();
                } else {
                    writeln!(out, "0").unwrap();
                }
            }
            "toggle" => {
                let num = scan.token::<usize>();
                arr[num] ^= 1;
            }
            "all" => {
                arr.iter_mut().for_each(|x| *x = 1);
            }
            "empty" => {
                arr.iter_mut().for_each(|x| *x = 0);
            }
            _ => {
                unimplemented!();
            }
        }
    }
}
