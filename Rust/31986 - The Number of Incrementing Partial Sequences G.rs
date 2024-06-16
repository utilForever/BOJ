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

#[allow(dead_code)]
mod fft {
    use std::{
        f64::consts::PI,
        ops::{Add, Mul, Sub},
    };

    #[derive(Clone, Copy, Default)]
    pub struct Complex {
        real: f64,
        imag: f64,
    }

    impl Add for Complex {
        type Output = Complex;

        fn add(self, rhs: Complex) -> Complex {
            let (r1, i1, r2, i2) = (self.real, self.imag, rhs.real, rhs.imag);
            Complex {
                real: r1 + r2,
                imag: i1 + i2,
            }
        }
    }

    impl Sub for Complex {
        type Output = Complex;

        fn sub(self, rhs: Complex) -> Complex {
            let (r1, i1, r2, i2) = (self.real, self.imag, rhs.real, rhs.imag);
            Complex {
                real: r1 - r2,
                imag: i1 - i2,
            }
        }
    }

    impl Mul for Complex {
        type Output = Complex;

        fn mul(self, rhs: Complex) -> Complex {
            let (r1, i1, r2, i2) = (self.real, self.imag, rhs.real, rhs.imag);
            Complex {
                real: r1 * r2 - i1 * i2,
                imag: r1 * i2 + r2 * i1,
            }
        }
    }

    impl Complex {
        fn zero() -> Complex {
            Complex {
                real: 0.0,
                imag: 0.0,
            }
        }

        fn one() -> Complex {
            Complex {
                real: 1.0,
                imag: 0.0,
            }
        }

        fn real(x: f64) -> Complex {
            Complex { real: x, imag: 0.0 }
        }

        fn root(n: f64) -> Complex {
            let angle = 2.0 * PI / n;
            Complex {
                real: angle.cos(),
                imag: angle.sin(),
            }
        }
    }

    pub fn fft(a: &mut [Complex], invert: bool) {
        let len = a.len();
        if len == 1 {
            return;
        }

        let mut a0 = vec![Complex::zero(); len / 2];
        let mut a1 = vec![Complex::zero(); len / 2];

        for i in 0..len / 2 {
            a0[i] = a[i * 2];
            a1[i] = a[i * 2 + 1];
        }

        fft(&mut a0, invert);
        fft(&mut a1, invert);

        let root = Complex::root(if invert { -1.0 } else { 1.0 } * len as f64);
        let mut cur_root = Complex::one();

        for i in 0..len / 2 {
            a[i] = a0[i] + cur_root * a1[i];
            a[i + len / 2] = a0[i] - cur_root * a1[i];

            if invert {
                a[i].real /= 2.0;
                a[i].imag /= 2.0;
                a[i + len / 2].real /= 2.0;
                a[i + len / 2].imag /= 2.0;
            }

            cur_root = cur_root * root;
        }
    }

    pub fn fft_opt(a: &mut [Complex], invert: bool) {
        let n = a.len();
        let mut j = 0;

        for i in 1..n {
            let mut bit = n >> 1;

            while (j & bit) != 0 {
                j ^= bit;
                bit >>= 1;
            }

            j ^= bit;

            if i < j {
                a.swap(i, j);
            }
        }

        let mut len = 2;

        while len <= n {
            let root = Complex::root(if invert { -1.0 } else { 1.0 } * len as f64);

            for i in 0..n / len {
                let i = i * len;
                let mut w = Complex::one();

                for j in 0..len / 2 {
                    let u = a[i + j];
                    let v = a[i + j + len / 2] * w;

                    a[i + j] = u + v;
                    a[i + j + len / 2] = u - v;
                    w = w * root;
                }
            }

            len <<= 1;
        }

        if invert {
            for x in a {
                x.real /= n as f64;
                x.imag /= n as f64;
            }
        }
    }

    pub fn polymul(a: &[f64], b: &[f64]) -> Vec<f64> {
        let lens = (a.len() + b.len()).next_power_of_two();
        let mut fa = vec![Complex::zero(); lens];
        let mut fb = vec![Complex::zero(); lens];

        for i in 0..a.len() {
            fa[i] = Complex::real(a[i]);
        }

        for i in 0..b.len() {
            fb[i] = Complex::real(b[i]);
        }

        fft_opt(&mut fa, false);
        fft_opt(&mut fb, false);

        for i in 0..lens {
            fa[i] = fa[i] * fb[i];
        }

        fft_opt(&mut fa, true);

        fa.iter()
            .take(a.len() + b.len() - 1)
            .map(|x| x.real)
            .collect()
    }

