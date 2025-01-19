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

struct YoungTableaux {
    table: [BTreeMap<i64, i64>; 5],
}

impl YoungTableaux {
    fn new() -> Self {
        Self {
            table: [
                BTreeMap::new(),
                BTreeMap::new(),
                BTreeMap::new(),
                BTreeMap::new(),
                BTreeMap::new(),
            ],
        }
    }

    fn insert(&mut self, idx: usize, num: i64, mut cnt: i64, sum: &mut i64) {
        if idx == 5 {
            return;
        }

        *self.table[idx].entry(num).or_insert(0) += cnt;
        *sum += cnt;

        let keys_to_traverse = self.table[idx]
            .range((num + 1)..)
            .map(|(k, _)| *k)
            .collect::<Vec<i64>>();

        for x in keys_to_traverse {
            if cnt == 0 {
                break;
            }

            let y = *self.table[idx].get_mut(&x).unwrap();
            let t = y.min(cnt);

            self.insert(idx + 1, x, t, sum);

            *sum -= t;
            self.table[idx].entry(x).and_modify(|e| *e -= t);

            if *self.table[idx].get(&x).unwrap() == 0 {
                self.table[idx].remove(&x);
            }

            cnt -= t;
        }
    }
}

// Reference: https://codeforces.com/blog/entry/98167
// Reference: https://youngyojun.github.io/secmem/2021/09/19/young-tableaux/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut young_tableaux = YoungTableaux::new();
        let mut ret = 0;

        for _ in 0..n {
            let val = scan.token::<i64>();
            young_tableaux.insert(0, val, val, &mut ret);

            write!(out, "{ret} ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
