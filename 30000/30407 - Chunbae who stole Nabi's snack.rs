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

fn process(
    damages: &Vec<i64>,
    ret: &mut i64,
    n: usize,
    k: i64,
    num_turn: usize,
    health: i64,
    dist: i64,
    skill_used: bool,
) {
    if num_turn >= n {
        if *ret <= health {
            *ret = health;
        }
        return;
    }

    // Case 1
    process(
        damages,
        ret,
        n,
        k,
        num_turn + 1,
        health - ((damages[num_turn] - dist) / 2).max(0),
        dist,
        skill_used,
    );

    // Case 2
    process(
        damages,
        ret,
        n,
        k,
        num_turn + 1,
        health - (damages[num_turn] - dist - k).max(0),
        dist + k,
        skill_used,
    );

    // Case 3
    if !skill_used {
        process(
            damages,
            ret,
            n,
            k,
            num_turn + 2,
            health - (damages[num_turn] - dist).max(0),
            dist + k,
            true,
        );
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let (h, d, k) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut damages = vec![0; n];

    for i in 0..n {
        damages[i] = scan.token::<i64>();
    }

    let mut ret = 0;

    process(&damages, &mut ret, n, k, 0, h, d, false);

    writeln!(out, "{}", if ret <= 0 { -1 } else { ret }).unwrap();
}
