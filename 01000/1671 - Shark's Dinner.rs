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
            for _ in 0..2 {
                self.check.fill(false);

                if self.process_dfs(i) {
                    ans += 1;
                }
            }
        }

        ans
    }
}

fn main() {
    let n = input_integers()[0] as usize;

    let mut flow = MaximumFlow::new(n);
    let mut size = vec![0; n];
    let mut speed = vec![0; n];
    let mut intelligence = vec![0; n];

    for i in 0..n {
        let nums = input_integers();

        size[i] = nums[0];
        speed[i] = nums[1];
        intelligence[i] = nums[2];
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            if size[i] == size[j] && speed[i] == speed[j] && intelligence[i] == intelligence[j] {
                if i < j {
                    flow.add_edge(i as i64, j as i64);
                }
            } else if size[i] >= size[j]
                && speed[i] >= speed[j]
                && intelligence[i] >= intelligence[j]
            {
                flow.add_edge(i as i64, j as i64);
            }
        }
    }

    println!("{}", n as i64 - flow.calculate_max_flow());
}
