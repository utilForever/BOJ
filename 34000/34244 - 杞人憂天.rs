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

const SEED: u64 = 0x1234_5678_9ABC_DEF0;

pub fn build_hash_table(n: usize) -> Vec<u64> {
    let splitmix64 = |mut seed: u64| {
        seed = seed.wrapping_add(0x9e3779b97f4a7c15_u64);
        seed = (seed ^ (seed >> 30)).wrapping_mul(0xbf58476d1ce4e5b9_u64);
        seed = (seed ^ (seed >> 27)).wrapping_mul(0x94d049bb133111eb_u64);
        seed = seed ^ (seed >> 31);
        seed
    };
    let mut table = vec![0; n + 1];

    for i in 1..=n {
        let val = splitmix64(SEED ^ i as u64) & ((1u64 << 61) - 1);
        table[i] = if val == 0 { 1 } else { val };
    }

    table
}

fn main() {
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let t = scan.token::<i64>();

    if t == 1 {
        let (n, x) = (scan.token::<usize>(), scan.token::<u64>());
        let hash_table = build_hash_table(2 * n);

        let mut a = vec![0; n];
        let mut b = vec![0; n];

        for i in 0..n {
            a[i] = scan.token::<usize>();
            b[i] = scan.token::<usize>();
        }

        let mut base = 0;

        for i in 0..n {
            base ^= hash_table[a[i]];
        }

        let mut delta = vec![0; n];
        let target = x ^ base;

        for i in 0..n {
            delta[i] = hash_table[a[i]] ^ hash_table[b[i]];
        }

        let mut basis = vec![0; 61];
        let mut combo = vec![0; 61];

        for i in 0..n {
            let mut val = delta[i];
            let mut mask = 1u128 << i;
            let mut bit = 60 as i64;

            while val != 0 && bit >= 0 {
                if (val >> bit) & 1 == 1 {
                    if basis[bit as usize] == 0 {
                        basis[bit as usize] = val;
                        combo[bit as usize] = mask;
                        break;
                    } else {
                        val ^= basis[bit as usize];
                        mask ^= combo[bit as usize];
                    }
                }

                bit -= 1;
            }
        }

        let mut d = target;
        let mut mask = 0;
        let mut check = true;

        for i in (0..61).rev() {
            if (d >> i) & 1 == 1 {
                if basis[i] == 0 {
                    check = false;
                    break;
                } else {
                    d ^= basis[i];
                    mask ^= combo[i];
                }
            }
        }

        if check {
            for i in 0..n {
                print!("{} ", if (mask >> i) & 1 == 1 { b[i] } else { a[i] });
            }
        } else {
            for i in 0..n {
                print!("{} ", a[i]);
            }
        }

        println!();
    } else {
        let n = scan.token::<usize>();
        let hash_table = build_hash_table(2 * n);

        let mut ret = 0;

        for _ in 0..n {
            let idx = scan.token::<usize>();
            ret ^= hash_table[idx];
        }

        println!("{ret}");
    }
}
