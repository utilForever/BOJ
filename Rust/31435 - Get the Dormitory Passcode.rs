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

static MOD: i64 = 998_244_353;

fn multiply(x: i64, y: i64) -> i64 {
    (x as i128 * y as i128 % MOD as i128) as i64
}

fn pow(x: i64, mut y: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % MOD;

    while y != 0 {
        if y & 1 != 0 {
            ret = multiply(ret, piv);
        }

        piv = multiply(piv, piv);
        y >>= 1;
    }

    ret
}

fn main() {
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let n = scan.token::<usize>();
    let mut questions = vec![vec![0; n]; n];
    let mut responses = vec![0; n];

    for i in 0..n {
        for j in 0..n {
            questions[i][j] = (2 * i + 2 * n * j) as i64;
        }

        if i >= 2 {
            questions[i][i] += 1;
        }

        print!("? ");

        for val in questions[i].iter() {
            print!("{val} ");
        }

        println!();

        let response = scan.token::<i64>();
        responses[i] = response;
    }

    let mut ret = vec![0; n];

    for i in 2..n {
        ret[i] = (responses[i] - responses[0] - (responses[1] - responses[0]) * i as i64 % MOD
            + MOD * 2)
            % MOD;
    }

    ret[1] = responses[0];

    for i in 2..n {
        ret[1] = (ret[1] - questions[0][i] * ret[i] % MOD + MOD) % MOD;
    }

    ret[1] = ret[1] * pow(2 * n as i64, MOD - 2) % MOD;
    ret[0] = responses[1];

    for i in 1..n {
        ret[0] = (ret[0] - questions[1][i] * ret[i] % MOD + MOD) % MOD;
    }

    ret[0] = ret[0] * pow(2, MOD - 2) % MOD;

    print!("! ");

    for val in ret.iter() {
        print!("{val} ");
    }

    println!();
}
