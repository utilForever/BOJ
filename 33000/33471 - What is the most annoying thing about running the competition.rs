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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn process_z(s: &Vec<char>) -> (Vec<usize>, Vec<Vec<usize>>) {
    let n = s.len();
    let mut left = 0;
    let mut right = 0;
    let mut z = vec![0; n];
    let mut vals = vec![Vec::new(); n + 1];

    for i in 1..n {
        if i > right {
            left = i;
            right = i;

            while right < n && s[right - left] == s[right] {
                right += 1;
            }

            z[i] = right - left;
            right -= 1;
        } else {
            let k = i - left;

            if z[k] < right - i + 1 {
                z[i] = z[k];
            } else {
                left = i;

                while right < n && s[right - left] == s[right] {
                    right += 1;
                }

                z[i] = right - left;
                right -= 1;
            }
        }
    }

    for i in 1..n {
        vals[z[i].min(i)].push(i);
    }

    (z, vals)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let (_, vals) = process_z(&s);

    let mut set = BTreeSet::new();
    set.insert(2_000_000_000);

    let mut flag = 0;
    let mut len = 0;

    let mut ret_fatigability = 2_100_000_000;
    let mut ret_cnt_behavior = 2_100_000_000;
    let mut ret_behaviors = Vec::new();

    for i in (1..=s.len()).rev() {
        let mut temp = Vec::new();

        for &val in vals[i].iter() {
            set.insert(val);
        }

        let mut location = i;
        let mut cnt = 0;

        loop {
            let next_val = set.range(location..).next().cloned();

            match next_val {
                Some(val) if val <= s.len() => {
                    location = val + i;
                    cnt += 1;
                    temp.push(location - 1);
                }
                _ => {
                    break;
                }
            }
        }

        if ret_fatigability >= s.len() - cnt * i {
            let ret = if cnt == 0 {
                s.len()
            } else {
                s.len() + 1 - cnt * (i - 1)
            };

            if ret_cnt_behavior > ret || ret_fatigability > s.len() - cnt * i {
                flag = cnt;
                len = i;
                ret_cnt_behavior = ret;
                ret_behaviors = temp.clone();
            }

            ret_fatigability = s.len() - cnt * i;
        }
    }

    ret_behaviors.push(2_000_000_000);

    writeln!(out, "{ret_fatigability} {ret_cnt_behavior}").unwrap();

    let mut j = 0;

    for i in 0..s.len() {
        if i == len && flag != 0 {
            write!(out, "2").unwrap();
        }

        if i <= ret_behaviors[j] - len {
            write!(out, "1").unwrap();
        }

        if i == ret_behaviors[j] {
            write!(out, "2").unwrap();
            j += 1;
        }
    }
}
