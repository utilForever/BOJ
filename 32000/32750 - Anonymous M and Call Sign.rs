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

    let signs_head = vec!["se", "mik-jjang", "Are", "u-", "hai"];
    let signs = vec![
        vec![
            "no", "hai-", "hai-", "hai", "hai", "hai", "hai", "u-", "hai",
        ],
        vec!["kawaii!"],
        vec!["you", "ready", "antena", "senku", "high!"],
        vec!["hai"],
        vec![],
    ];

    let s = scan.line().trim().to_string();
    let words = s
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    let mut idx = 0;
    let mut idx_sign = -1;
    let mut idx_sign_internal = 0;
    let mut cnt_lead_sign = 0;
    let mut cnt_sign = 0;

    while idx < words.len() {
        if idx_sign == -1 && signs_head.contains(&words[idx].as_str()) {
            cnt_sign += 1;

            idx_sign = match words[idx].as_str() {
                "se" => 0,
                "mik-jjang" => 1,
                "Are" => 2,
                "u-" => 3,
                "hai" => -1,
                _ => unreachable!(),
            };

            if idx_sign >= 0 && idx_sign <= 2 {
                cnt_lead_sign += 1;
            }

            idx += 1;
            continue;
        }

        if idx_sign != -1 && words[idx] == signs[idx_sign as usize][idx_sign_internal] {
            idx += 1;
            idx_sign_internal += 1;

            if idx_sign_internal == signs[idx_sign as usize].len() {
                if idx_sign == 0 {
                    while idx < words.len() {
                        if words[idx] == "u-" {
                            idx_sign_internal -= 2;
                            break;
                        } else if words[idx] == "se"
                            || words[idx] == "mik-jjang"
                            || words[idx] == "Are"
                            || words[idx] == "hai"
                        {
                            idx_sign = -1;
                            idx_sign_internal = 0;
                            break;
                        } else {
                            idx += 1;
                        }
                    }
                } else {
                    idx_sign = -1;
                    idx_sign_internal = 0;
                }
            }

            continue;
        }

        idx += 1;
    }

    writeln!(out, "{cnt_lead_sign} {cnt_sign}").unwrap();
}
