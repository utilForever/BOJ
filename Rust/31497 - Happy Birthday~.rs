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
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let n = scan.token::<usize>();
    let mut names = vec![String::new(); n];
    let mut is_valid = vec![-1; n];

    for i in 0..n {
        names[i] = scan.token::<String>();
    }

    for i in 0..n {
        println!("? {}", names[i]);
        let val1 = scan.token::<i64>();

        println!("? {}", names[i]);
        let val2 = scan.token::<i64>();

        if val1 == val2 {
            is_valid[i] = val1;
        }
    }

    let is_valid_exist = is_valid.iter().any(|&x| x == 1);
    let pos = if is_valid_exist {
        is_valid.iter().position(|&x| x == 1).unwrap()
    } else {
        is_valid.iter().position(|&x| x == -1).unwrap()
    };
    
    println!("! {}", names[pos]);
}
