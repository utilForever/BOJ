use io::Write;
use std::{collections::HashSet, io, str};

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
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut blocked_edges = HashSet::new();
    let mut blocked_from_first = Vec::new();

    for _ in 0..m {
        let (mut a, mut b) = (scan.token::<usize>(), scan.token::<usize>());

        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        blocked_edges.insert((a, b));

        if a == 1 {
            blocked_from_first.push(b);
        }
    }

    let find_relay =
        |blocked_edges: &HashSet<(usize, usize)>, blocked_from_first: &Vec<usize>| -> usize {
            'outer: for a in 2..=n {
                if blocked_from_first.contains(&a) {
                    continue;
                }

                for &b in blocked_from_first.iter() {
                    let key = if a < b { (a, b) } else { (b, a) };

                    if blocked_edges.contains(&key) {
                        continue 'outer;
                    }
                }

                return a;
            }

            unreachable!()
        };

    let cost_base = n * (n + 1) / 2 + n - 2;
    let cost_extra = match blocked_from_first.len() {
        0 => 0,
        1 => {
            let relay = find_relay(&blocked_edges, &blocked_from_first);
            relay - 1
        }
        2 => {
            blocked_from_first.sort_unstable();

            let relay = find_relay(&blocked_edges, &blocked_from_first);
            let cost_single_relay = 2 * (relay - 1);
            let cost_chain = blocked_from_first[0] + relay - 2;

            cost_single_relay.min(cost_chain)
        }
        _ => unreachable!(),
    };

    writeln!(out, "{}", cost_base + cost_extra).unwrap();
}
