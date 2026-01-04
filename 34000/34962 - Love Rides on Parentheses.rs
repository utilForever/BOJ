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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn ask_query(scan: &mut UnsafeScanner<impl io::BufRead>, s: &str) -> i64 {
    println!("? {s}");

    let x = scan.token::<i64>();

    if x == -1 {
        std::process::exit(0);
    }

    if x == 0 {
        println!("! {s}");
        std::process::exit(0);
    }

    x
}

fn main() {
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let n = scan.token::<usize>();
    let m = 2 * n;

    let mut a = vec![' '; m];

    for i in 0..m {
        a[i] = if i % 2 == 0 { '(' } else { ')' };
    }

    let mut prev = ask_query(&mut scan, &a.iter().collect::<String>());
    let mut c = vec![0; m - 1];

    for i in 0..n {
        a[2 * i] = '{';
        a[2 * i + 1] = '}';

        let curr = ask_query(&mut scan, &a.iter().collect::<String>());

        c[2 * i] = curr - prev;
        prev = curr;
    }

    if n >= 2 {
        let mut b = vec![' '; m];
        b[0] = '(';

        for i in 1..m - 1 {
            b[i] = if i % 2 == 1 { '(' } else { ')' };
        }

        b[m - 1] = ')';

        let mut prev = ask_query(&mut scan, &b.iter().collect::<String>());

        for i in 1..n {
            b[2 * i - 1] = '{';
            b[2 * i] = '}';

            let curr = ask_query(&mut scan, &b.iter().collect::<String>());

            c[2 * i - 1] = curr - prev;
            prev = curr;
        }
    }

    let convert_val = |x: char| -> i64 {
        if x == '(' {
            1
        } else if x == '{' {
            -1
        } else {
            0
        }
    };
    let mut s = vec![' '; m];

    s[0] = if c[0] > 0 { '(' } else { '{' };
    s[1] = match c[0] - convert_val(s[0]) {
        1 => ')',
        -1 => '}',
        _ => {
            if c[1] > 0 {
                '('
            } else {
                '{'
            }
        }
    };

    for i in 1..m - 1 {
        s[i + 1] = match c[i] - convert_val(s[i]) {
            1 => ')',
            -1 => '}',
            _ => {
                if c[i + 1] > 0 {
                    '('
                } else {
                    '{'
                }
            }
        };
    }

    println!("! {}", s.iter().collect::<String>());
}
