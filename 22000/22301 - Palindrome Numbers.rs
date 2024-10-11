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

fn make_palindrome(n: &Vec<char>) -> Vec<char> {
    let len = n.len();
    let mut ret = n.clone();

    for i in 0..len / 2 {
        ret[len - 1 - i] = ret[i];
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let mut n = scan.token::<String>().chars().collect::<Vec<_>>();
        let len = n.len();
        let mut ret = make_palindrome(&n);

        if ret < n {
            let mut carry = 1;

            for i in (0..(len + 1) / 2).rev() {
                let mut digit = n[i] as u8 - b'0' + carry;

                carry = digit / 10;
                digit %= 10;

                n[i] = (digit + b'0') as char;

                if carry == 0 {
                    break;
                }
            }

            if carry == 1 {
                n.insert(0, '1');
            }

            ret = make_palindrome(&n);
        }

        writeln!(out, "{}", ret.iter().collect::<String>()).unwrap();
    }
}
