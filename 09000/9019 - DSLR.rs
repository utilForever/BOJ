use io::Write;
use std::{collections::VecDeque, io, str};

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

struct Register {
    pub num: u32,
    pub operations: u128,
    pub depth: usize,
}

const D: u128 = 0b00;
const S: u128 = 0b01;
const L: u128 = 0b10;
const R: u128 = 0b11;

fn bfs(a: u32, b: u32) -> String {
    let mut queue = VecDeque::new();
    let mut check = [false; 10001];
    queue.push_back(Register {
        num: a,
        operations: 0,
        depth: 0,
    });

    while !queue.is_empty() {
        let register = queue.pop_front().unwrap();

        if register.num == b {
            let mut ans = String::new();

            for i in (0..register.depth).rev() {
                let operation = (register.operations & (0b11 << (i * 2))) >> (i * 2);
                ans += match operation {
                    D => "D",
                    S => "S",
                    L => "L",
                    R => "R",
                    _ => "_",
                };
            }

            return ans;
        }

        let mut new_num = (register.num * 2) % 10000;

        if !check[new_num as usize] {
            check[new_num as usize] = true;
            queue.push_back(Register {
                num: new_num,
                operations: register.operations << 2 | D,
                depth: register.depth + 1,
            })
        }

        new_num = if register.num == 0 {
            9999
        } else {
            register.num - 1
        };

        if !check[new_num as usize] {
            check[new_num as usize] = true;
            queue.push_back(Register {
                num: new_num,
                operations: register.operations << 2 | S,
                depth: register.depth + 1,
            })
        }

        new_num = (register.num % 1000) * 10 + (register.num / 1000);

        if !check[new_num as usize] {
            check[new_num as usize] = true;
            queue.push_back(Register {
                num: new_num,
                operations: register.operations << 2 | L,
                depth: register.depth + 1,
            })
        }

        new_num = (register.num % 10) * 1000 + (register.num / 10);

        if !check[new_num as usize] {
            check[new_num as usize] = true;
            queue.push_back(Register {
                num: new_num,
                operations: register.operations << 2 | R,
                depth: register.depth + 1,
            })
        }
    }

    String::new()
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token();

    for _ in 0..t {
        let (a, b) = (scan.token(), scan.token());
        writeln!(out, "{}", bfs(a, b)).unwrap();
    }
}
