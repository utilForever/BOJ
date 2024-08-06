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

    let (n, mut d) = (scan.token::<i64>(), scan.token::<i64>());
    let mut monsters = Vec::new();
    let mut equipments = Vec::new();

    for _ in 0..n {
        let (a, x) = (scan.token::<i64>(), scan.token::<i64>());

        if a == 1 {
            monsters.push(x);
        } else {
            equipments.push(x);
        }
    }

    monsters.sort();
    equipments.sort();

    let mut idx_monster = 0;
    let mut idx_equipment = 0;
    let mut ret = 0;

    loop {
        if idx_monster < monsters.len() && d > monsters[idx_monster] {
            if d > 1_000_000_000 {
                ret = monsters.len() + equipments.len();
                break;
            }

            d += monsters[idx_monster];
            idx_monster += 1;
            ret += 1;
        } else if idx_equipment < equipments.len() {
            d *= equipments[idx_equipment];
            idx_equipment += 1;
            ret += 1;
        } else {
            break;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
