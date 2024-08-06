use io::Write;
use std::{collections::BinaryHeap, io, str, cmp::Reverse};

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
    let mut d = vec![0; n - 1];
    let mut g = vec![0; n];

    for i in 0..n - 1 {
        d[i] = scan.token::<usize>();
    }

    for i in 0..n {
        g[i] = scan.token::<usize>();
    }

    let mut convex1: BinaryHeap<usize> = BinaryHeap::new();
    let mut convex2: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut cost_carry = 0;
    let mut ret = 0;

    convex1.push(g[0]);
    convex2.push(Reverse(g[0]));

    // Using slope trick
    for i in 0..n - 1 {
        ret += (i + 1) * d[i];
        cost_carry += d[i];

        if g[i + 1] < convex2.peek().unwrap().0 + cost_carry {
            convex1.push(g[i + 1] + cost_carry);
            convex1.push(g[i + 1] + cost_carry);
            convex2.push(Reverse(convex1.peek().unwrap() - 2 * cost_carry));
            convex1.pop();
        } else {
            convex2.push(Reverse(g[i + 1] - cost_carry));
            convex2.push(Reverse(g[i + 1] - cost_carry));
            convex1.push(convex2.peek().unwrap().0 + 2 * cost_carry);
            convex2.pop();
        }
    }

    while !convex1.is_empty() {
        let val = convex1.pop().unwrap();
        ret += val - cost_carry;
    }

    writeln!(out, "{ret}").unwrap();
}
