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

    let n = scan.token::<i64>();

    writeln!(
        out,
        "어느 한 컴퓨터공학과 학생이 유명한 교수님을 찾아가 물었다."
    )
    .unwrap();

    for i in 0..n {
        for _ in 0..i {
            write!(out, "____").unwrap();
        }
        writeln!(out, "\"재귀함수가 뭔가요?\"").unwrap();

        for _ in 0..i {
            write!(out, "____").unwrap();
        }
        writeln!(
            out,
            "\"잘 들어보게. 옛날옛날 한 산 꼭대기에 이세상 모든 지식을 통달한 선인이 있었어."
        )
        .unwrap();

        for _ in 0..i {
            write!(out, "____").unwrap();
        }
        writeln!(
            out,
            "마을 사람들은 모두 그 선인에게 수많은 질문을 했고, 모두 지혜롭게 대답해 주었지."
        )
        .unwrap();

        for _ in 0..i {
            write!(out, "____").unwrap();
        }
        writeln!(out, "그의 답은 대부분 옳았다고 하네. 그런데 어느 날, 그 선인에게 한 선비가 찾아와서 물었어.\"").unwrap();
    }

    for _ in 0..n {
        write!(out, "____").unwrap();
    }
    writeln!(out, "\"재귀함수가 뭔가요?\"").unwrap();

    for _ in 0..n {
        write!(out, "____").unwrap();
    }
    writeln!(out, "\"재귀함수는 자기 자신을 호출하는 함수라네\"").unwrap();

    for _ in 0..n {
        write!(out, "____").unwrap();
    }
    writeln!(out, "라고 답변하였지.").unwrap();

    for i in (0..n).rev() {
        for _ in 0..i {
            write!(out, "____").unwrap();
        }
        writeln!(out, "라고 답변하였지.").unwrap();
    }
}