    pub fn bigmul(a: &[u8], b: &[u8]) -> Vec<u8> {
        let mut fa = vec![0.0; a.len()];
        let mut fb = vec![0.0; b.len()];

        for i in 0..a.len() {
            fa[i] = (a[i] - 48) as f64;
        }

        for i in 0..b.len() {
            fb[i] = (b[i] - 48) as f64;
        }

        let mut ret = vec![0];

        for &x in &polymul(&fa, &fb) {
            ret.push(x.round() as u32);
        }

        for i in (1..ret.len()).rev() {
            ret[i - 1] += ret[i] / 10;
            ret[i] %= 10;
        }

        ret.iter().map(|&x| x as u8 + 48).collect()
    }
}

#[allow(dead_code)]
mod ntt {
    mod mod998244353 {
        pub const MOD: usize = 998244353;

        pub const ROOTS: [usize; 24] = [
            1, 998244352, 911660635, 372528824, 929031873, 452798380, 922799308, 781712469,
            476477967, 166035806, 258648936, 584193783, 63912897, 350007156, 666702199, 968855178,
            629671588, 24514907, 996173970, 363395222, 565042129, 733596141, 267099868, 15311432,
        ];

        pub const INV: [usize; 24] = [
            1, 998244352, 86583718, 509520358, 337190230, 87557064, 609441965, 135236158,
            304459705, 685443576, 381598368, 335559352, 129292727, 358024708, 814576206, 708402881,
            283043518, 3707709, 121392023, 704923114, 950391366, 428961804, 382752275, 469870224,
        ];

        pub const INV_P2: [usize; 24] = [
            1, 499122177, 748683265, 873463809, 935854081, 967049217, 982646785, 990445569,
            994344961, 996294657, 997269505, 997756929, 998000641, 998122497, 998183425, 998213889,
            998229121, 998236737, 998240545, 998242449, 998243401, 998243877, 998244115, 998244234,
        ];
    }

    mod mod104857601 {
        pub const MOD: usize = 104857601;

        pub const ROOTS: [usize; 23] = [
            1, 104857600, 104847361, 76756981, 34399420, 93323136, 98667812, 78472926, 73653238,
            33690314, 18773644, 4354736, 43120115, 62844082, 65430330, 80259084, 100680575,
            81980263, 35912312, 18702539, 79427530, 98507391, 39193363,
        ];

        pub const INV: [usize; 23] = [
            1, 104857600, 10240, 83765945, 45929376, 19297248, 21338453, 99625490, 42994480,
            83847972, 23338676, 18512281, 24489994, 82421973, 8903218, 45551298, 89241999,
            59591738, 35844891, 72243308, 8583183, 71338971, 96987805,
        ];

        pub const INV_P2: [usize; 23] = [
            1, 52428801, 78643201, 91750401, 98304001, 101580801, 103219201, 104038401, 104448001,
            104652801, 104755201, 104806401, 104832001, 104844801, 104851201, 104854401, 104856001,
            104856801, 104857201, 104857401, 104857501, 104857551, 104857576,
        ];
    }

    pub use mod998244353::{INV, INV_P2, MOD, ROOTS};

    pub fn fft_opt(a: &mut [usize], invert: bool) {
        let n = a.len();
        let mut j = 0;

        for i in 1..n {
            let mut bit = n >> 1;

            while (j & bit) != 0 {
                j ^= bit;
                bit >>= 1;
            }

            j ^= bit;

            if i < j {
                a.swap(i, j);
            }
        }

        let mut len = 2;
        let mut idx = 1;

        while len <= n {
            let root = if invert { INV } else { ROOTS }[idx];

            for i in 0..n / len {
                let i = i * len;
                let mut w = 1;

                for j in 0..len / 2 {
                    let u = a[i + j];
                    let v = a[i + j + len / 2] * w % MOD;

                    a[i + j] = (u + v) % MOD;
                    a[i + j + len / 2] = (u + MOD - v) % MOD;
                    w = w * root % MOD;
                }
            }

            len <<= 1;
            idx += 1;
        }

        if invert {
            let inv = INV_P2[n.trailing_zeros() as usize];

            for x in a {
                *x = *x * inv % MOD;
            }
        }
    }

    pub fn polymul(a: &[usize], b: &[usize]) -> Vec<usize> {
        let lens = (a.len() + b.len()).next_power_of_two();
        let mut fa = vec![0; lens];
        let mut fb = vec![0; lens];

        fa[..a.len()].copy_from_slice(a);
        fb[..b.len()].copy_from_slice(b);

        fft_opt(&mut fa, false);
        fft_opt(&mut fb, false);

        for i in 0..lens {
            fa[i] *= fb[i];
        }

        fft_opt(&mut fa, true);

        fa.iter().take(a.len() + b.len() - 1).copied().collect()
    }
}

