use io::Write;
use std::{collections::BTreeSet, io, str};

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

fn find_neighbors(tree: &BTreeSet<usize>, val: usize) -> (Option<&usize>, Option<&usize>) {
    use std::ops::Bound::*;

    let mut before = tree.range((Unbounded, Included(val)));
    let mut after = tree.range((Included(val), Unbounded));

    (before.next_back(), after.next())
}

fn process_manachers(text: &str) -> Vec<usize> {
    let mut s = String::from('*');

    for c in text.chars() {
        s.push(c);
        s.push('*');
    }

    let s = s.chars().collect::<Vec<_>>();
    let mut ret = vec![0; s.len()];
    let mut r = 0;
    let mut c = 0;

    for i in 0..s.len() {
        ret[i] = if r < i { 0 } else { ret[2 * c - i].min(r - i) };

        while i as i64 - ret[i] as i64 - 1 >= 0
            && i + ret[i] + 1 < s.len()
            && s[i - ret[i] - 1] == s[i + ret[i] + 1]
        {
            ret[i] += 1;
        }

        if r < i + ret[i] {
            r = i + ret[i];
            c = i;
        }
    }

    ret
}

// Reference: http://www.secmem.org/blog/2019/03/10/Manacher/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let t = scan.token::<String>();

    let t1 = t.chars().collect::<Vec<_>>();
    let mut t2 = String::new();

    let mut left = 0;
    let mut right = n - 1;

    for i in 0..n {
        if i % 2 == 0 {
            t2.push(t1[left]);
            left += 1;
        } else {
            t2.push(t1[right]);
            right -= 1;
        }
    }

    let mut pre = BTreeSet::new();
    let ret = process_manachers(&t2);
    let mut ans = 0;

    pre.insert(0);

    for i in (0..n * 2).step_by(2) {
        let (_, next) = find_neighbors(&pre, i - ret[i]);
        let next = *next.unwrap_or(&pre.len());

        if next >= i - ret[i] {
            let mut tmp = next as i64 - (i - ret[i]) as i64;
            tmp = ret[i] as i64 - tmp;

            ans = ans.max(i + tmp as usize);
        }

        if i == ret[i] {
            pre.insert(i + ret[i]);
        }
    }

    writeln!(out, "{}", ans / 4).unwrap();
}
