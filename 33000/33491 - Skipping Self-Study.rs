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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

const INF: i64 = 1_000_000_000;

fn check(
    teachers: &Vec<usize>,
    capacity: &Vec<usize>,
    mid: usize,
    n: usize,
    m: usize,
    mut a: i64,
    b: i64,
    ret: &mut Vec<usize>,
) -> bool {
    // The number of times each teacher appears on the day they are absent -> the number of modifications required
    let mut cnt_modify = vec![0; m + 1];
    // The number of free spaces after removing teachers[i]
    let mut free_space = vec![0; m + 1];

    // dp[i][j][k]
    // - i: number of teachers (1..=m)
    // - j: number of free spaces (0..=n)
    // - k: flag that uses option 2 at least once (0 or 1)
    let mut dp = vec![vec![[INF; 2]; n + 1]; m + 1];

    // For reconstructing the solution
    let mut from_free = vec![vec![[0; 2]; n + 1]; m + 1];
    let mut from_flag = vec![vec![[0; 2]; n + 1]; m + 1];
    let mut chosen_option = vec![vec![[0; 2]; n + 1]; m + 1];

    // The option chosen for each teacher
    let mut decision_option = vec![0; m + 2];
    // A list of teachers who choose option 2
    let mut teachers_chosen_two = vec![0; m + 2];
    // The number of times each teacher appears on the day they are present
    let mut count = vec![0; m + 2];

    dp[0][0][0] = 0;

    // Calculate the number of times each teacher appears on the day they are absent (mid + 1 ~ n)
    for i in (mid + 1)..=n {
        cnt_modify[teachers[i]] += 1;
    }

    // Subtract the number of times each teacher appears on the day they are present (1 ~ mid)
    for i in 1..=mid {
        free_space[teachers[i]] -= 1;
    }

    // Determine the number of free spaces
    for i in 1..=m {
        free_space[i] += capacity[i] as i64;

        if free_space[i] < 0 {
            free_space[i] = 0;
        }
    }

    for i in 1..=m {
        for j in 0..=n {
            dp[i][j][0] = INF;
            dp[i][j][1] = INF;
        }

        for j in 0..=n {
            // Option 1: Only add the modification cost for the teacher
            let cost0 = dp[i - 1][j][0] + cnt_modify[i];

            if cost0 < dp[i][j][0] {
                dp[i][j][0] = cost0;
                from_free[i][j][0] = j;
                from_flag[i][j][0] = 0;
                chosen_option[i][j][0] = 0;
            }

            let cost1 = dp[i - 1][j][1] + cnt_modify[i];

            if cost1 < dp[i][j][1] {
                dp[i][j][1] = cost1;
                from_free[i][j][1] = j;
                from_flag[i][j][1] = 1;
                chosen_option[i][j][1] = 0;
            }

            // Option 2: Use the free space secured on the day of attendance
            if j >= capacity[i] {
                let prev_free = j - capacity[i];
                let prev_cost = dp[i - 1][prev_free][0].min(dp[i - 1][prev_free][1]);
                let new_cost = prev_cost + free_space[i];

                if new_cost < dp[i][j][1] {
                    dp[i][j][1] = new_cost;
                    from_free[i][j][1] = prev_free;
                    from_flag[i][j][1] = if dp[i - 1][prev_free][0] + free_space[i] == new_cost {
                        0
                    } else {
                        1
                    };
                    chosen_option[i][j][1] = 1;
                }
            }
        }
    }

    // If dp[m][0][0] is 0, it means that the condition is satisfied without any modifications.
    if dp[m][0][0] == 0 {
        for i in 1..=n {
            ret[i] = teachers[i];
        }

        return true;
    }

    let limit = mid + b as usize;

    // Among dp[m][i][1], find the state where i is less than or equal to mid + b
    // and the total cost is less than or equal to a + b
    for i in 0..=limit {
        if dp[m][i][1] <= a + b {
            // Backtracking: Go up to the option chosen by each teacher.
            let mut curr_free = i;
            let mut curr_flag = 1;

            for j in (1..=m).rev() {
                let option = chosen_option[j][curr_free][curr_flag];
                let prev_free = from_free[j][curr_free][curr_flag];

                decision_option[j] = option;
                curr_flag = from_flag[j][curr_free][curr_flag];
                curr_free = prev_free;
            }

            // Collect the teachers who choose option 2.
            let mut tail = 0;
            let mut head = 0;

            for j in 1..=m {
                if decision_option[j] == 1 {
                    teachers_chosen_two[tail] = j;
                    tail += 1;
                }
            }

            for j in 1..=n {
                ret[j] = teachers[j];
            }

            decision_option[m + 1] = 0;

            for j in 1..=m {
                count[j] = 0;
            }

            for j in 1..=mid {
                if count[ret[j]] >= capacity[ret[j]] {
                    ret[j] = m + 1;
                } else {
                    count[ret[j]] += 1;
                }
            }

            if tail < teachers_chosen_two.len() {
                teachers_chosen_two[tail] = 0;
            }

            // If there are any remaining 'a' for the days of attendance and the teacher has not used Option 2,
            // replace the teacher with free space from the 'teachers_chosen_two'.
            for j in 1..=mid {
                if a <= 0 {
                    break;
                }

                if decision_option[ret[j]] == 0 {
                    while head < tail && free_space[teachers_chosen_two[head]] == 0 {
                        head += 1;
                    }

                    if head == tail {
                        continue;
                    }

                    free_space[teachers_chosen_two[head]] -= 1;
                    ret[j] = teachers_chosen_two[head];
                    a -= 1;
                }
            }

            for j in 1..=mid {
                if ret[j] == m + 1 {
                    ret[j] = teachers[j];
                }
            }

            // If there are any remaining 'a' for the days of absence (mid + 1 ~ n),
            // handle the case where Option II is not used.
            for j in (mid + 1)..=n {
                if a <= 0 {
                    break;
                }

                if decision_option[ret[j]] == 0 {
                    ret[j] = teachers_chosen_two[0];
                    a -= 1;
                }
            }

            return true;
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, a, b) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut teachers = vec![0; n + 1];
    let mut capacity = vec![0; m + 1];

    for i in 1..=n {
        teachers[i] = scan.token::<usize>();
    }

    for i in 1..=m {
        capacity[i] = scan.token::<usize>();
    }

    let mut left = 0;
    let mut right = n;
    let mut ret = vec![0; n + 1];

    while left + 1 < right {
        let mid = (left + right) / 2;

        if check(&teachers, &capacity, mid, n, m, a, b, &mut ret) {
            right = mid;
        } else {
            left = mid;
        }
    }

    if !check(&teachers, &capacity, left, n, m, a, b, &mut ret) {
        check(&teachers, &capacity, right, n, m, a, b, &mut ret);
    }

    for i in 1..=n {
        write!(out, "{} ", ret[i]).unwrap();
    }

    writeln!(out).unwrap();
}
