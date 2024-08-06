use std::{io, str};

struct Rng([u64; 4]);

impl Rng {
    fn split_mix(v: u64) -> u64 {
        let mut z = v.wrapping_add(0x9e3779b97f4a7c15);

        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    fn new() -> Self {
        let mut seed = 0;
        unsafe { std::arch::x86_64::_rdrand64_step(&mut seed) };

        let mut prev = seed;

        Self(std::array::from_fn(|_| {
            prev = Self::split_mix(prev);
            prev
        }))
    }

    fn next(&mut self, n: u64) -> u64 {
        let [x, y, z, c] = &mut self.0;
        let t = x.wrapping_shl(58) + *c;

        *c = *x >> 6;
        *x = x.wrapping_add(t);

        if *x < t {
            *c += 1;
        }

        *z = z.wrapping_mul(6906969069).wrapping_add(1234567);
        *y ^= y.wrapping_shl(13);
        *y ^= *y >> 17;
        *y ^= y.wrapping_shl(43);

        let base = x.wrapping_add(*y).wrapping_add(*z);
        ((base as u128 * n as u128) >> 64) as u64
    }
}

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

fn main() {
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let mut arr_a = [0; 10000];
    let mut arr_b = [0; 10000];

    for i in 0..10000 {
        arr_a[i] = i as i64 + 1;
        arr_b[i] = i as i64 + 1;
    }

    let mut rng = Rng::new();

    for _ in 0..10000 {
        let i = rng.next(10000) as usize;
        let j = rng.next(10000) as usize;

        arr_a.swap(i, j);
    }

    for _ in 0..10000 {
        let i = rng.next(10000) as usize;
        let j = rng.next(10000) as usize;

        arr_b.swap(i, j);
    }

    let mut idx_a = 0;
    let mut idx_b = 0;
    let a;
    let b;

    loop {
        println!("? A {}", arr_a[idx_a]);

        let ret = scan.token::<i64>();

        if ret == 1 {
            a = arr_a[idx_a];
            break;
        } else {
            idx_a += 1;
        }
    }

    loop {
        println!("? B {}", arr_b[idx_b]);

        let ret = scan.token::<i64>();

        if ret == 1 {
            b = arr_b[idx_b];
            break;
        } else {
            idx_b += 1;
        }
    }

    println!("! {}", a + b);
}
