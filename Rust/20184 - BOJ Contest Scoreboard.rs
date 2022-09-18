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

struct Contest {
    penalty: i64,
    start_date: String,
    start_time: String,
    last: bool,
    ce: bool,
    cscore: bool,
    format: bool,

    problems: Vec<Problem>,
    submissions: Vec<Submission>,
    scoreboard: Scoreboard,
}

impl Contest {
    fn new(
        penalty: i64,
        start_date: String,
        start_time: String,
        last: bool,
        ce: bool,
        cscore: bool,
        format: bool,
    ) -> Self {
        Self {
            penalty,
            start_date,
            start_time,
            last,
            ce,
            cscore,
            format,
            problems: Vec::new(),
            submissions: Vec::new(),
            scoreboard: Scoreboard::new(),
        }
    }

    fn find_problem_idx(&self, pid: i64) -> usize {
        self.problems.iter().position(|p| p.id == pid).unwrap()
    }

    fn init_scoreboard(&mut self, users: &Vec<String>, num_problems: usize) {
        self.scoreboard.init(users, num_problems);
    }

    fn process_submissions(&mut self) {
        for submission in self.submissions.iter() {
            if submission.result == 13 {
                continue;
            }

            if submission.result == 11 {
                if self.ce {
                    continue;
                }

                self.scoreboard.add_penalty(
                    submission.uid.clone(),
                    self.find_problem_idx(submission.pid),
                    self.penalty,
                );
            }

            if submission.result == 4 {
                let pidx = self.find_problem_idx(submission.pid);
                let panelty = 0;

                self.scoreboard.add_score(
                    self.ce,
                    submission.uid.clone(),
                    pidx,
                    self.problems[pidx].pscore,
                    panelty,
                );
            }
        }
    }
}

struct Problem {
    id: i64,
    order: i64,
    pscore: i64,
}

impl Problem {
    fn new(id: i64, order: i64, pscore: i64) -> Self {
        Self { id, order, pscore }
    }
}

struct Submission {
    sid: i64,
    pid: i64,
    uid: String,
    result: i64,
    presult: bool,
    score: i64,
    date: String,
    time: String,
}

impl Submission {
    fn new(
        sid: i64,
        pid: i64,
        uid: String,
        result: i64,
        presult: bool,
        score: i64,
        date: String,
        time: String,
    ) -> Self {
        Self {
            sid,
            pid,
            uid,
            result,
            presult,
            score,
            date,
            time,
        }
    }
}

struct Scoreboard {
    scores: Vec<Score>,
}

impl Scoreboard {
    fn new() -> Self {
        Self { scores: Vec::new() }
    }

    fn init(&mut self, users: &Vec<String>, num_problems: usize) {
        self.scores = users.iter().map(|u| Score::new(u, num_problems)).collect();
    }

    /*
                    self.scoreboard.add_score(
                    self.ce,
                    submission.uid.clone(),
                    pidx,
                    self.problems[pidx].pscore,
                    submission.date,
                    submission.time,
                );
    */

    fn add_score(&mut self, ce: bool, uid: String, pidx: usize, pscore: i64, panelty: i64) {}

    fn add_penalty(&mut self, uid: String, pidx: usize, penalty: i64) {
        self.scores
            .iter_mut()
            .find(|s| s.uid == uid)
            .unwrap()
            .add_penalty1(pidx, penalty);
    }
}

#[derive(Clone)]
struct ScoreInfo {
    result: char,
    score: i64,
    tries: i64,
    penalty1: i64,
    penalty2: i64,
}

impl ScoreInfo {
    fn new(result: char, score: i64, tries: i64, penalty1: i64, penalty2: i64) -> Self {
        Self {
            result,
            score,
            tries,
            penalty1,
            penalty2,
        }
    }
}

struct Score {
    rank: i64,
    uid: String,
    results: Vec<ScoreInfo>,
}

impl Score {
    fn new(uid: &String, num_problems: usize) -> Self {
        Self {
            rank: 0,
            uid: uid.clone(),
            results: vec![ScoreInfo::new('-', 0, 0, 0, 0); num_problems],
        }
    }

    fn add_penalty1(&mut self, pidx: usize, penalty: i64) {
        if self.results[pidx].result != 'a' && self.results[pidx].result != 'p' {
            self.results[pidx].result = 'w';
        }

        self.results[pidx].penalty1 += penalty;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut contest = Contest::new(
        scan.token::<i64>(),
        scan.token::<String>(),
        scan.token::<String>(),
        scan.token::<i64>() == 1,
        scan.token::<i64>() == 1,
        scan.token::<i64>() == 1,
        scan.token::<i64>() == 1,
    );

    let n = scan.token::<usize>();

    for _ in 0..n {
        contest.problems.push(Problem::new(
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        ));
    }

    let m = scan.token::<i64>();
    let mut users = Vec::new();

    for _ in 0..m {
        users.push(scan.token::<String>());
    }

    let s = scan.token::<i64>();

    for _ in 0..s {
        contest.submissions.push(Submission::new(
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<String>(),
            scan.token::<i64>(),
            scan.token::<i64>() == 1,
            scan.token::<i64>(),
            scan.token::<String>(),
            scan.token::<String>(),
        ));
    }

    contest.init_scoreboard(&users, n);
    contest.process_submissions();
}
