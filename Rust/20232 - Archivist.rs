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

    let y = scan.token::<i32>();

    writeln!(
        out,
        "{}",
        match y {
            1995 | 1998 | 1999 | 2001 | 2002 | 2003 | 2004 | 2005 | 2009 | 2010 | 2011 | 2012
            | 2014 | 2015 | 2016 | 2017 | 2019 => "ITMO",
            1996 | 1997 | 2000 | 2007 | 2008 | 2013 | 2018 => "SPbSU",
            2006 => "PetrSU, ITMO",
            _ => unreachable!(),
        }
    )
    .unwrap();
}
