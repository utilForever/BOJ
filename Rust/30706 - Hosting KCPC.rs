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
 
static MOD: i64 = 1_000_000_007;
 
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
 
    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut pos_alps = vec![(0, 0); n];
    let mut pos_alkor = vec![(0, 0); m];
    let mut pos_matkor = vec![(0, 0); k];
 
    for i in 0..n {
        pos_alps[i] = (scan.token::<i64>(), scan.token::<i64>());
    }
 
    for i in 0..m {
        pos_alkor[i] = (scan.token::<i64>(), scan.token::<i64>());
    }
 
    for i in 0..k {
        pos_matkor[i] = (scan.token::<i64>(), scan.token::<i64>());
    }
 
    let sizes = [n as i64, m as i64, k as i64];
    let mut dists = [(0, 0); 3];
    let mut dists_squared = [(0, 0); 3];
 
    for i in 0..n {
        dists[0].0 = (dists[0].0 + pos_alps[i].0) % MOD;
        dists[0].1 = (dists[0].1 + pos_alps[i].1) % MOD;
        dists_squared[0].0 = (dists_squared[0].0 + pos_alps[i].0 * pos_alps[i].0) % MOD;
        dists_squared[0].1 = (dists_squared[0].1 + pos_alps[i].1 * pos_alps[i].1) % MOD;
    }
 
    for i in 0..m {
        dists[1].0 = (dists[1].0 + pos_alkor[i].0) % MOD;
        dists[1].1 = (dists[1].1 + pos_alkor[i].1) % MOD;
        dists_squared[1].0 = (dists_squared[1].0 + pos_alkor[i].0 * pos_alkor[i].0) % MOD;
        dists_squared[1].1 = (dists_squared[1].1 + pos_alkor[i].1 * pos_alkor[i].1) % MOD;
    }
 
    for i in 0..k {
        dists[2].0 = (dists[2].0 + pos_matkor[i].0) % MOD;
        dists[2].1 = (dists[2].1 + pos_matkor[i].1) % MOD;
        dists_squared[2].0 = (dists_squared[2].0 + pos_matkor[i].0 * pos_matkor[i].0) % MOD;
        dists_squared[2].1 = (dists_squared[2].1 + pos_matkor[i].1 * pos_matkor[i].1) % MOD;
    }
 
    let mut ret = 0;
 
    for i in 0..3 {
        let a = i;
        let b = (i + 1) % 3;
        let c = (i + 2) % 3;
 
        ret = (ret + 2 * dists_squared[a].0 * sizes[b] % MOD * sizes[c] % MOD) % MOD;
        ret = (ret + 2 * dists_squared[a].1 * sizes[b] % MOD * sizes[c] % MOD) % MOD;
        ret = (ret - 2 * dists[a].0 * dists[b].0 % MOD * sizes[c] % MOD) % MOD;
        ret = (ret - 2 * dists[a].1 * dists[b].1 % MOD * sizes[c] % MOD) % MOD;
 
        ret = (ret + dists[a].0 * dists[b].1 % MOD * sizes[c] % MOD) % MOD;
        ret = (ret - dists[a].1 * dists[b].0 % MOD * sizes[c] % MOD) % MOD;
    }
 
    writeln!(out, "{}", (ret + MOD) % MOD).unwrap();
}
