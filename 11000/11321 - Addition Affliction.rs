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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let expression = scan.token::<String>();

        if expression == "0" {
            break;
        }

        let nums = expression
            .split('+')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let mut buckets = vec![Vec::new(); 10];

        for num in nums {
            buckets[num as usize % 10].push(num);
        }

        let mut ret = Vec::new();

        while buckets[0].len() >= 2 {
            let a = buckets[0].pop().unwrap();
            let b = buckets[0].pop().unwrap();

            ret.push(a.to_string());
            ret.push(b.to_string());
        }

        while buckets[5].len() >= 2 {
            let a = buckets[5].pop().unwrap();
            let b = buckets[5].pop().unwrap();

            ret.push(a.to_string());
            ret.push(b.to_string());
        }

        for num in 1..5 {
            let other = 10 - num;

            while !buckets[num].is_empty() && !buckets[other].is_empty() {
                let a = buckets[num].pop().unwrap();
                let b = buckets[other].pop().unwrap();

                ret.push(a.to_string());
                ret.push(b.to_string());
            }
        }

        for num in buckets {
            for val in num {
                ret.push(val.to_string());
            }
        }

        writeln!(out, "{}", ret.join("+")).unwrap();
    }
}
