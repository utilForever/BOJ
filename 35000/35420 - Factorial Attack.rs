use io::Write;
use std::{
    collections::{BinaryHeap, HashMap},
    io, str,
};

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

#[derive(Clone, Copy, Eq, PartialEq)]
struct Node {
    f: i32, // f(x) = g(x) + h(x)
    g: i32, // g(x) = cost from start to current node
    state: [i32; 11],
}

impl Node {
    fn new(f: i32, g: i32, state: [i32; 11]) -> Self {
        Self { f, g, state }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f).then_with(|| self.g.cmp(&other.g))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn lower_bound(damages_max: &Vec<i32>, state: &[i32; 11], n: usize) -> i32 {
    let mut health_sum_subset = [0; 1 << 11];
    let mut ret = 0;

    for mask in 1..1 << n {
        let lsb = (mask & (!mask + 1)) as usize;
        let idx = lsb.trailing_zeros() as usize;

        health_sum_subset[mask] = health_sum_subset[mask ^ lsb] + state[idx];
        ret = ret.max((health_sum_subset[mask] + damages_max[mask] - 1) / damages_max[mask]);
    }

    ret
}

fn process_astar(
    attacks: &Vec<[i32; 11]>,
    hits: &Vec<Vec<usize>>,
    damages_max: &Vec<i32>,
    start: [i32; 11],
    n: usize,
) -> i32 {
    let mut priority_queue = BinaryHeap::new();
    let mut best = HashMap::new();
    let h = lower_bound(&damages_max, &start, n);

    priority_queue.push(Node::new(h, 0, start));
    best.insert(start, 0);

    while let Some(node) = priority_queue.pop() {
        let Node { f: _, g, state } = node;

        if best.get(&state) != Some(&g) {
            continue;
        }

        if state[..n].iter().all(|&x| x == 0) {
            return g;
        }

        let mut pivot = 0;

        for i in 1..n {
            if state[i] > state[pivot] {
                pivot = i;
            }
        }

        for &attack_idx in hits[pivot].iter() {
            let mut state_next = state;
            let g_next = g + 1;

            for i in 0..n {
                state_next[i] = (state_next[i] - attacks[attack_idx][i]).max(0);
            }

            if best.get(&state_next).map_or(false, |&x| x <= g_next) {
                continue;
            }

            best.insert(state_next, g_next);

            let h_next = lower_bound(&damages_max, &state_next, n);
            priority_queue.push(Node::new(g_next + h_next, g_next, state_next));
        }
    }

    unreachable!()
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut healths = vec![0; n];

    for i in 0..n {
        healths[i] = scan.token::<i32>();
    }

    if n >= 12 {
        writeln!(
            out,
            "{}",
            if healths.iter().all(|&x| x == 1) {
                1
            } else {
                2
            }
        )
        .unwrap();
        return;
    }

    let mut factorial = vec![1; 12];

    for i in 2..=11 {
        factorial[i] = (factorial[i - 1] * i as i32).min(5000);
    }

    let mut attacks = Vec::new();

    for l in 0..n {
        for r in l..n {
            let len = r - l + 1;
            let damage = factorial[n - len + 1];
            let mut attack = [0; 11];

            for i in l..=r {
                attack[i] = damage;
            }

            attacks.push(attack);
        }
    }

    attacks.sort_unstable_by(|a, b| {
        let sum_a = a[..n].iter().sum::<i32>();
        let sum_b = b[..n].iter().sum::<i32>();

        sum_b.cmp(&sum_a)
    });

    let mut hits = vec![Vec::new(); n];

    for (idx, attack) in attacks.iter().enumerate() {
        for i in 0..n {
            if attack[i] > 0 {
                hits[i].push(idx);
            }
        }
    }

    let mut damages_max = vec![0; 1 << n];

    for mask in 1..1 << n {
        let mut damage_max = 0;

        for &attack in attacks.iter() {
            let mut sum = 0;

            for i in 0..n {
                if (mask >> i) & 1 == 1 {
                    sum += attack[i];
                }
            }

            damage_max = damage_max.max(sum);
        }

        damages_max[mask] = damage_max;
    }

    let mut start = [0; 11];

    for i in 0..n {
        start[i] = healths[i];
    }

    let ret = process_astar(&attacks, &hits, &damages_max, start, n);

    writeln!(out, "{ret}").unwrap();
}
