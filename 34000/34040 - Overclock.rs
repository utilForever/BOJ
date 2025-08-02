use io::Write;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::ops::{Add, Div, Mul, Sub};
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

#[derive(Debug, Clone, Copy)]
struct Fraction {
    num: i64,
    den: i64,
}

impl Fraction {
    fn new(num: i64, den: i64) -> Self {
        assert!(den != 0);

        let mut n = num;
        let mut d = den;

        if d < 0 {
            n = -n;
            d = -d;
        }

        let g = gcd(n, d);

        Fraction {
            num: n / g,
            den: d / g,
        }
    }

    fn zero() -> Self {
        Fraction { num: 0, den: 1 }
    }

    fn one() -> Self {
        Fraction { num: 1, den: 1 }
    }

    fn is_zero(&self) -> bool {
        self.num == 0
    }

    fn positive(&self) -> bool {
        self.num > 0
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Fraction) -> Fraction {
        Fraction::new(self.num * rhs.den + rhs.num * self.den, self.den * rhs.den)
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Fraction) -> Fraction {
        Fraction::new(self.num * rhs.den - rhs.num * self.den, self.den * rhs.den)
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Fraction {
        Fraction::new(self.num * rhs.num, self.den * rhs.den)
    }
}

impl Div for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Fraction) -> Fraction {
        Fraction::new(self.num * rhs.den, self.den * rhs.num)
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.den == other.den
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.num as i64 * other.den).partial_cmp(&(other.num as i64 * self.den))
    }
}

#[derive(Default, Clone)]
struct Factory {
    input: i64,
    output: i64,
    dest: Option<usize>,
    pred: Vec<usize>,
}

