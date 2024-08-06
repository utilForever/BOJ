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

#[inline(always)]
fn process_stern_brocot_tree(
    len: usize,
    n: usize,
    p_left: usize,
    q_left: usize,
    p_right: usize,
    q_right: usize,
    m: &mut i64,
    arg_min: &mut (usize, usize, usize),
    info1_left: &Vec<(bool, usize)>,
    info2_left: &mut Vec<(bool, usize)>,
    info1_right: &Vec<(bool, usize)>,
    info2_right: &mut Vec<(bool, usize)>,
) {
    let p = p_left + p_right;
    let mut info1 = vec![(false, 0); p];

    let mut idx = 0;

    while idx < p_right {
        info1[idx] = info1_right[idx];
        idx += 1;
    }

    while idx < p {
        if info1_left[idx - p_right].0 || info2_right[info1_left[idx - p_right].1].0 {
            break;
        }

        info1[idx].1 = info2_right[info1_left[idx - p_right].1].1;
        idx += 1;
    }

    while idx < p {
        if info1_left[idx - p_right].0 {
            break;
        }

        info1[idx].0 = true;
        info1[idx].1 = idx - p_right + info2_right[info1_left[idx - p_right].1].1;
        idx += 1;
    }

    while idx < p {
        info1[idx] = info1_left[idx - p_right];
        idx += 1;
    }

    let q = q_left + q_right;
    let mut info2 = vec![(false, 0); n];
    let mut idx = 0;

    while idx < n {
        if info2_left[idx].0 || info2_right[info2_left[idx].1].0 {
            break;
        }

        info2[idx].1 = info2_right[info2_left[idx].1].1;
        idx += 1;
    }

    while idx < n {
        if info2_left[idx].0 {
            break;
        }

        info2[idx].0 = true;
        info2[idx].1 = p_left + info2_right[info2_left[idx].1].1;
        idx += 1;
    }

    while idx < n {
        info2[idx] = info2_left[idx];
        idx += 1;
    }

    let mut temp = vec![0; n];
    let mut idx = n as i64 - 1;

    while idx >= 0 {
        if !info2[idx as usize].0 {
            break;
        }

        temp[idx as usize] = info2[idx as usize].1;
        idx -= 1;
    }

    while idx >= 0 {
        temp[idx as usize] = p + temp[info2[idx as usize].1];
        idx -= 1;
    }

    let mut idx = 0;

    while idx < p {
        if info1[idx].0 {
            break;
        }

        let m_temp = len as i64 - (idx + temp[info1[idx].1]) as i64;

        if m_temp < *m {
            *arg_min = (p, q, idx);
            *m = m_temp;
        }

        idx += 1;
    }

    while idx < p {
        let m_temp = len as i64 - info1[idx].1 as i64;

        if m_temp < *m {
            *arg_min = (p, q, idx);
            *m = m_temp;
        }

        idx += 1;
    }

    if p_left + p <= n {
        process_stern_brocot_tree(
            len,
            n,
            p_left,
            q_left,
            p,
            q,
            m,
            arg_min,
            &info1_left,
            info2_left,
            &info1,
            &mut info2,
        );
    }

    if p + p_right <= n {
        process_stern_brocot_tree(
            len,
            n,
            p,
            q,
            p_right,
            q_right,
            m,
            arg_min,
            &info1,
            &mut info2,
            &info1_right,
            info2_right,
        );
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; n];
    let mut nums_assorted = vec![Vec::new(); 9];
    let mut cnt_digit = vec![0; 10];

    for i in 0..n {
        let val = scan.token::<usize>();
        nums[i] = val;

        if val > 0 {
            nums_assorted[val - 1].push(true);
        }

        if val < 9 {
            nums_assorted[val].push(false);
        }

        cnt_digit[val] += 1;
    }

    let mut arg_min = (0, 0, 0);
    let mut m = n as i64;

    for i in 0..9 {
        let len = nums_assorted[i].len();

        if len == 0 {
            continue;
        }

        let info1_init = vec![(false, 0)];
        let mut info2_init_left = vec![(false, 0); n];
        let mut info2_init_right = vec![(false, 0); n];

        let mut idx = len as i64 - 1;

        while idx >= 0 && nums_assorted[i][idx as usize] {
            info2_init_left[idx as usize] = (true, 0);
            idx -= 1;
        }

        let mut idx_temp = idx;

        while idx >= 0 {
            if !nums_assorted[i][idx as usize] {
                idx_temp = idx;
            }

            if idx_temp == len as i64 - 1 {
                info2_init_left[idx as usize] = (true, 1);
            } else {
                info2_init_left[idx as usize].1 = idx_temp as usize + 1;
            };

            idx -= 1;
        }

        let mut idx = len as i64 - 1;

        while idx >= 0 && !nums_assorted[i][idx as usize] {
            info2_init_right[idx as usize] = (true, 0);
            idx -= 1;
        }

        let mut idx_temp = idx;

        while idx >= 0 {
            if nums_assorted[i][idx as usize] {
                idx_temp = idx;
            }

            if idx_temp == len as i64 - 1 {
                info2_init_right[idx as usize] = (true, 1);
            } else {
                info2_init_right[idx as usize].1 = idx_temp as usize + 1;
            };

            idx -= 1;
        }

        process_stern_brocot_tree(
            n,
            len,
            1,
            i,
            1,
            i + 1,
            &mut m,
            &mut arg_min,
            &info1_init,
            &mut info2_init_left,
            &info1_init,
            &mut info2_init_right,
        );
    }

    let max_cnt_digit = *cnt_digit.iter().max().unwrap();
    let q = cnt_digit.iter().position(|&x| x == max_cnt_digit).unwrap();

    if (n as i64 - max_cnt_digit as i64) < m {
        m = n as i64 - max_cnt_digit as i64;

        writeln!(out, "{m}").unwrap();

        for i in 0..n {
            if nums[i] != q {
                write!(out, "{} ", i + 1).unwrap();
            }
        }

        writeln!(out).unwrap();
        return;
    }

    writeln!(out, "{m}").unwrap();

    let (p, q, shift) = arg_min;
    let mut val = q * (p - shift) % p;

    val += q;
    let mut target = val / p;
    val %= p;

    for i in 0..n {
        if nums[i] == target {
            val += q;
            target = val / p;
            val %= p;
        } else {
            write!(out, "{} ", i + 1).unwrap();
        }
    }

    writeln!(out).unwrap();
}
