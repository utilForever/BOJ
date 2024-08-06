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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut people = BTreeMap::new();

    loop {
        let s = scan.token::<String>();

        if s == "------" {
            break;
        }

        let (s, e, t) = (
            s.parse::<i64>().unwrap(),
            scan.token::<i64>(),
            scan.token::<String>(),
        );

        if people.contains_key(&t) {
            people.entry(t).and_modify(|x| *x += e - s);
        } else {
            people.insert(t, e - s);
        }
    }

    loop {
        let s = scan.token::<String>();

        if s == "======" {
            break;
        }

        let (s, e, t) = (
            s.parse::<i64>().unwrap(),
            scan.token::<i64>(),
            scan.token::<String>(),
        );

        if people.contains_key(&t) {
            people.entry(t).and_modify(|x| *x -= e - s);
        } else {
            people.insert(t, -(e - s));
        }
    }

    if people.iter().all(|(_, v)| *v == 0) {
        writeln!(out, "No differences found.").unwrap();
    } else {
        for (k, v) in people.iter() {
            if *v == 0 {
                continue;
            }

            if *v > 0 {
                writeln!(out, "{k} -{}", v.abs()).unwrap();
            } else {
                writeln!(out, "{k} +{}", v.abs()).unwrap();
            }
        }
    }
}
