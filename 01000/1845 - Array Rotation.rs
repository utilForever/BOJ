use io::Write;
use std::collections::HashSet;
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

fn is_adjacency(x: i64, y: i64) -> bool {
    y == x + 1
}

fn apply_reversal(arr: &mut Vec<i64>, i: usize, j: usize) {
    let mut left = i;
    let mut right = j;

    while left < right {
        let val_left = arr[left];
        let val_right = arr[right];

        arr[left] = -val_right;
        arr[right] = -val_left;

        left += 1;
        right -= 1;
    }

    if left == right {
        arr[left] = -arr[left];
    }
}

fn gain_of(arr: &Vec<i64>, i: usize, j: usize) -> i64 {
    let old_left = is_adjacency(arr[i - 1], arr[i]) as i64;
    let old_right = is_adjacency(arr[j], arr[j + 1]) as i64;
    let new_left = is_adjacency(arr[i - 1], -arr[j]) as i64;
    let new_right = is_adjacency(-arr[i], arr[j + 1]) as i64;

    (new_left + new_right) - (old_left + old_right)
}

fn sign(x: i64) -> i64 {
    if x >= 0 {
        1
    } else {
        -1
    }
}

fn oriented_pairs(arr: &Vec<i64>) -> Vec<(usize, usize, i64)> {
    let n = arr.len();
    let mut ret = Vec::new();

    for i in 0..n {
        for j in i + 1..n {
            let a = arr[i];
            let b = arr[j];

            if sign(a) != sign(b) {
                let sum = a + b;

                if sum == 1 || sum == -1 {
                    ret.push((i, j, sum));
                }
            }
        }
    }

    ret
}

fn oriented_pair_to_reversal(i: usize, j: usize, s: i64, n: usize) -> Option<(usize, usize)> {
    if s == 1 {
        let a = if i == 0 { 1 } else { i };
        let b = if j == 0 { 0 } else { j - 1 };

        if a >= 1 && b <= n && a <= b {
            Some((a, b))
        } else {
            None
        }
    } else if s == -1 {
        let a = i + 1;
        let b = if j > n { n } else { j };

        if a >= 1 && b <= n && a <= b {
            Some((a, b))
        } else {
            None
        }
    } else {
        None
    }
}

fn find_hurdles(arr: &Vec<i64>) -> Vec<(usize, usize)> {
    let n = arr.len() - 2;
    let mut framed = Vec::<(usize, usize)>::new();

    for i in 1..=n {
        let mut visited = HashSet::new();
        let mut val_min = arr[i].abs();
        let mut val_max = val_min;

        visited.insert(val_min);

        for j in (i + 1)..=n {
            let val = arr[j].abs();
            visited.insert(val);

            val_min = val.min(val_min);
            val_max = val.max(val_max);

            let len = (j - i + 1) as i64;

            if (val_max - val_min + 1) == len && visited.len() as i64 == len {
                let val1 = arr[i].abs();
                let val2 = arr[j].abs();

                if (val1 == val_min && val2 == val_max) || (val1 == val_max && val2 == val_min) {
                    if j > i {
                        framed.push((i, j));
                    }
                }
            }
        }
    }

    let mut non_minimal = vec![false; framed.len()];

    for a in 0..framed.len() {
        for b in 0..framed.len() {
            if a == b {
                continue;
            }

            let (i1, j1) = framed[a];
            let (i2, j2) = framed[b];

            if i1 <= i2 && j2 <= j1 && (i1 < i2 || j2 < j1) {
                non_minimal[a] = true;
            }
        }
    }

    let mut hurdles = Vec::new();

    for (idx, &(i, j)) in framed.iter().enumerate() {
        if !non_minimal[idx] {
            hurdles.push((i, j));
        }
    }

    hurdles.sort_by_key(|&(i, _)| i);
    hurdles
}

fn is_identity(arr: &Vec<i64>) -> bool {
    for (idx, &v) in arr.iter().enumerate() {
        if idx == 0 {
            if v != 0 {
                return false;
            }
        } else if idx == arr.len() - 1 {
            if v != (arr.len() - 1) as i64 {
                return false;
            }
        } else if v != idx as i64 {
            return false;
        }
    }

    true
}

