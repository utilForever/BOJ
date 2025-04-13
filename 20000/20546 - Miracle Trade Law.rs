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

    let money = scan.token::<i64>();
    let mut stocks = [0; 14];

    for i in 0..14 {
        stocks[i] = scan.token::<i64>();
    }

    let mut money_junhyeon = money;
    let mut money_sungmin = money;
    let mut stock_junhyeon = 0;
    let mut stock_sungmin = 0;

    for i in 0..14 {
        if money_junhyeon >= stocks[i] {
            let buy = money_junhyeon / stocks[i];
            stock_junhyeon += buy;
            money_junhyeon -= buy * stocks[i];
        }
    }

    let ret_junhyeon = money_junhyeon + stock_junhyeon * stocks[13];

    for i in 3..14 {
        if stocks[i] < stocks[i - 1]
            && stocks[i - 1] < stocks[i - 2]
            && stocks[i - 2] < stocks[i - 3]
        {
            let buy = money_sungmin / stocks[i];
            stock_sungmin += buy;
            money_sungmin -= buy * stocks[i];
        } else if stocks[i] > stocks[i - 1]
            && stocks[i - 1] > stocks[i - 2]
            && stocks[i - 2] > stocks[i - 3]
        {
            money_sungmin += stock_sungmin * stocks[i];
            stock_sungmin = 0;
        }
    }

    let ret_sungmin = money_sungmin + stock_sungmin * stocks[13];

    writeln!(
        out,
        "{}",
        if ret_junhyeon > ret_sungmin {
            "BNP"
        } else if ret_junhyeon < ret_sungmin {
            "TIMING"
        } else {
            "SAMESAME"
        }
    )
    .unwrap();
}
