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

fn build_failure(pat: &[u8]) -> Vec<usize> {
    let mut fail = vec![0; pat.len()];
    let mut j = 0;

    for i in 1..pat.len() {
        while j > 0 && pat[i] != pat[j] {
            j = fail[j - 1];
        }

        if pat[i] == pat[j] {
            j += 1;
            fail[i] = j;
        }
    }

    fail
}

fn kmp_count(text: &[u8], pat: &[u8], fail: &[usize]) -> u64 {
    if text.len() < pat.len() {
        return 0;
    }

    let (mut j, mut cnt) = (0, 0);

    for &c in text {
        while j > 0 && c != pat[j] {
            j = fail[j - 1];
        }

        if c == pat[j] {
            j += 1;

            if j == pat.len() {
                cnt += 1;
                j = fail[j - 1];
            }
        }
    }

    cnt
}

fn concate_and_truncate(mut a: Vec<u8>, b: &[u8], limit: usize, is_prefix: bool) -> Vec<u8> {
    if is_prefix {
        if a.len() < limit {
            let need = limit - a.len();
            a.extend_from_slice(&b[..need.min(b.len())]);
        }

        a.truncate(limit);
    } else {
        a.extend_from_slice(b);

        if a.len() > limit {
            a.drain(..a.len() - limit);
        }
    }

    a
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut t = 1;

    loop {
        let line = scan.line().trim().to_string();

        if line.is_empty() {
            break;
        }

        let n = line.parse::<usize>().unwrap();
        let p = scan.token::<String>().bytes().collect::<Vec<_>>();
        let m = p.len();

        if m == 1 {
            let mut cnt: Vec<u64> = vec![0; (n + 1).max(2)];
            cnt[0] = if p[0] == b'0' { 1 } else { 0 };
            cnt[1] = if p[0] == b'1' { 1 } else { 0 };

            for i in 2..=n {
                cnt[i] = cnt[i - 1] + cnt[i - 2];
            }

            writeln!(out, "Case {t}: {}", cnt[n]).unwrap();
            t += 1;
            continue;
        }

        let limit = m - 1;
        let fail = build_failure(&p);
        let mut cnt = vec![0; n + 1];
        let mut prefix = vec![vec![b'0'], vec![b'1']];
        let mut suffix = prefix.clone();

        for i in 2..=n {
            prefix.push(concate_and_truncate(
                prefix[i - 1].clone(),
                &prefix[i - 2],
                limit,
                true,
            ));
            suffix.push(concate_and_truncate(
                suffix[i - 1].clone(),
                &suffix[i - 2],
                limit,
                false,
            ));

            let mut cross = suffix[i - 1].clone();
            cross.extend_from_slice(&prefix[i - 2]);

            cnt[i] = cnt[i - 1] + cnt[i - 2] + kmp_count(&cross, &p, &fail);
        }

        writeln!(out, "Case {t}: {}", cnt[n]).unwrap();
        t += 1;
    }
}
