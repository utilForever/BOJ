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

struct FenwickTree {
    n: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            n,
            data: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut idx: usize, delta: i64) {
        while idx <= self.n {
            self.data[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut ret = 0;

        while idx > 0 {
            ret += self.data[idx];
            idx -= idx & (!idx + 1);
        }

        ret
    }
}

fn gcd(mut first: i64, mut second: i64) -> i64 {
    if first < 0 {
        first = -first;
    }

    if second < 0 {
        second = -second;
    }

    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        std::mem::swap(&mut min, &mut max);
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn gcd_extended(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = gcd_extended(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let (_, x, _) = gcd_extended(a, m);
    (x % m + m) % m
}

#[derive(Clone, Copy)]
enum Feasible {
    None,
    Single(i64),
    Class { left: i64, right: i64 },
}

fn init_state(n: i64, r: i64, m: i64) -> Feasible {
    if n > m {
        let val = if r == 0 { n } else { r };
        Feasible::Single(val)
    } else {
        Feasible::Class { left: n, right: r }
    }
}

fn merge_with(feas: Feasible, n: i64, d: i64, m: i64) -> Option<Feasible> {
    match feas {
        Feasible::None => Some(init_state(n, d, m)),
        Feasible::Single(val) => {
            if (val % n + n) % n == d {
                Some(Feasible::Single(val))
            } else {
                None
            }
        }
        Feasible::Class { left, right } => {
            let g = gcd(left, n);
            let diff = (right - d).rem_euclid(g);

            if diff != 0 {
                return None;
            }

            let l0 = left / g;
            let n0 = n / g;

            if n0 == 1 {
                return Some(Feasible::Class {
                    left,
                    right: right.rem_euclid(left),
                });
            }

            let delta = (d - right) / g;
            let inv = mod_inv((l0 % n0 + n0) % n0, n0);
            let t = ((delta % n0 + n0) % n0) * inv % n0;

            let lp = left * n0;
            let mut rp = (right + left * t) % lp;

            if rp < 0 {
                rp += lp;
            }

            if lp <= m {
                Some(Feasible::Class {
                    left: lp,
                    right: rp,
                })
            } else {
                let mut rpos = rp;

                if rpos == 0 {
                    rpos = lp;
                }
                if rpos <= m {
                    Some(Feasible::Single(rpos))
                } else {
                    None
                }
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut fenwick_tree = FenwickTree::new(n);

    for i in 1..=n {
        fenwick_tree.update(i, 1);
    }

    let mut prev = n;
    let mut feasible = Feasible::None;
    let mut ret = 0;

    for i in 0..n {
        let num = scan.token::<usize>();
        let mut cnt_survived = if num > prev {
            fenwick_tree.query(num) - fenwick_tree.query(prev)
        } else {
            fenwick_tree.query(n) - fenwick_tree.query(prev) + fenwick_tree.query(num)
        };
        let diff = (n - i) as i64;

        cnt_survived %= diff;

        match merge_with(feasible, diff, cnt_survived, m) {
            Some(feasible_new) => feasible = feasible_new,
            None => {
                ret += 1;
                feasible = init_state(diff, cnt_survived, m);
            }
        }

        fenwick_tree.update(num, -1);
        prev = num;
    }

    writeln!(out, "{ret}").unwrap();
}
