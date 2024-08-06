use io::Write;
use std::{cmp::Ordering, io, str, vec};

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

    let n = scan.token::<usize>();
    let mut strings = vec![String::new(); n];

    for i in 0..n {
        strings[i] = scan.token::<String>();
    }

    let order_alphabet = [
        'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
        'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r',
        'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z',
    ];

    strings.sort_by(|a, b| {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut i = 0;
        let mut j = 0;

        while i < a.len() && j < b.len() {
            let a_char = a[i] as char;
            let b_char = b[j] as char;

            let a_is_digit = a_char.is_digit(10);
            let b_is_digit = b_char.is_digit(10);

            if a_is_digit && !b_is_digit {
                return Ordering::Less;
            } else if !a_is_digit && b_is_digit {
                return Ordering::Greater;
            } else if a_is_digit && b_is_digit {
                let mut a_num = String::new();
                let mut b_num = String::new();
                let mut a_prefix = true;
                let mut b_prefix = true;
                let mut a_zero_count = 0;
                let mut b_zero_count = 0;

                while i < a.len() {
                    if a[i] == b'0' {
                        if a_prefix {
                            a_zero_count += 1;
                        }
                    } else {
                        a_prefix = false;
                    }

                    if !a_prefix {
                        a_num.push(a[i] as char);
                    }

                    if i + 1 < a.len() && !a[i + 1].is_ascii_digit() {
                        break;
                    }

                    i += 1;
                }

                while j < b.len() {
                    if b[j] == b'0' {
                        if b_prefix {
                            b_zero_count += 1;
                        }
                    } else {
                        b_prefix = false;
                    }

                    if !b_prefix {
                        b_num.push(b[j] as char);
                    }

                    if j + 1 < b.len() && !b[j + 1].is_ascii_digit() {
                        break;
                    }

                    j += 1;
                }

                if a_num != b_num {
                    if a_num.len() != b_num.len() {
                        return a_num.len().cmp(&b_num.len());
                    }

                    return a_num.cmp(&b_num);
                } else if a_zero_count != b_zero_count {
                    return a_zero_count.cmp(&b_zero_count);
                }
            } else {
                let a_index = order_alphabet.iter().position(|&x| x == a_char).unwrap();
                let b_index = order_alphabet.iter().position(|&x| x == b_char).unwrap();

                if a_index != b_index {
                    return a_index.cmp(&b_index);
                }
            }

            i += 1;
            j += 1;
        }

        a.len().cmp(&b.len())
    });

    for val in strings {
        writeln!(out, "{val}").unwrap();
    }
}