fn process_dfs(
    factories: &Vec<Factory>,
    stack: &mut Vec<usize>,
    idx: &mut usize,
    dfn: &mut Vec<usize>,
    low: &mut Vec<usize>,
    in_stack: &mut Vec<bool>,
    scc_id: &mut Vec<usize>,
    scc_group: &mut Vec<Vec<usize>>,
    curr: usize,
) {
    stack.push(curr);
    in_stack[curr] = true;

    *idx += 1;
    dfn[curr] = *idx;
    low[curr] = *idx;

    if let Some(next) = factories[curr].dest {
        if dfn[next] == 0 {
            process_dfs(
                factories, stack, idx, dfn, low, in_stack, scc_id, scc_group, next,
            );
            low[curr] = low[curr].min(low[next]);
        } else if in_stack[next] {
            low[curr] = low[curr].min(dfn[next]);
        }
    }

    if dfn[curr] == low[curr] {
        let mut scc = Vec::new();

        loop {
            let next = stack.pop().unwrap();

            in_stack[next] = false;
            scc_id[next] = scc_group.len();
            scc.push(next);

            if next == curr {
                break;
            }
        }

        scc_group.push(scc);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut factories = vec![Factory::default(); n];

    for i in 0..n {
        let (input, output) = (scan.token::<i64>(), scan.token::<i64>());
        factories[i].input = input;
        factories[i].output = output;

        if output != 0 {
            let dest = scan.token::<usize>() - 1;
            factories[i].dest = Some(dest);
        }
    }

    for i in 0..factories.len() {
        let (u, factory) = (i, &factories[i]);

        if let Some(v) = factory.dest {
            factories[v].pred.push(u);
        }
    }

    for i in 0..n {
        if factories[i].input == 0 && !factories[i].pred.is_empty() {
            writeln!(out, "stop").unwrap();
            return;
        }

        if factories[i].input > 0 && factories[i].pred.is_empty() {
            writeln!(out, "stop").unwrap();
            return;
        }
    }

    let mut stack = Vec::new();
    let mut idx = 0;
    let mut dfn = vec![0; n];
    let mut low = vec![0; n];
    let mut in_stack = vec![false; n];
    let mut scc_id = vec![0; n];
    let mut scc_group = Vec::new();

    for i in 0..n {
        if dfn[i] == 0 {
            process_dfs(
                &factories,
                &mut stack,
                &mut idx,
                &mut dfn,
                &mut low,
                &mut in_stack,
                &mut scc_id,
                &mut scc_group,
                i,
            );
        }
    }

    let mut dag = vec![Vec::new(); scc_group.len()];
    let mut in_degree = vec![0; scc_group.len()];

    for curr in 0..n {
        if let Some(next) = factories[curr].dest {
            let pos_curr = scc_id[curr];
            let pos_next = scc_id[next];

            if pos_curr == pos_next {
                continue;
            }

            dag[pos_curr].push(pos_next);
            in_degree[pos_next] += 1;
        }
    }

    let mut vals = vec![None; n];
    let mut queue = VecDeque::new();

    for idx in 0..scc_group.len() {
        if in_degree[idx] == 0 {
            queue.push_back(idx);
        }
    }

    while let Some(idx) = queue.pop_front() {
        let scc = &scc_group[idx];

        let is_cyclic = scc.len() > 1 || {
            let curr = scc[0];

            if let Some(next) = factories[curr].dest {
                next == curr
            } else {
                false
            }
        };

        if is_cyclic {
            let mut cycle = Vec::new();
            let start = scc[0];
            let mut curr = start;

            loop {
                cycle.push(curr);
                curr = factories[curr].dest.unwrap();

                if curr == start {
                    break;
                }
            }

            let m = cycle.len();
            let mut a = vec![Fraction::zero(); m];
            let mut c = vec![Fraction::zero(); m];

            for i in 0..m {
                let curr = cycle[i];
                let prev = cycle[(i + m - 1) % m];

                if factories[curr].input == 0 {
                    writeln!(out, "stop").unwrap();
                    return;
                }

                a[i] = Fraction::new(factories[prev].output, factories[curr].input);

                let mut s = Fraction::zero();

                for &p in factories[curr].pred.iter() {
                    if scc_id[p] != idx {
                        s = s + vals[p].unwrap() * Fraction::new(factories[p].output, 1);
                    }
                }

                c[i] = s / Fraction::new(factories[curr].input, 1);
            }

            let mut coeff = Fraction::one();
            let mut cons = Fraction::zero();

            for idx in 1..m {
                coeff = a[idx] * coeff;
                cons = a[idx] * cons + c[idx];
            }

            let denom = Fraction::one() - a[0] * coeff;
            let rhs = a[0] * cons + c[0];

            let k0: Fraction;

            if denom.is_zero() {
                if !rhs.is_zero() {
                    writeln!(out, "stop").unwrap();
                    return;
                }

                k0 = Fraction::one();
            } else {
                k0 = rhs / denom;

                if !k0.positive() {
                    writeln!(out, "stop").unwrap();
                    return;
                }
            }

            let mut kv = vec![Fraction::zero(); m];
            kv[0] = k0;

            for idx in 1..m {
                kv[idx] = a[idx] * kv[idx - 1] + c[idx];

                if !kv[idx].positive() {
                    writeln!(out, "stop").unwrap();
                    return;
                }
            }

            for (idx, &node) in cycle.iter().enumerate() {
                vals[node] = Some(kv[idx]);
            }
        } else {
            let start = scc[0];
            let mut sum = Fraction::zero();

            for &p in factories[start].pred.iter() {
                if scc_id[p] != idx {
                    let val = vals[p].unwrap();
                    sum = sum + val * Fraction::new(factories[p].output, 1);
                }
            }

            if factories[start].input == 0 {
                if !sum.is_zero() {
                    writeln!(out, "stop").unwrap();
                    return;
                }

                vals[start] = Some(Fraction::one());
            } else {
                if sum.is_zero() {
                    writeln!(out, "stop").unwrap();
                    return;
                }

                let kv = sum / Fraction::new(factories[start].input, 1);

                if !kv.positive() {
                    writeln!(out, "stop").unwrap();
                    return;
                }

                vals[start] = Some(kv);
            }
        }

        for &next in dag[idx].iter() {
            in_degree[next] -= 1;

            if in_degree[next] == 0 {
                queue.push_back(next);
            }
        }
    }

    if vals.iter().any(|&val| val.is_none()) {
        writeln!(out, "stop").unwrap();
        return;
    }

    let mut lcm = 1;

    for val in vals.iter() {
        let d = val.unwrap().den;
        let g = gcd(lcm, d);

        lcm = lcm / g * d;
    }

    let mut ret = Vec::with_capacity(n);

    for val in vals {
        let val = val.unwrap();
        ret.push(val.num * (lcm / val.den));
    }

    let mut g = ret[0].abs();

    for val in ret[1..].iter() {
        g = gcd(g, val.abs());
    }

    if g > 1 {
        for val in ret.iter_mut() {
            *val /= g;
        }
    }

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
