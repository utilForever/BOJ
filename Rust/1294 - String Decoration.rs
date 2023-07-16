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
    let mut strings = vec![String::new(); n];

    for i in 0..n {
        strings[i] = scan.token::<String>();
    }

    strings.sort_by(|a, b| {
        let ab = a.clone() + b;
        let ba = b.clone() + a;

        ab.cmp(&ba)
    });

    let mut ret = String::new();

    while !strings.is_empty() {
        let char = strings[0].remove(0);
        ret.push(char);

        if strings[0].len() == 0 {
            strings.remove(0);
        }

        strings.sort_by(|a, b| {
            let ab = a.clone() + b;
            let ba = b.clone() + a;

            ab.cmp(&ba)
        });
    }

    writeln!(out, "{ret}").unwrap();
}
