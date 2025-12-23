use std::{collections::HashMap, io, str};

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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let n = scan.token::<usize>();
    let q = (n + 1) / 2;
    let mut queries = Vec::with_capacity(q);

    if n % 2 == 0 {
        for i in 1..=q {
            let a = i;
            let b = q + i;
            let c = q + if i == 1 { q } else { i - 1 };

            queries.push((a, b, c));
        }
    } else {
        queries.push((1, q + 1, n));

        for i in 2..q {
            let a = i;
            let b = q + i;
            let c = q + i - 1;

            queries.push((a, b, c));
        }

        queries.push((q, n, q + 1));
    }

    let mut mask = vec![0; n + 1];
    let mut mapping = HashMap::new();

    for (idx, triangle) in queries.iter().enumerate() {
        mask[triangle.0] |= 1 << idx;
        mask[triangle.1] |= 1 << idx;
        mask[triangle.2] |= 1 << idx;

        println!("? 3 {} {} {}", triangle.0, triangle.1, triangle.2);

        let m = scan.token::<usize>();

        for _ in 0..m {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
            let entry = mapping.entry((x, y)).or_insert(0i64);

            *entry |= 1 << idx;
        }
    }

    let mut ret = vec![(0, 0); n + 1];

    for (&coord, &val) in mapping.iter() {
        if let Some(idx) = mask.iter().position(|&m| m == val) {
            ret[idx] = coord;
        }
    }

    print!("!");

    for i in 1..=n {
        print!(" {} {}", ret[i].0, ret[i].1);
    }

    println!();
}
