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

fn pow(x: i64, mut p: i64, modulo: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x;

    while p != 0 {
        if p & 1 != 0 {
            ret = ret * piv % modulo;
        }

        piv = piv * piv % modulo;
        p >>= 1;
    }

    ret
}

fn berlekamp_massey(vals: Vec<i64>, modulo: i64) -> Vec<i64> {
    let mut ls = Vec::new();
    let mut cur = Vec::new();

    let mut lf = 0;
    let mut ld = 0;

    for i in 0..vals.len() {
        let mut t = 0;

        for j in 0..cur.len() {
            t = (t + vals[i - j - 1] * cur[j]) % modulo;
        }

        if (t - vals[i]) % modulo == 0 {
            continue;
        }

        if cur.is_empty() {
            cur.resize(i + 1, 0);
            lf = i;
            ld = (t - vals[i]) % modulo;

            continue;
        }

        let k = -(vals[i] - t) * pow(ld, modulo - 2, modulo) % modulo;

        let mut c = vec![0; i - lf - 1];
        c.push(k);

        for j in ls.iter() {
            c.push(-j * k % modulo);
        }

        if c.len() < cur.len() {
            c.resize(cur.len(), 0);
        }

        for j in 0..cur.len() {
            c[j] = (c[j] + cur[j]) % modulo;
        }

        if i - lf + ls.len() >= cur.len() {
            (ls, lf, ld) = (cur, i, (t - vals[i]) % modulo);
        }

        cur = c;
    }

    for i in cur.iter_mut() {
        *i = (*i % modulo + modulo) % modulo;
    }

    cur
}

fn get_nth(rec: Vec<i64>, dp: Vec<i64>, mut n: usize, modulo: i64) -> i64 {
    let m = rec.len();
    let mut s = vec![0; m];
    let mut t = vec![0; m];

    s[0] = 1;
    if m != 1 {
        t[1] = 1;
    } else {
        t[0] = rec[0];
    }

    let mul = |v: Vec<i64>, w: Vec<i64>| -> Vec<i64> {
        let m = v.len();
        let mut t = vec![0; 2 * m];

        for j in 0..m {
            for k in 0..m {
                t[j + k] += v[j] * w[k] % modulo;

                if t[j + k] >= modulo {
                    t[j + k] -= modulo;
                }
            }
        }

        for j in (m..=2 * m - 1).rev() {
            for k in 1..=m {
                t[j - k] += t[j] * rec[k - 1] % modulo;

                if t[j - k] >= modulo {
                    t[j - k] -= modulo;
                }
            }
        }

        t.resize(m, 0);

        t
    };

    while n != 0 {
        if n & 1 != 0 {
            s = mul(s, t.clone());
        }

        t = mul(t.clone(), t.clone());
        n >>= 1;
    }

    let mut ret = 0;

    for i in 0..m {
        ret += s[i] * dp[i] % modulo;
    }

    ret % modulo
}

fn guess_nth_term(vals: Vec<i64>, n: usize, modulo: i64) -> i64 {
    if n < vals.len() {
        return vals[n as usize];
    }

    let ret = berlekamp_massey(vals.clone(), modulo);

    if ret.is_empty() {
        0
    } else {
        get_nth(ret, vals, n, modulo)
    }
}

// Reference: https://koosaga.com/231
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let precomputed = vec![
        vec![
            2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
            131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432,
            67108864, 134217728, 268435456, 536870912, 73741815, 147483630, 294967260, 589934520,
            179869031, 359738062, 719476124, 438952239, 877904478, 755808947, 511617885, 23235761,
            46471522, 92943044, 185886088, 371772176, 743544352, 487088695, 974177390, 948354771,
            896709533,
        ],
        vec![
            4, 16, 36, 81, 225, 625, 1600, 4096, 10816, 28561, 74529, 194481, 509796, 1336336,
            3496900, 9150625, 23961025, 62742241, 164249856, 429981696, 125736695, 947295503,
            716041218, 200652461, 886200432, 458408758, 488281642, 5232020, 529362765, 586008745,
            223562643, 76425889, 19069136, 2388928, 953136135, 800450530, 539745896, 966886539,
            121283871, 9235872, 533782787, 607200726, 645372102, 671380047, 668750842, 292390808,
            950920440, 345351007, 557653218, 15148755,
        ],
        vec![
            8, 36, 94, 278, 1062, 3650, 11856, 39444, 135704, 456980, 1534668, 5166204, 17480600,
            58888528, 198548648, 669291696, 258436230, 613387281, 676312919, 575341762, 991128221,
            557546496, 284542480, 209398972, 232230803, 303596263, 939962513, 351290213, 415931359,
            328520111, 887554940, 303667674, 351233655, 747600119, 130781946, 702928593, 155509746,
            538853820, 548779965, 726903524, 370846848, 989333901, 795920339, 432839282, 815115627,
            902444432, 3195020, 783730971, 232305131, 894592622,
        ],
        vec![
            16, 81, 278, 1365, 7164, 33858, 161307, 791722, 3859473, 18702843, 90938441, 442661923,
            152542080, 466805482, 911253057, 627500238, 355979736, 651184968, 444168477, 637675570,
            340713937, 193363675, 666524059, 932645942, 897647645, 834763352, 662912921, 725854997,
            840822360, 61565774, 135123018, 995036230, 730107533, 462094335, 710509782, 525321589,
            949550086, 8069878, 739604600, 955573146, 817055186, 27292242, 254984760, 388463753,
            467535957, 483265312, 352974171, 592298092, 268922749, 66458109,
        ],
    ];

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (m, n) = (scan.token::<usize>(), scan.token::<usize>());

        let mut arr = Vec::new();

        for i in 0..precomputed[0].len() {
            arr.push(precomputed[m - 1][i]);
        }

        writeln!(out, "{}", guess_nth_term(arr, n - 1, 1_000_000_009)).unwrap();
    }
}
