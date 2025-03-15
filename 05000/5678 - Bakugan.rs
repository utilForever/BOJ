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
        let r = scan.token::<usize>();

        if r == 0 {
            break;
        }

        let mut mark = vec![0; r];
        let mut leti = vec![0; r];

        for i in 0..r {
            mark[i] = scan.token::<i64>();
        }

        for i in 0..r {
            leti[i] = scan.token::<i64>();
        }

        let mut pos_bonus_mark = 10;
        let mut pos_bonus_leti = 10;

        for i in 0..r {
            if i + 2 >= r {
                break;
            }

            if mark[i] == mark[i + 1] && mark[i + 1] == mark[i + 2] {
                pos_bonus_mark = i;
                break;
            }
        }

        for i in 0..r {
            if i + 2 >= r {
                break;
            }

            if leti[i] == leti[i + 1] && leti[i + 1] == leti[i + 2] {
                pos_bonus_leti = i;
                break;
            }
        }

        let mut ret_mark = mark.iter().sum::<i64>();
        let mut ret_leti = leti.iter().sum::<i64>();

        if pos_bonus_mark < pos_bonus_leti {
            ret_mark += 30;
        } else if pos_bonus_mark > pos_bonus_leti {
            ret_leti += 30;
        }

        writeln!(
            out,
            "{}",
            if ret_mark > ret_leti {
                "M"
            } else if ret_mark < ret_leti {
                "L"
            } else {
                "T"
            }
        )
        .unwrap();
    }
}