#[allow(dead_code)]
mod ntt2 {
    mod mod998244353 {
        pub const MOD: usize = 998244353;

        pub const ROOTS: [usize; 24] = [
            1, 998244352, 911660635, 372528824, 929031873, 452798380, 922799308, 781712469,
            476477967, 166035806, 258648936, 584193783, 63912897, 350007156, 666702199, 968855178,
            629671588, 24514907, 996173970, 363395222, 565042129, 733596141, 267099868, 15311432,
        ];

        pub const INV: [usize; 24] = [
            1, 998244352, 86583718, 509520358, 337190230, 87557064, 609441965, 135236158,
            304459705, 685443576, 381598368, 335559352, 129292727, 358024708, 814576206, 708402881,
            283043518, 3707709, 121392023, 704923114, 950391366, 428961804, 382752275, 469870224,
        ];

        pub const INV_P2: [usize; 24] = [
            1, 499122177, 748683265, 873463809, 935854081, 967049217, 982646785, 990445569,
            994344961, 996294657, 997269505, 997756929, 998000641, 998122497, 998183425, 998213889,
            998229121, 998236737, 998240545, 998242449, 998243401, 998243877, 998244115, 998244234,
        ];
    }

    mod mod104857601 {
        pub const MOD: usize = 104857601;

        pub const ROOTS: [usize; 23] = [
            1, 104857600, 104847361, 76756981, 34399420, 93323136, 98667812, 78472926, 73653238,
            33690314, 18773644, 4354736, 43120115, 62844082, 65430330, 80259084, 100680575,
            81980263, 35912312, 18702539, 79427530, 98507391, 39193363,
        ];

        pub const INV: [usize; 23] = [
            1, 104857600, 10240, 83765945, 45929376, 19297248, 21338453, 99625490, 42994480,
            83847972, 23338676, 18512281, 24489994, 82421973, 8903218, 45551298, 89241999,
            59591738, 35844891, 72243308, 8583183, 71338971, 96987805,
        ];

        pub const INV_P2: [usize; 23] = [
            1, 52428801, 78643201, 91750401, 98304001, 101580801, 103219201, 104038401, 104448001,
            104652801, 104755201, 104806401, 104832001, 104844801, 104851201, 104854401, 104856001,
            104856801, 104857201, 104857401, 104857501, 104857551, 104857576,
        ];
    }

    mod mod1092616193 {
        pub const MOD: usize = 1092616193;

        pub const ROOTS: [usize; 22] = [
            1, 1092616192, 1028093584, 239571712, 1043144720, 872231331, 88368768, 223607613,
            42054824, 873412670, 695854315, 669752502, 267003629, 669512101, 882027755, 347003797,
            330611819, 80286801, 575983809, 918212341, 1005563392, 633127788,
        ];

        pub const INV: [usize; 22] = [
            1, 1092616192, 64522609, 1032520529, 339061949, 27527525, 93648825, 1033369768,
            712664421, 950593389, 647547039, 179030670, 336332660, 250105162, 64481618, 495417288,
            568756837, 1047590964, 767212362, 102805677, 945086739, 422582537,
        ];

        pub const INV_P2: [usize; 22] = [
            1, 546308097, 819462145, 956039169, 1024327681, 1058471937, 1075544065, 1084080129,
            1088348161, 1090482177, 1091549185, 1092082689, 1092349441, 1092482817, 1092549505,
            1092582849, 1092599521, 1092607857, 1092612025, 1092614109, 1092615151, 1092615672,
        ];
    }

    pub fn fft_opt(
        a: &mut [usize],
        invert: bool,
        m: usize,
        roots: &[usize],
        inv: &[usize],
        inv_p2: &[usize],
    ) {
        let n = a.len();
        let mut j = 0;

        for i in 1..n {
            let mut bit = n >> 1;

            while (j & bit) != 0 {
                j ^= bit;
                bit >>= 1;
            }

            j ^= bit;

            if i < j {
                a.swap(i, j);
            }
        }

        let mut len = 2;
        let mut idx = 1;

        while len <= n {
            let root = if invert { inv } else { roots }[idx];

            for i in 0..n / len {
                let i = i * len;
                let mut w = 1;

                for j in 0..len / 2 {
                    let u = a[i + j];
                    let v = a[i + j + len / 2] * w % m;

                    a[i + j] = (u + v) % m;
                    a[i + j + len / 2] = (u + m - v) % m;
                    w = w * root % m;
                }
            }

            len <<= 1;
            idx += 1;
        }

        if invert {
            let inv = inv_p2[n.trailing_zeros() as usize];

            for x in a {
                *x = *x * inv % m;
            }
        }
    }

