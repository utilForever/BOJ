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

    let n = scan.token::<usize>();
    let mut records = vec![Vec::new(); 70];

    for _ in 0..n {
        let (s, w, d, p) = (
            scan.token::<String>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
        );

        records[((w - 1) * 7) + d].push((s, p));
    }

    let mut record_book = BTreeMap::new();

    for _ in 0..n {
        record_book.insert(scan.token::<String>(), scan.token::<i64>());
    }

    let mut cnt = 0;
    let mut ret = 0;

    for i in 0..70 {
        let mut is_exist = false;

        for (s, p) in records[i].iter() {
            let money = *record_book.get(s).unwrap();

            if money >= *p {
                is_exist = true;
            }
        }

        if is_exist {
            cnt += 1;
        } else {
            ret = ret.max(cnt);
            cnt = 0;
        }
    }

    ret = ret.max(cnt);

    writeln!(out, "{ret}").unwrap();
}
