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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let s = scan.token::<String>();
        let mut ret = -1;

        for i in 0..=9 {
            let mut s_new = s
                .replace('?', &i.to_string())
                .replace("=", "-")
                .replace("--", "+");
            let mut is_first_negative = false;
            let mut is_second_negative = false;

            if s_new.chars().nth(0).unwrap() == '-' {
                is_first_negative = true;
                s_new.remove(0);
            }

            if s_new.contains("*-") {
                is_second_negative = true;
                s_new = s_new.replace("*-", "*");
            } else if s_new.contains("+-") {
                is_second_negative = true;
                s_new = s_new.replace("+-", "+");
            }

            let ops = ['+', '-', '*', '='];
            let values: Vec<&str> = s_new.split(&ops).map(|v| v.trim()).collect();

            if values.iter().any(|v| v.len() > 1 && v.starts_with('0')) {
                continue;
            }

            if s.chars()
                .into_iter()
                .any(|v| v == char::from_digit(i as u32, 10).unwrap())
            {
                continue;
            }

            let values = values
                .iter()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let operands: Vec<_> = s_new.matches(&ops).collect();
            let (&(mut cur), values) = values.split_first().unwrap();

            if is_first_negative {
                cur = -cur;
            }

            for (op, &value) in operands.into_iter().zip(values) {
                let mut value = value;

                if is_second_negative {
                    value = -value;
                    is_second_negative = false;
                }
                match op {
                    "+" => cur = cur + value,
                    "-" => cur = cur - value,
                    "*" => cur = cur * value,
                    "/" => cur = cur / value,
                    _ => unreachable!(),
                }
            }

            if cur == 0 {
                ret = i;
                break;
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
