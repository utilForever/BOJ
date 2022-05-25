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

fn process_dfs(potions: &mut Vec<char>, orders: &Vec<Vec<usize>>, cur_idx: usize, prev_idx: usize) {
    let cur_potion = if prev_idx == 0 {
        'X'
    } else if potions[prev_idx] == 'X' {
        'Y'
    } else {
        'X'
    };
    potions[cur_idx] = cur_potion;

    for order in orders[cur_idx].iter() {
        if potions[*order] == '?' {
            process_dfs(potions, orders, *order, cur_idx);
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut orders = vec![Vec::new(); n * 2 + 1];

    for i in 1..=n {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

        orders[a].push(b);
        orders[b].push(a);
        orders[i * 2 - 1].push(i * 2);
        orders[i * 2].push(i * 2 - 1);
    }

    let mut potions = vec!['?'; n * 2 + 1];

    for i in 1..=n * 2 {
        if potions[i] == '?' {
            process_dfs(&mut potions, &orders, i, 0);
        }
    }

    for i in 1..=n * 2 {
        write!(out, "{}", potions[i]).unwrap();
    }
    writeln!(out).unwrap();
}
