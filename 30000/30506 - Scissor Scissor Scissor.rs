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

    let mut k = scan.token::<i64>();
    let mut hand = vec!['2'; 100];

    for i in 0..100 {
        hand[i] = '0';

        println!("? {}", hand.iter().collect::<String>());

        let ret = scan.token::<i64>();

        if ret > k {
            k = ret;
        } else if ret == k {
            hand[i] = '5';
            k += 1;
        } else {
            hand[i] = '2';
        }
    }

    print!("! ");

    for rsp in hand {
        print!(
            "{}",
            match rsp {
                '0' => '2',
                '2' => '5',
                '5' => '0',
                _ => unreachable!(),
            }
        );
    }

    println!();
}
