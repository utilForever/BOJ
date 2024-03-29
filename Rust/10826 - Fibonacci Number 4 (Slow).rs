use io::Write;
use std::{cmp, io, str};

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

fn add_fibonacci(a: &String, b: &String) -> String {
    let mut ret = String::from("0");
    ret = ret.repeat(cmp::max(a.len(), b.len()));

    let mut carry = false;

    for i in 0..ret.len() {
        let mut temp = carry as i64;
        carry = false;

        if i < a.len() {
            temp += (a.as_bytes()[a.len() - i - 1] - 48) as i64;
        }
        if i < b.len() {
            temp += (b.as_bytes()[b.len() - i - 1] - 48) as i64;
        }

        if temp >= 10 {
            carry = true;
            temp -= 10;
        }

        ret.replace_range(
            (ret.len() - i - 1)..(ret.len() - i),
            String::from_utf8(vec![(temp + 48) as u8]).unwrap().as_str(),
        );
    }

    if carry {
        ret.insert(0, '1');
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    if n == 0 {
        writeln!(out, "0").unwrap();
        return;
    }

    if n == 1 || n == 2 {
        writeln!(out, "1").unwrap();
        return;
    }

    let mut a = "0".to_string();
    let mut b = "1".to_string();
    let mut ret = String::new();

    for _ in 0..n - 1 {
        ret = add_fibonacci(&a, &b);
        a = b.clone();
        b = ret.clone();
    }

    writeln!(out, "{ret}").unwrap();
}
