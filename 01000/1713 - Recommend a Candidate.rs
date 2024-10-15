use io::Write;
use std::{collections::BTreeMap, io, str};

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
    let cnt = scan.token::<i64>();
    let mut photos = BTreeMap::new();

    for i in 0..cnt {
        let num = scan.token::<usize>();

        if !photos.contains_key(&num) {
            if photos.len() < n {
                photos.insert(num, (1, i));
            } else {
                let (&k1, &v1) = photos.iter().next().unwrap();
                let (mut k1, mut v1) = (k1, v1);

                for (&k2, &v2) in photos.iter() {
                    if v1 > v2 {
                        k1 = k2;
                        v1 = v2;
                    }
                }

                photos.remove(&k1);
                photos.insert(num, (1, i));
            }
        } else {
            photos.entry(num).and_modify(|(vote, _)| {
                *vote += 1;
            });
        }
    }

    for (num, _) in photos.iter() {
        write!(out, "{num} ").unwrap();
    }

    writeln!(out).unwrap();
}
