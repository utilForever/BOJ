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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

#[derive(Debug, Clone)]
struct Op {
    a: i64,
    op: char,
    b: i64,
    c: i64,
}

fn possible_ops(a: i64, b: i64) -> Vec<(char, i64)> {
    let mut ret = Vec::new();

    ret.push(('+', a + b));

    if a > b {
        ret.push(('-', a - b));
    }

    ret.push(('*', a * b));

    if a % b == 0 {
        let div = a / b;

        if div > 0 {
            ret.push(('/', div));
        }
    }

    ret
}

fn process_dfs(
    board: &mut Vec<i64>,
    memory: &mut HashMap<Vec<i64>, Option<Vec<Op>>>,
    target: i64,
) -> Option<Vec<Op>> {
    if board.contains(&target) {
        return Some(Vec::new());
    }

    if board.len() == 1 {
        return None;
    }

    if let Some(ret) = memory.get(board) {
        return ret.clone();
    }

    for i in 0..board.len() {
        for j in 0..board.len() {
            if i == j {
                continue;
            }

            let a = board[i];
            let b = board[j];
            let mut board_new = Vec::with_capacity(board.len() - 1);

            for k in 0..board.len() {
                if k == i || k == j {
                    continue;
                }

                board_new.push(board[k]);
            }

            let candidates = possible_ops(a, b);

            for (op, c) in candidates {
                let mut board_next = board_new.clone();

                board_next.push(c);
                board_next.sort_unstable();

                if let Some(mut ops) = process_dfs(&mut board_next, memory, target) {
                    let op_next = Op { a, op, b, c };
                    ops.insert(0, op_next);
                    memory.insert(board.to_vec(), Some(ops.clone()));

                    return Some(ops);
                }
            }
        }
    }

    memory.insert(board.clone(), None);

    None
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();
    let mut board = vec![0; 6];

    for i in 0..6 {
        board[i] = scan.token::<i64>();
    }

    if board.contains(&t) {
        writeln!(out, "0").unwrap();
        return;
    }

    board.sort_unstable();

    let mut memory = HashMap::new();

    if let Some(ret) = process_dfs(&mut board, &mut memory, t) {
        writeln!(out, "{}", ret.len()).unwrap();

        for Op { a, op, b, c } in ret {
            writeln!(out, "{a} {op} {b} = {c}").unwrap();
        }
    } else {
        writeln!(out, "-1").unwrap();
    }
}
