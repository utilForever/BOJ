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

fn main() {
    let c = input_integers()[0] as usize;

    for _ in 0..c {
        let nums = input_integers();
        let (n, m) = (nums[0] as usize, nums[1] as usize);

        let mut classroom = vec![String::new(); n];

        for i in 0..n {
            let mut s = String::new();
            io::stdin().read_line(&mut s).unwrap();
            classroom[i] = s.trim().to_string();
        }

        let mut flow = MaximumFlow::new(n * m);

        for i in 0..n {
            for j in 0..m {
                if classroom[i].chars().nth(j).unwrap() == 'x' {
                    continue;
                }

                if i > 0 {
                    if j > 0 {
                        if classroom[i - 1].chars().nth(j - 1).unwrap() == '.' {
                            if j % 2 == 0 {
                                flow.add_edge((i * m + j) as i64, ((i - 1) * m + j - 1) as i64);
                            } else {
                                flow.add_edge(((i - 1) * m + j - 1) as i64, (i * m + j) as i64);
                            }
                        }
                    }

                    if j + 1 < m {
                        if classroom[i - 1].chars().nth(j + 1).unwrap() == '.' {
                            if j % 2 == 0 {
                                flow.add_edge((i * m + j) as i64, ((i - 1) * m + j + 1) as i64);
                            } else {
                                flow.add_edge(((i - 1) * m + j + 1) as i64, (i * m + j) as i64);
                            }
                        }
                    }
                }

                if j > 0 {
                    if classroom[i].chars().nth(j - 1).unwrap() == '.' {
                        if j % 2 == 0 {
                            flow.add_edge((i * m + j) as i64, (i * m + j - 1) as i64);
                        } else {
                            flow.add_edge((i * m + j - 1) as i64, (i * m + j) as i64);
                        }
                    }
                }
            }
        }

        let mut ans = 0;

        for i in 0..n {
            for j in 0..m {
                if classroom[i].chars().nth(j).unwrap() == '.' {
                    ans += 1;
                }
            }
        }

        println!("{}", ans - flow.calculate_max_flow());
    }
}
