use io::Write;
use std::{cmp::Reverse, collections::BinaryHeap, io, str};

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

fn process_dijkstra(graph: &Vec<Vec<(usize, i64)>>, from: usize, to: usize) -> Vec<i64> {
    let mut dist = vec![i64::MAX / 4; graph.len()];
    dist[from] = 0;

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), from));

    while let Some((Reverse(cost_curr), vertex_curr)) = heap.pop() {
        if cost_curr != dist[vertex_curr] {
            continue;
        }

        if vertex_curr == to {
            break;
        }

        for &(vertex_next, edge_cost) in graph[vertex_curr].iter() {
            let cost_next = cost_curr + edge_cost;

            if cost_next < dist[vertex_next] {
                dist[vertex_next] = cost_next;
                heap.push((Reverse(cost_next), vertex_next));
            }
        }
    }

    dist
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut heights = vec![0; n + 1];

    for i in 1..=n {
        heights[i] = scan.token::<i64>();
    }

    let start = 2 * n + 1;
    let end = 2 * n + 2;
    let mut idx = end;
    let mut graph = vec![Vec::new(); idx + 1];

    graph[start].push((1, 0));

    for i in 1..=n {
        graph[i].push((i + n, 2 * (heights[i] - 1).max(0)));

        if i < n {
            graph[i + n].push((i + 1, 0));
        }
    }

    graph[2 * n].push((end, 0));

    let mut prefix = vec![0; n + 1];
    let mut suffix = vec![0; n + 2];

    for i in 1..=n {
        prefix[i] = prefix[i - 1] + heights[i];
        graph[start].push((i + n, (prefix[i] - 1).max(0)));
    }

    for i in (1..=n).rev() {
        suffix[i] = suffix[i + 1] + heights[i];
        graph[i].push((end, (suffix[i] - 1).max(0)));
    }

    let mut pos_left = vec![Vec::new(); n + 1];
    let mut pos_right = vec![Vec::new(); n + 1];
    let mut nums = vec![0; n + 1];
    let mut stack = Vec::new();

    let top = |nums: &mut Vec<i64>, stack: &mut Vec<usize>| -> usize {
        let pos = *stack.last().unwrap();
        nums[pos] -= 1;

        if nums[pos] == 0 {
            stack.pop();
        }

        pos
    };

    for i in 1..=n {
        if heights[i] > 1 {
            stack.push(i);
            nums[i] = heights[i] - 1;
        } else if heights[i] == 0 && !stack.is_empty() {
            let pos = top(&mut nums, &mut stack);
            pos_right[pos].push(i);
        }
    }

    stack.clear();
    nums.fill(0);

    for i in (1..=n).rev() {
        if heights[i] > 1 {
            stack.push(i);
            nums[i] = heights[i] - 1;
        } else if heights[i] == 0 && !stack.is_empty() {
            let pos = top(&mut nums, &mut stack);
            pos_left[pos].push(i);
        }
    }

    for i in 1..=n {
        if heights[i] <= 1 {
            continue;
        }

        pos_right[i].reverse();

        let mut cnt_block_below = Vec::new();
        let mut graph_node_num = Vec::new();

        cnt_block_below.push(0);
        graph_node_num.push(i);

        let mut sum = 0;

        for &pos in pos_left[i].iter() {
            idx += 1;
            sum += 1;

            graph.push(Vec::new());
            cnt_block_below.push(sum);
            graph_node_num.push(idx);

            graph[pos].push((idx, (i as i64 - pos as i64)));
        }

        sum = heights[i] - (pos_right[i].len() as i64) - 2;

        let mut idx_top = (graph_node_num.len() as i64) - 1;

        for &pos in pos_right[i].iter() {
            sum += 1;

            if sum <= idx_top {
                graph[graph_node_num[sum as usize]].push((pos + n, (pos as i64 - i as i64)));
            } else {
                idx += 1;
                idx_top += 1;

                graph.push(Vec::new());
                cnt_block_below.push(sum);
                graph_node_num.push(idx);

                graph[idx].push((pos + n, (pos as i64 - i as i64)));
            }
        }

        cnt_block_below.push(heights[i] - 1);
        graph_node_num.push(i + n);

        for i in 0..graph_node_num.len() - 1 {
            let cost = 2 * (cnt_block_below[i + 1] - cnt_block_below[i]);
            graph[graph_node_num[i]].push((graph_node_num[i + 1], cost));
        }
    }

    let dist = process_dijkstra(&graph, start, end);

    writeln!(out, "{}", dist[end]).unwrap();
}
