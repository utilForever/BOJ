use std::io;

fn input_integers() -> Vec<i64> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i64> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

struct MaximumFlow {
    graph: Vec<Vec<i64>>,
    check: Vec<bool>,
    pred: Vec<i64>,
    n: i64,
}

impl MaximumFlow {
    fn new(n: usize) -> Self {
        Self {
            graph: vec![Vec::new(); n],
            check: vec![false; n],
            pred: vec![-1; n],
            n: n as i64,
        }
    }

    fn add_edge(&mut self, u: i64, v: i64) {
        self.graph[u as usize].push(v);
    }

    fn process_dfs(&mut self, x: i64) -> bool {
        if x == -1 {
            return true;
        }

        for i in 0..self.graph[x as usize].len() {
            if self.check[self.graph[x as usize][i] as usize] {
                continue;
            }

            self.check[self.graph[x as usize][i] as usize] = true;

            if self.process_dfs(self.pred[self.graph[x as usize][i] as usize]) {
                self.pred[self.graph[x as usize][i] as usize] = x;
                return true;
            }
        }

        return false;
    }

    fn calculate_max_flow(&mut self) -> i64 {
        let mut ans = 0;

        for i in 0..self.n {
            self.check.fill(false);

            if self.process_dfs(i) {
                ans += 1;
            }
        }

        ans
    }
}

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return true;
    }

    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }

    true
}

fn main() {
    let n = input_integers()[0] as usize;
    let arr = input_integers();

    let mut ans = Vec::new();

    for i in 1..n {
        if !is_prime(arr[0] + arr[i]) {
            continue;
        }

        let mut even = Vec::new();
        let mut odd = Vec::new();

        for j in 1..n {
            if j == i {
                continue;
            }

            if arr[j] % 2 == 0 {
                even.push(arr[j]);
            } else {
                odd.push(arr[j]);
            }
        }

        if even.len() != odd.len() {
            continue;
        }

        let m = even.len();
        let mut flow = MaximumFlow::new(m);

        for j in 0..m {
            for k in 0..m {
                if is_prime(odd[j] + even[k]) {
                    flow.add_edge(j as i64, k as i64);
                }
            }
        }

        if flow.calculate_max_flow() == m as i64 {
            ans.push(arr[i]);
        }
    }

    if ans.is_empty() {
        println!("-1");
    } else {
        ans.sort();

        for val in ans.iter() {
            print!("{} ", val);
        }

        println!();
    }
}
