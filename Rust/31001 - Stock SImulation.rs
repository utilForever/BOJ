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
                let slice: &str = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

#[derive(Default, Clone)]
struct Company {
    group_num: i64,
    name: String,
    price: i64,
}

impl Company {
    fn new(group_num: i64, name: String, price: i64) -> Self {
        Self {
            group_num,
            name,
            price,
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, q) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut companies = vec![Company::default(); n];

    for i in 0..n {
        let (g, h, p) = (
            scan.token::<i64>(),
            scan.token::<String>(),
            scan.token::<i64>(),
        );
        companies[i] = Company::new(g, h, p);
    }

    let mut money = m;
    let mut stocks = vec![0; n];

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 1 {
            let (a, b) = (scan.token::<String>(), scan.token::<i64>());
            let pos = companies.iter().position(|x| x.name == a).unwrap();

            if money >= b * companies[pos].price {
                money -= b * companies[pos].price;
                stocks[pos] += b;
            }
        } else if command == 2 {
            let (a, b) = (scan.token::<String>(), scan.token::<i64>());
            let pos = companies.iter().position(|x| x.name == a).unwrap();
            let num = stocks[pos].min(b);

            money += num * companies[pos].price;
            stocks[pos] -= num;
        } else if command == 3 {
            let (a, c) = (scan.token::<String>(), scan.token::<i64>());
            let pos = companies.iter().position(|x| x.name == a).unwrap();

            companies[pos].price += c;
        } else if command == 4 {
            let (d, c) = (scan.token::<i64>(), scan.token::<i64>());

            companies
                .iter_mut()
                .filter(|x| x.group_num == d)
                .for_each(|x| x.price += c);
        } else if command == 5 {
            let (d, e) = (scan.token::<i64>(), scan.token::<i64>());

            companies
                .iter_mut()
                .filter(|x| x.group_num == d)
                .for_each(|x| x.price = (x.price * (100 + e) / 1000) * 10);
        } else if command == 6 {
            writeln!(out, "{money}").unwrap();
        } else {
            let ret = companies
                .iter()
                .enumerate()
                .fold(0, |acc, (i, x)| acc + x.price * stocks[i]);

            writeln!(out, "{}", ret + money).unwrap();
        }
    }
}
