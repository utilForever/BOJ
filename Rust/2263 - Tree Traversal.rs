use io::Write;
use std::{
    io::{self, BufWriter, StdoutLock},
    str,
};

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

fn traverse(
    out: &mut BufWriter<StdoutLock>,
    inorder_idx: &Vec<usize>,
    postorder: &Vec<usize>,
    inorder_start_idx: usize,
    inorder_end_idx: usize,
    postorder_start_idx: usize,
    postorder_end_idx: usize,
) {
    if inorder_start_idx > inorder_end_idx || postorder_start_idx > postorder_end_idx {
        return;
    }

    write!(out, "{} ", postorder[postorder_end_idx]).unwrap();

    let root_idx = inorder_idx[postorder[postorder_end_idx]];
    let left_subtree_size = root_idx - inorder_start_idx;

    traverse(
        out,
        inorder_idx,
        postorder,
        inorder_start_idx,
        root_idx - 1,
        postorder_start_idx,
        postorder_start_idx + left_subtree_size - 1,
    );
    traverse(
        out,
        inorder_idx,
        postorder,
        root_idx + 1,
        inorder_end_idx,
        postorder_start_idx + left_subtree_size,
        postorder_end_idx - 1,
    );
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut inorder_idx = vec![0; n + 1];
    let mut inorder = vec![0; n + 1];
    let mut postorder = vec![0; n + 1];

    for i in 1..=n {
        inorder[i] = scan.token::<usize>();
        inorder_idx[inorder[i]] = i;
    }

    for i in 1..=n {
        postorder[i] = scan.token::<usize>();
    }

    traverse(&mut out, &inorder_idx, &postorder, 1, n, 1, n);
}
