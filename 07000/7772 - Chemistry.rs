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

    let n = scan.token::<usize>();
    let ret: [i64; 51] = [
        1,
        1,
        1,
        1,
        2,
        3,
        5,
        9,
        18,
        35,
        75,
        159,
        355,
        802,
        1858,
        4347,
        10359,
        24894,
        60523,
        148284,
        366319,
        910726,
        2278658,
        5731580,
        14490245,
        36797588,
        93839412,
        240215803,
        617105614,
        1590507121,
        4111846763,
        10660307791,
        27711253769,
        72214088660,
        188626236139,
        493782952902,
        1295297588128,
        3404490780161,
        8964747474595,
        23647478933969,
        62481801147341,
        165351455535782,
        438242894769226,
        1163169707886427,
        3091461011836856,
        8227162372221203,
        21921834086683418,
        58481806621987010,
        156192366474590639,
        417612400765382272,
        1117743651746953270,
    ];

    writeln!(out, "{}", ret[n]).unwrap();
}
