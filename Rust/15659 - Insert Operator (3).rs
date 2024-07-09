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

fn calculate(
    nums: &Vec<i64>,
    mut operators: [i64; 4],
    mut operators_used: Vec<usize>,
    idx: usize,
) -> (i64, i64) {
    if idx == nums.len() - 1 {
        let priority = [0, 0, 1, 1];
        let mut stack_nums = Vec::new();
        let mut stack_operators = Vec::new();

        for i in 0..nums.len() {
            stack_nums.push(nums[i]);

            if i < nums.len() - 1 {
                if stack_operators.is_empty() {
                    stack_operators.push(operators_used[i]);
                } else {
                    while !stack_operators.is_empty()
                        && priority[operators_used[i]] <= priority[*stack_operators.last().unwrap()]
                    {
                        let b = stack_nums.pop().unwrap();
                        let a = stack_nums.pop().unwrap();
                        let op = stack_operators.pop().unwrap();

                        match op {
                            0 => stack_nums.push(a + b),
                            1 => stack_nums.push(a - b),
                            2 => stack_nums.push(a * b),
                            3 => stack_nums.push(a / b),
                            _ => {}
                        }
                    }

                    stack_operators.push(operators_used[i]);
                }
            }
        }

        while !stack_operators.is_empty() {
            let b = stack_nums.pop().unwrap();
            let a = stack_nums.pop().unwrap();
            let op = stack_operators.pop().unwrap();

            match op {
                0 => stack_nums.push(a + b),
                1 => stack_nums.push(a - b),
                2 => stack_nums.push(a * b),
                3 => stack_nums.push(a / b),
                _ => {}
            }
        }

        let ret = stack_nums.pop().unwrap();

        return (ret, ret);
    }

    let mut min = i64::MAX;
    let mut max = i64::MIN;

    for i in 0..4 {
        if operators[i] == 0 {
            continue;
        }

        operators[i] -= 1;
        operators_used.push(i);

        let (val_min, val_max) = calculate(nums, operators, operators_used.clone(), idx + 1);
        min = min.min(val_min);
        max = max.max(val_max);

        operators[i] += 1;
        operators_used.pop();
    }

    (min, max)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let mut operators = [0, 0, 0, 0];

    for i in 0..4 {
        operators[i] = scan.token::<i64>();
    }

    let ret = calculate(&nums, operators, Vec::new(), 0);

    writeln!(out, "{}", ret.1).unwrap();
    writeln!(out, "{}", ret.0).unwrap();
}
