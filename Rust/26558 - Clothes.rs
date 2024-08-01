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

    let n = scan.token::<i64>();

    for i in 0..n {
        let c = scan.token::<i64>();
        let mut shirts = Vec::new();
        let mut pants = Vec::new();
        let mut socks = Vec::new();

        for _ in 0..c {
            let line = scan.line().trim().to_string();
            let line = line.split(" (").map(|x| x.to_string()).collect::<Vec<_>>();
            let (name, kind) = (line[0].clone(), line[1].split(")").collect::<Vec<_>>()[0]);

            match kind {
                "shirt" => shirts.push(name),
                "pants" => pants.push(name),
                "socks" => socks.push(name),
                _ => (),
            }
        }

        loop {
            if shirts.is_empty() || pants.is_empty() || socks.is_empty() {
                break;
            }

            let shirt = shirts.pop().unwrap();
            let pant = pants.pop().unwrap();
            let sock = socks.pop().unwrap();

            writeln!(out, "{shirt}, {pant}, {sock}").unwrap();
        }

        if i != n - 1 {
            writeln!(out).unwrap();
        }
    }
}
