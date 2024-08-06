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

fn calculate_fail(pat: &str) -> Vec<usize> {
    let pat_bytes = pat.as_bytes();
    let mut fail = vec![0; 101];
    let mut cmp = 0;

    for i in 1..pat.len() {
        while cmp > 0 && pat_bytes[cmp] != pat_bytes[i] {
            cmp = fail[cmp - 1];
        }

        if pat_bytes[cmp] == pat_bytes[i] {
            cmp += 1;
            fail[i] = cmp;
        }
    }

    fail
}

fn next_match(dp: &mut Vec<Vec<i32>>, fail: &Vec<usize>, num: usize, pat: &str, ch: u8) -> usize {
    if dp[num][ch as usize] != -1 {
        return dp[num][ch as usize] as usize;
    }

    let pat_bytes = pat.as_bytes();
    let mut ret = num;

    while ret > 0 && ch != pat_bytes[ret] {
        ret = fail[ret - 1];
    }

    if ch == pat_bytes[ret] {
        ret += 1;
    }

    dp[num][ch as usize] = ret as i32;

    ret
}

static MOD: i32 = 998_244_353;

fn calculate_count_all(str: &str) -> i32 {
    let mut cnt = 0;

    for c in str.chars() {
        cnt = (cnt * 10 + (c as i64 - '0' as i64)) % MOD as i64;
    }

    cnt as i32
}

fn calculate_count_pattern(
    fail: &Vec<usize>,
    str: &str,
    pat: &str,
) -> i32 {
    let str_bytes = str.as_bytes();

    let mut dp = vec![vec![-1; 101]; 100001];
    let mut dp_num = vec![vec![-1; 101]; 101];
    let mut idx = 0;
    let mut ret = 0;

    for i in 0..str.len() {
        for ch in ('0' as u8)..str_bytes[i] {
            let next = next_match(&mut dp_num, fail, idx, pat, ch);
            ret = (ret + calculate_count_fail(&mut dp, &mut dp_num, fail, str, pat, i + 1, next)) % MOD;
        }

        idx = next_match(&mut dp_num, fail, idx, pat, str_bytes[i]);

        if idx == pat.len() {
            break;
        }
    }

    if str.find(pat).is_none() {
        ret = (ret + 1) % MOD;
    }

    ret
}

fn calculate_count_fail(
    dp: &mut Vec<Vec<i32>>,
    dp_num: &mut Vec<Vec<i32>>,
    fail: &Vec<usize>,
    str: &str,
    pat: &str,
    idx_str: usize,
    idx_pat: usize,
) -> i32 {
    if idx_str == str.len() {
        return 1;
    }

    if idx_pat == pat.len() {
        return 0;
    }

    if dp[idx_str][idx_pat] != -1 {
        return dp[idx_str][idx_pat];
    }

    let mut ret = 0;

    for ch in ('0' as u8)..=('9' as u8) {
        let match_new = next_match(dp_num, fail, idx_pat, pat, ch);

        if match_new < pat.len() {
            ret = (ret + calculate_count_fail(dp, dp_num, fail, str, pat, idx_str + 1, match_new))
                % MOD;
        }
    }

    dp[idx_str][idx_pat] = ret;

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (l, r) = (scan.token::<String>(), scan.token::<String>());
    let x = scan.token::<String>();
    let fail = calculate_fail(&x);

    let mut start = calculate_count_all(&l);
    start = (start - calculate_count_pattern(&fail, &l, &x) + MOD) % MOD;

    let mut end = calculate_count_all(&r);
    end = (end - calculate_count_pattern(&fail, &r, &x) + MOD) % MOD;

    let mut ret = (end - start + MOD) % MOD;

    if l.find(&x).is_some() {
        ret = (ret + 1) % MOD;
    }

    writeln!(out, "{ret}").unwrap();
}
