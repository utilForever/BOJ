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

    let mut resistances: BTreeMap<&str, i64> = BTreeMap::new();
    resistances.insert("black", 0);
    resistances.insert("brown", 1);
    resistances.insert("red", 2);
    resistances.insert("orange", 3);
    resistances.insert("yellow", 4);
    resistances.insert("green", 5);
    resistances.insert("blue", 6);
    resistances.insert("violet", 7);
    resistances.insert("grey", 8);
    resistances.insert("white", 9);

    let mut sum = 0;

    for i in 0..3 {
        let color = scan.token::<String>();
        let resistance = resistances.get(&color.as_str()).unwrap();

        match i {
            0 => {
                sum += 10 * resistance;
            }
            1 => {
                sum += resistance;
            }
            2 => {
                sum *= 10_i64.pow(*resistance as u32);
            }
            _ => unimplemented!(),
        }
    }

    writeln!(out, "{}", sum).unwrap();
}
