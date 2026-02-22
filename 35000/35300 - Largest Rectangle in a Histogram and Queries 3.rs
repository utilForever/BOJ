use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hasher},
    iter::repeat_with,
};
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

#[derive(Clone, Debug)]
pub struct Xorshift {
    y: u64,
}

impl Xorshift {
    pub fn new_with_seed(seed: u64) -> Self {
        Xorshift { y: seed }
    }

    pub fn new() -> Self {
        Xorshift::new_with_seed(RandomState::new().build_hasher().finish())
    }

    pub fn rand64(&mut self) -> u64 {
        self.y ^= self.y << 5;
        self.y ^= self.y >> 17;
        self.y ^= self.y << 11;
        self.y
    }

    pub fn rand(&mut self, k: u64) -> u64 {
        self.rand64() % k
    }

    pub fn rands(&mut self, k: u64, n: usize) -> Vec<u64> {
        repeat_with(|| self.rand(k)).take(n).collect()
    }

    pub fn randf(&mut self) -> f64 {
        const UPPER_MASK: u64 = 0x3FF0_0000_0000_0000;
        const LOWER_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;
        let x = self.rand64();
        let tmp = UPPER_MASK | (x & LOWER_MASK);
        let result: f64 = f64::from_bits(tmp);
        f64::from_bits(f64::to_bits(result - 1.0) ^ (x >> 63))
    }

    pub fn gen_bool(&mut self, p: f64) -> bool {
        self.randf() < p
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        let mut n = slice.len();
        while n > 1 {
            let i = self.rand(n as _) as usize;
            n -= 1;
            slice.swap(i, n);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let n = scan.token::<usize>();
    let mut rng = Xorshift::new();
    let mut cnt = n;
    let mut ret = vec![true; n + 1];

    for _ in 0..10000 {
        let mut p = (1..=n).collect::<Vec<_>>();
        rng.shuffle(&mut p);

        print!("?");

        for val in p.iter() {
            print!(" {val}");
        }

        println!();

        let (l, r, h) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        let mut segment = vec![false; n + 1];

        for &idx in p[l - 1..r].iter() {
            segment[idx] = true;
        }

        if h == n / 2 + 1 {
            for i in 1..=n {
                if ret[i] && !segment[i] {
                    ret[i] = false;
                    cnt -= 1;
                }
            }
        } else if h > n / 2 + 1 {
            for i in 1..=n {
                if ret[i] && segment[i] {
                    ret[i] = false;
                    cnt -= 1;
                }
            }
        }

        if cnt == 1 {
            print!("!");

            for i in 1..=n {
                if ret[i] {
                    print!(" {i}");
                    break;
                }
            }

            println!();
            return;
        }
    }
}
