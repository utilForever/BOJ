use io::Write;
use std::{collections::HashMap, io, str};

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

const HASH_TABLE: [u64; 26] = [
    0x243F6A8885A308D3,
    0x13198A2E03707344,
    0xA4093822299F31D0,
    0x082EFA98EC4E6C89,
    0x452821E638D01377,
    0xBE5466CF34E90C6C,
    0xC0AC29B7C97C50DD,
    0x3F84D5B5B5470917,
    0x9216D5D98979FB1B,
    0xD1310BA698DFB5AC,
    0x2FFD72DBD01ADFB7,
    0xB8E1AFED6A267E96,
    0xBA7C9045F12C7F99,
    0x24A19947B3916CF7,
    0x0801F2E2858EFC16,
    0x636920D871574E69,
    0xA458FEA3F4933D7E,
    0x0D95748F728EB658,
    0x718BCD5882154AEE,
    0x7B54A41DC25A59B5,
    0x9C30D5392AF26013,
    0xC5D1B023286085F0,
    0xCA417918B8DB38EF,
    0x8E79DCB0603A180E,
    0x6C9E0E8BB01E8A3E,
    0xD71577C1BD314B27,
];

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let s = scan.token::<String>();

    let mut hash_prefix = Vec::with_capacity(n + 1);
    let mut acc = 0;

    hash_prefix.push(0);

    for b in s.bytes() {
        acc = acc + HASH_TABLE[(b - b'a') as usize];
        hash_prefix.push(acc);
    }

    let mut hash_by_len: HashMap<usize, Vec<(u64, usize)>> = HashMap::new();
    let mut ret = vec![0; m];

    for i in 0..m {
        let q = scan.token::<String>();
        let len = q.len();
        let mut acc = 0;

        for b in q.bytes() {
            acc = acc + HASH_TABLE[(b - b'a') as usize];
        }

        hash_by_len.entry(len).or_default().push((acc, i));
    }

    for (len, q) in hash_by_len {
        if len > n {
            for (_, idx) in q {
                ret[idx] = 0;
            }

            continue;
        }

        let mut freq = HashMap::with_capacity(n - len + 1);

        for i in 0..=n - len {
            let acc = hash_prefix[i + len] - hash_prefix[i];
            *freq.entry(acc).or_insert(0) += 1;
        }

        for (acc, idx) in q {
            ret[idx] = *freq.get(&acc).unwrap_or(&0) as i64;
        }
    }

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
