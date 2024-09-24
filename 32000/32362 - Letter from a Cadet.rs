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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

#[derive(Clone)]
struct BitSet(Vec<u64>);

impl BitSet {
    fn new(n: usize) -> Self {
        let v = vec![0; (n + 63) / 64];
        Self(v)
    }

    fn set(&mut self, i: usize, v: bool) {
        if v {
            self.0[i / 64] |= 1 << (i % 64);
        } else {
            self.0[i / 64] &= !(1 << (i % 64));
        }
    }

    #[target_feature(enable = "avx2")]
    unsafe fn is_and_zero(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(&x, &y)| x & y == 0)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut letters_written = vec![Vec::new(); n + 1];

    for i in 1..=n {
        letters_written[i] = scan.token::<String>().chars().collect();
    }

    let k = scan.token::<usize>();
    let mut names = vec![Vec::new(); k + 1];

    for i in 1..=k {
        names[i] = scan.token::<String>().chars().collect();
    }

    let mut bit_prefix = vec![vec![BitSet::new(901); 901]; 901];
    let mut bit_suffix = vec![vec![BitSet::new(901); 901]; 901];
    let mut is_skip = vec![vec![false; 901]; 901];
    let mut ret = vec![false; k + 1];

    for i in 1..=k {
        let mut z_prefix = vec![0; 901];

        for j in 1..=n {
            let mut target = names[i].clone();
            target.push('$');
            target.extend(letters_written[j].iter());

            let z_size = target.len();

            // Prefix Z
            let mut l = if j >= 2 { names[i].len() } else { 0 };
            let mut r = if j >= 2 { names[i].len() } else { 0 };
            let mut h = if j >= 2 { names[i].len() + 1 } else { 1 };
            let mut max = 0;

            while h < z_size {
                if h > r {
                    l = h;
                    r = h;

                    while r < z_size && target[r - l] == target[r] {
                        r += 1;
                    }

                    z_prefix[h] = r - l;
                    max = max.max(z_prefix[h]);
                    r -= 1;
                } else {
                    let g = h - l;

                    if z_prefix[g] < r - h + 1 {
                        z_prefix[h] = z_prefix[g];
                    } else {
                        l = h;

                        while r < z_size && target[r - l] == target[r] {
                            r += 1;
                        }

                        z_prefix[h] = r - l;
                        max = max.max(z_prefix[h]);
                        r -= 1;
                    }
                }

                h += 1;
            }

            for h in z_size - names[i].len()..z_size {
                if z_prefix[h] == z_size - h {
                    bit_prefix[i][j].set(z_size - h, true);
                }
            }

            if max == names[i].len() {
                is_skip[i][j] = true;
            }
        }
    }

    for i in 1..=n {
        let mut z_suffix = vec![0; 901];

        for j in 1..=k {
            if is_skip[j][i] {
                continue;
            }

            let mut target = letters_written[i].clone();
            target.push('$');
            target.extend(names[j].iter());

            let z_size = target.len();

            // Suffix Z
            let mut l = 0;
            let mut r = 0;
            let mut h = 1;

            while h < z_size {
                if h > r {
                    l = h;
                    r = h;

                    while r < z_size && target[r - l] == target[r] {
                        r += 1;
                    }

                    z_suffix[h] = r - l;
                    r -= 1;
                } else {
                    let g = h - l;

                    if z_suffix[g] < r - h + 1 {
                        z_suffix[h] = z_suffix[g];
                    } else {
                        l = h;

                        while r < z_size && target[r - l] == target[r] {
                            r += 1;
                        }

                        z_suffix[h] = r - l;
                        r -= 1;
                    }
                }

                h += 1;
            }

            for h in z_size - names[j].len()..z_size {
                if z_suffix[h] == z_size - h {
                    bit_suffix[i][j].set(names[j].len() - z_size + h, true);
                }
            }
        }
    }

    for i in 1..=k {
        let mut dp = vec![0; 451];
        let mut max = 0;

        for j in 1..=n {
            if is_skip[i][j] {
                continue;
            }

            dp[j] = 1;

            for h in 0..j {
                if is_skip[i][h] {
                    continue;
                }
                
                if unsafe { bit_prefix[i][h].is_and_zero(&bit_suffix[j][i]) } {
                    dp[j] = dp[j].max(dp[h] + 1);
                }
            }

            max = max.max(dp[j]);
        }

        ret[i] = if max < n - m { true } else { false };
    }

    writeln!(out, "{}", ret.iter().filter(|&&x| x).count()).unwrap();

    for &val in ret.iter().skip(1) {
        writeln!(out, "{}", if val { "Yes" } else { "No" }).unwrap();
    }
}