    pub fn polymul(a: &[usize], b: &[usize], m: usize) -> Vec<usize> {
        let lens = (a.len() + b.len()).next_power_of_two();
        let mut fa = vec![0; lens];
        let mut fb = vec![0; lens];

        fa[..a.len()].copy_from_slice(a);
        fb[..b.len()].copy_from_slice(b);

        let mut fa2 = fa.clone();
        let mut fb2 = fb.clone();

        fft_opt(
            &mut fa,
            false,
            mod998244353::MOD,
            &mod998244353::ROOTS,
            &mod998244353::INV,
            &mod998244353::INV_P2,
        );
        fft_opt(
            &mut fb,
            false,
            mod998244353::MOD,
            &mod998244353::ROOTS,
            &mod998244353::INV,
            &mod998244353::INV_P2,
        );

        for i in 0..lens {
            fa[i] = fa[i] * fb[i] % mod998244353::MOD;
        }

        fft_opt(
            &mut fa,
            true,
            mod998244353::MOD,
            &mod998244353::ROOTS,
            &mod998244353::INV,
            &mod998244353::INV_P2,
        );

        fft_opt(
            &mut fa2,
            false,
            mod1092616193::MOD,
            &mod1092616193::ROOTS,
            &mod1092616193::INV,
            &mod1092616193::INV_P2,
        );
        fft_opt(
            &mut fb2,
            false,
            mod1092616193::MOD,
            &mod1092616193::ROOTS,
            &mod1092616193::INV,
            &mod1092616193::INV_P2,
        );

        for i in 0..lens {
            fa2[i] = fa2[i] * fb2[i] % mod1092616193::MOD;
        }

        fft_opt(
            &mut fa2,
            true,
            mod1092616193::MOD,
            &mod1092616193::ROOTS,
            &mod1092616193::INV,
            &mod1092616193::INV_P2,
        );

        for i in 0..fa.len() {
            let u = fa[i] as u128;
            let v = fa2[i] as u128;

            fa[i] = ((u * 533230094720090466 + v * 557467849938517664) % 1090697944658608129)
                as usize
                % m;
        }

        fa.truncate(a.len() + b.len() - 1);
        fa
    }
}

#[allow(dead_code)]
mod xorconv {
    const P: usize = 30011;

    pub fn fft_opt(a: &mut [usize], invert: bool) {
        let n = a.len();
        let mut j = 0;

        for i in 1..n {
            let mut bit = n >> 1;

            while (j & bit) != 0 {
                j ^= bit;
                bit >>= 1;
            }

            j ^= bit;

            if i < j {
                a.swap(i, j);
            }
        }

        let mut len = 2;

        while len <= n {
            for i in 0..n / len {
                let i = i * len;

                for j in 0..len / 2 {
                    let u = a[i + j];
                    let v = a[i + j + len / 2];

                    a[i + j] = (u + v) % P;
                    a[i + j + len / 2] = (u + P - v) % P;
                }
            }

            len <<= 1;
        }

        if invert {
            for x in a {
                *x = *x * 11729 % P;
            }
        }
    }

    pub fn xorconv(a: &[usize], b: &[usize]) -> Vec<usize> {
        let mut fa = a.to_vec();
        let mut fb = b.to_vec();

        fft_opt(&mut fa, false);
        fft_opt(&mut fb, false);

        for i in 0..fa.len() {
            fa[i] = fa[i] * fb[i] % P;
        }

        fft_opt(&mut fa, true);

        fa
    }
}

const MOD: usize = 998_244_353;

fn divide_and_conquer(golomb: &Vec<usize>, left: usize, right: usize) -> Vec<usize> {
    if left == right {
        return vec![golomb[left], 1];
    }

    let mid = (left + right) / 2;
    let dnc_left = divide_and_conquer(golomb, left, mid);
    let dnc_right = divide_and_conquer(golomb, mid + 1, right);

    let mut ret = ntt::polymul(&dnc_left, &dnc_right);
    ret.truncate(right - left + 2);

    ret
}

// Reference: https://github.com/Bubbler-4/rust-problem-solving/blob/main/competitive/src/competelib/fft.rs
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut golomb = vec![0; m + 1];

    golomb[1] = 1;

    for i in 2..=m {
        golomb[i] = 1 + golomb[i - golomb[golomb[i - 1]]];
    }

    if n == 1 {
        writeln!(out, "{}", golomb[m]).unwrap();
        return;
    }

    let ret = divide_and_conquer(&golomb, 1, m - 1);

    writeln!(out, "{}", (ret[m - n] * golomb[m]) % MOD).unwrap();
}
