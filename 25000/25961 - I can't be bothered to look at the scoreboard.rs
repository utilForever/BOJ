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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, s) = (scan.token::<usize>(), scan.token::<usize>());
    let q = scan.token::<usize>();

    // Problem based
    let mut cnt_solved_at_least = vec![0; q + 2];
    let mut cnt_solved_equal = vec![0; q + 2];

    cnt_solved_at_least[0] = n - 1;
    cnt_solved_equal[0] = n - 1;

    // Participant based
    let mut solved_cnt = vec![0; n + 1];
    let mut solved_time = vec![0; n + 1];
    let mut jisu_cnt = 0;
    let mut jisu_time = 0;

    let mut cnt_solved_equal_but_time_early = 0;
    let mut cnt_solved_for_award = 0;

    while cnt_solved_at_least[cnt_solved_for_award] >= s {
        cnt_solved_for_award += 1;
    }

    for t in 1..=q {
        let num = scan.token::<usize>();

        if num == 1 {
            jisu_cnt += 1;
            jisu_time = t;
            cnt_solved_equal_but_time_early = cnt_solved_equal[jisu_cnt];
        } else {
            let prev_cnt = solved_cnt[num];
            let prev_time = solved_time[num];

            if jisu_cnt > 0 && prev_cnt == jisu_cnt && prev_time < jisu_time {
                cnt_solved_equal_but_time_early -= 1;
            }

            cnt_solved_equal[prev_cnt] -= 1;
            cnt_solved_equal[prev_cnt + 1] += 1;
            cnt_solved_at_least[prev_cnt + 1] += 1;

            solved_cnt[num] = prev_cnt + 1;
            solved_time[num] = t;

            if prev_cnt + 1 == cnt_solved_for_award {
                while cnt_solved_for_award <= q && cnt_solved_at_least[cnt_solved_for_award] >= s {
                    cnt_solved_for_award += 1;
                }
            }
        }

        let rank = if jisu_cnt == 0 {
            n
        } else {
            1 + cnt_solved_at_least[jisu_cnt + 1] + cnt_solved_equal_but_time_early
        };
        let need = if rank <= s {
            0
        } else {
            cnt_solved_for_award - jisu_cnt
        };

        writeln!(out, "{rank} {need}").unwrap();
    }
}