// Reference: https://infossm.github.io/blog/2021/12/19/turnips-to-cabbage/
// Reference: https://www.sciencedirect.com/science/article/pii/S0166218X04003440
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let mut arr = Vec::with_capacity(n + 2);

    arr.push(0);
    arr.extend(nums.iter());
    arr.push((n + 1) as i64);

    let mut ops = Vec::new();

    while !is_identity(&arr) {
        let pairs = oriented_pairs(&arr);
        let mut chosen = None;

        if !pairs.is_empty() {
            let mut best_score = -1;
            let mut best_gain = -10;
            let mut best_ij = (0, 0);

            for &(i, j, sgn) in pairs.iter() {
                if let Some((a, b)) = oriented_pair_to_reversal(i, j, sgn, n) {
                    let mut tmp = arr.clone();

                    apply_reversal(&mut tmp, a, b);

                    let score = oriented_pairs(&tmp).len() as i64;
                    let gain = gain_of(&arr, a, b);

                    if score > best_score
                        || (score == best_score && gain > best_gain)
                        || (score == best_score && gain == best_gain && (a, b) < best_ij)
                    {
                        best_score = score;
                        best_gain = gain;
                        best_ij = (a, b);
                    }
                }
            }

            if best_score >= 0 {
                chosen = Some(best_ij);
            }
        }

        if chosen.is_none() {
            let mut best_score = -1;
            let mut best_gain = -10;
            let mut best_len = usize::MAX;
            let mut best_ij = None;

            for i in 1..=n {
                for j in i..=n {
                    let gain = gain_of(&arr, i, j);
                    let mut tmp = arr.clone();

                    apply_reversal(&mut tmp, i, j);

                    let score = oriented_pairs(&tmp).len() as i64;

                    if gain > best_gain
                        || (gain == best_gain && score > best_score)
                        || (gain == best_gain && score == best_score && (j - i) < best_len)
                        || (gain == best_gain
                            && score == best_score
                            && (j - i) == best_len
                            && Some((i, j)) < best_ij)
                    {
                        best_gain = gain;
                        best_score = score;
                        best_len = j - i;
                        best_ij = Some((i, j));
                    }
                }
            }

            if let Some((bi, bj)) = best_ij {
                if best_score > 0 {
                    chosen = Some((bi, bj));
                } else {
                    let hurdles = find_hurdles(&arr);

                    if hurdles.len() >= 2 {
                        let mut pick = None;
                        let mut best_gap = usize::MAX;

                        for w in hurdles.windows(2) {
                            let (_, j1) = w[0];
                            let (i2, _) = w[1];

                            if j1 < i2 {
                                let gap = i2 - j1;

                                if gap < best_gap {
                                    best_gap = gap;
                                    pick = Some((j1, i2));
                                }
                            } else {
                                // Do nothing
                            }
                        }

                        if let Some((pi, pj)) = pick {
                            chosen = Some((pi.min(pj), pi.max(pj)));
                        }
                    } else if hurdles.len() == 1 {
                        let (hi, hj) = hurdles[0];
                        let candidates = vec![(hi, hj.saturating_sub(1)), (hi + 1, hj), (bi, bj)]
                            .into_iter()
                            .filter(|&(x, y)| x <= y && x >= 1 && y <= n)
                            .collect::<Vec<_>>();

                        let mut best_score = -1;
                        let mut best_ij = (bi, bj);

                        for (x, y) in candidates {
                            let mut tmp = arr.clone();

                            apply_reversal(&mut tmp, x, y);

                            let score = oriented_pairs(&tmp).len() as i64;

                            if score > best_score {
                                best_score = score;
                                best_ij = (x, y);
                            }
                        }

                        chosen = Some(best_ij);
                    } else {
                        chosen = Some((bi, bj));
                    }
                }
            }
        }

        if let Some((i, j)) = chosen {
            apply_reversal(&mut arr, i, j);
            ops.push((i, j));
        }
    }

    writeln!(out, "{}", ops.len()).unwrap();

    for &(i, j) in ops.iter().rev() {
        writeln!(out, "{i} {j}").unwrap();
    }
}
