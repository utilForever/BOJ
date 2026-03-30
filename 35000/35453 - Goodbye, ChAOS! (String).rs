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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<i64>();

        if n == 1 {
            let s = scan.token::<String>();

            writeln!(
                out,
                "{}",
                if s == "Chung-Ang_Programming_Contest" {
                    "2020"
                } else if s == "Newbie_Programming_Challenge" {
                    "2021"
                } else {
                    "Goodbye, ChAOS!"
                }
            )
            .unwrap();
        } else if n == 2 {
            let s1 = scan.token::<String>();
            let s2 = scan.token::<String>();

            writeln!(
                out,
                "{}",
                match (s1.as_str(), s2.as_str()) {
                    ("CodeRace", "Chung-Ang_Programming_Contest")
                    | ("Chung-Ang_Programming_Contest", "CodeRace") => "2017",
                    ("CodeRace", "SCAL-MOOKJA") | ("SCAL-MOOKJA", "CodeRace") => "2018",
                    ("Newbie_Programming_Contest", "Chung-Ang_Programming_Contest")
                    | ("Chung-Ang_Programming_Contest", "Newbie_Programming_Contest") => "2019",
                    ("ChAOS_Hello{Year}_Algorithm_Contest", "Chung-Ang_Programming_Contest")
                    | ("Chung-Ang_Programming_Contest", "ChAOS_Hello{Year}_Algorithm_Contest") =>
                        "2022",
                    (
                        "Kookmin_Chung-Ang_Programming_Contest",
                        "ChAOS_Hello{Year}_Algorithm_Contest",
                    )
                    | (
                        "ChAOS_Hello{Year}_Algorithm_Contest",
                        "Kookmin_Chung-Ang_Programming_Contest",
                    ) => "2023",
                    ("Kookmin_Chung-Ang_Programming_Contest", "Chung-Ang_Programming_Contest")
                    | ("Chung-Ang_Programming_Contest", "Kookmin_Chung-Ang_Programming_Contest") =>
                        "2024",
                    ("Centroid_Cup", "Chung-Ang_Programming_Contest")
                    | ("Chung-Ang_Programming_Contest", "Centroid_Cup") => "2025",
                    _ => "Goodbye, ChAOS!",
                }
            )
            .unwrap();
        }
    }
}
