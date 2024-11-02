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

    let n = scan.token::<usize>();
    let basis_jeonghoon = scan.token::<String>().chars().collect::<Vec<_>>();
    let key_jeonghoon = scan.token::<String>().chars().collect::<Vec<_>>();
    let basis_ian = scan.token::<String>().chars().collect::<Vec<_>>();
    let key_ian = scan.token::<String>().chars().collect::<Vec<_>>();

    let mut ret = true;
    let mut key = String::new();

    for i in 0..n {
        if basis_jeonghoon[i] != basis_ian[i] {
            continue;
        }

        if key_jeonghoon[i] != key_ian[i] {
            ret = false;
            break;
        }

        key.push(key_jeonghoon[i]);
    }

    if ret {
        writeln!(out, "{key}").unwrap();
    } else {
        writeln!(out, "htg!").unwrap();
    }
}
