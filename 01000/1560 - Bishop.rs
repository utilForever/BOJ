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

// Reference: https://gist.github.com/Wolfman13/a7075b2d886473d54a5a57679e84f1f6/
fn add(num1: String, num2: String) -> String {
    let mut ret = String::new();
    let mut number1: Vec<char> = num1.chars().collect();
    let mut number2: Vec<char> = num2.chars().collect();

    let mut is_number1_negative = false;
    let mut is_number2_negative = false;

    if number1[0] == '-' {
        is_number1_negative = true;
        number1.remove(0);
    }

    if number2[0] == '-' {
        is_number2_negative = true;
        number2.remove(0);
    }

    let longest_len = number1.len().max(number2.len());

    if is_number1_negative == is_number2_negative {
        let mut overflow = 0;

        for _ in 0..longest_len {
            let first: u8 = match number1.pop() {
                Some(character) => character.to_string().parse().unwrap(),
                None => 0,
            };

            let second: u8 = match number2.pop() {
                Some(character) => character.to_string().parse().unwrap(),
                None => 0,
            };

            let mut sum: String = (first + second + overflow)
                .to_string()
                .chars()
                .rev()
                .collect();

            overflow = if sum.len() > 1 {
                sum.pop().unwrap().to_string().parse().unwrap()
            } else {
                0
            };

            ret.push(*sum.chars().collect::<Vec<char>>().first().unwrap());
        }

        if overflow > 0 {
            ret.push(
                *overflow
                    .to_string()
                    .chars()
                    .collect::<Vec<char>>()
                    .first()
                    .unwrap(),
            );
        }

        if is_number1_negative && is_number2_negative {
            ret.push('-');
        }

        ret = ret.chars().rev().collect::<String>();
    } else {
        let mut underflow = 0;

        let is_smaller = {
            if number1.len() < number2.len() {
                true
            } else if number1.len() > number2.len() {
                false
            } else {
                let mut ret = false;
                for i in 0..number1.len() {
                    if number1[i] < number2[i] {
                        ret = true;
                        break;
                    } else if number1[i] > number2[i] {
                        ret = false;
                        break;
                    }
                }

                ret
            }
        };

        for _ in 0..longest_len {
            let first: i32 = match number1.pop() {
                Some(character) => character.to_string().parse().unwrap(),
                None => 0,
            };

            let second: i32 = match number2.pop() {
                Some(character) => character.to_string().parse().unwrap(),
                None => 0,
            };

            let diff: String = if is_smaller {
                let mut ret = second - first - underflow;
                if ret < 0 {
                    ret += 10;
                    underflow = 1;
                } else {
                    underflow = 0;
                }

                ret.to_string().chars().rev().collect()
            } else {
                let mut ret = first - second - underflow;
                if ret < 0 {
                    ret += 10;
                    underflow = 1;
                } else {
                    underflow = 0;
                }

                ret.to_string().chars().rev().collect()
            };

            ret.push(*diff.chars().collect::<Vec<char>>().first().unwrap());
        }

        if underflow > 0 {
            ret.push(
                *underflow
                    .to_string()
                    .chars()
                    .collect::<Vec<char>>()
                    .first()
                    .unwrap(),
            );
        }

        ret = ret.chars().rev().collect::<String>();
        let ret_clone = ret.clone();

        for ch in ret_clone.chars() {
            if ch == '0' {
                ret.remove(0);
            } else {
                break;
            }
        }

        if ret.is_empty() {
            ret = "0".to_string();
        } else if (!is_smaller && is_number1_negative) || (is_smaller && is_number2_negative) {
            ret.insert(0, '-');
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<String>();

    if n == "1" {
        writeln!(out, "1").unwrap();
    } else {
        writeln!(out, "{}", add(add(n.clone(), n), "-2".to_string())).unwrap();
    }
}
