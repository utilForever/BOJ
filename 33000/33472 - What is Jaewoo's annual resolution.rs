use io::Write;
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

fn divide(s: &str, x: i64) -> (String, i64) {
    let mut quotient = String::new();
    let mut remainder = 0;

    for c in s.chars() {
        remainder = remainder * 10 + (c as u8 - b'0') as i64;
        let d = remainder / x;

        if d != 0 || !quotient.is_empty() {
            quotient.push((d as u8 + b'0') as char);
        }

        remainder %= x;
    }

    (quotient, remainder)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();
    let mut rng = Rng::new();

    for _ in 0..t {
        let mut nums = [0; 10];

        for i in 0..10 {
            nums[i] = scan.token::<i64>();
        }

        if nums[0] > 0 || nums[2] > 0 || nums[4] > 0 || nums[5] > 0 || nums[6] > 0 || nums[8] > 0 {
            let order = [1, 3, 7, 9, 2, 4, 6, 8, 5, 0];
            let mut s = String::new();

            for &idx in order.iter() {
                for _ in 0..nums[idx] {
                    s.push(std::char::from_digit(idx as u32, 10).unwrap());
                }
            }

            if nums[5] > 0 {
                let (q, _) = divide(&s, 5);
                writeln!(out, "{} = {} X 5", s, q).unwrap();
            } else {
                let (q, _) = divide(&s, 2);
                writeln!(out, "{} = {} X 2", s, q).unwrap();
            }

            continue;
        }

        let mut s = String::new();

        for _ in 0..nums[1] {
            s.push('1');
        }
        for _ in 0..nums[3] {
            s.push('3');
        }
        for _ in 0..nums[7] {
            s.push('7');
        }
        for _ in 0..nums[9] {
            s.push('9');
        }

        let mut swap_to = 2;
        let mut cnts = vec![(nums[1], 1), (nums[3], 3), (nums[7], 7), (nums[9], 9)];

        cnts.sort();

        if cnts[1].0 != 0 {
            swap_to = 1;
        }

        if cnts[0].0 != 0 {
            swap_to = 0;
        }

        nums[cnts[swap_to].1 as usize] -= 1;
        nums[cnts[3].1 as usize] += 1;

        let mut t = String::new();

        for _ in 0..nums[1] {
            t.push('1');
        }
        for _ in 0..nums[3] {
            t.push('3');
        }
        for _ in 0..nums[7] {
            t.push('7');
        }
        for _ in 0..nums[9] {
            t.push('9');
        }

        'outer: loop {
            let mut chars_s = s.chars().collect::<Vec<_>>();

            for i in 0..chars_s.len() {
                let k = rng.next(i as u64 + 1) as usize;
                chars_s.swap(i, k);
            }

            let s_shuffled = chars_s.iter().collect::<String>();
            let ret = divide(&s_shuffled, 3);

            if ret.0 != "1" && ret.1 == 0 {
                writeln!(out, "{} = {} X 3", s_shuffled, ret.0).unwrap();
                break;
            }

            let ret = divide(&s_shuffled, 7);

            if ret.0 != "1" && ret.1 == 0 {
                writeln!(out, "{} = {} X 7", s_shuffled, ret.0).unwrap();
                break;
            }

            if s_shuffled.len() < 7 {
                for &num in [11, 13, 17, 19].iter() {
                    let ret = divide(&s_shuffled, num);

                    if ret.0 != "1" && ret.1 == 0 {
                        writeln!(out, "{} = {} X {}", s_shuffled, ret.0, num).unwrap();
                        break 'outer;
                    }
                }
            }

            let mut chars_t = t.chars().collect::<Vec<_>>();

            for i in 0..chars_t.len() {
                let k = rng.next(i as u64 + 1) as usize;
                chars_t.swap(i, k);
            }

            let t_shuffled: String = chars_t.iter().collect();
            let ret = divide(&t_shuffled, 3);

            if ret.0 != "1" && ret.1 == 0 {
                writeln!(out, "{} = {} X 3", t_shuffled, ret.0).unwrap();
                break;
            }

            let ret = divide(&t_shuffled, 7);

            if ret.0 != "1" && ret.1 == 0 {
                writeln!(out, "{} = {} X 7", t_shuffled, ret.0).unwrap();
                break;
            }

            if t_shuffled.len() < 7 {
                for &num in [11, 13, 17, 19].iter() {
                    let ret = divide(&t_shuffled, num);

                    if ret.0 != "1" && ret.1 == 0 {
                        writeln!(out, "{} = {} X {}", t_shuffled, ret.0, num).unwrap();
                        break 'outer;
                    }
                }
            }
        }
    }
}
