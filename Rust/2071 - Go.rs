use io::Write;
use std::{collections::VecDeque, io, str};

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Stone {
    Black,
    NotBlack,
    Unknown,
}

static mut STONES_TOTAL_ROW: [i32; 15] = [0; 15];
static mut STONES_TOTAL_COLUMN: [i32; 15] = [0; 15];
static mut STONES_TOTAL_DIAGONAL_SLASH: [i32; 29] = [0; 29];
static mut STONES_TOTAL_DIAGONAL_BACKSLASH: [i32; 29] = [0; 29];
static mut STONES_FILLED_ROW: [i32; 15] = [0; 15];
static mut STONES_FILLED_COLUMN: [i32; 15] = [0; 15];
static mut STONES_FILLED_DIAGONAL_SLASH: [i32; 29] = [0; 29];
static mut STONES_FILLED_DIAGONAL_BACKSLASH: [i32; 29] = [0; 29];
static mut BOARD: [[Stone; 15]; 15] = [[Stone::Unknown; 15]; 15];
static mut N: usize = 0;

unsafe fn set_stone(i: usize, j: usize, stone: Stone) {
    if BOARD[i][j] != Stone::Unknown {
        return;
    }

    STONES_TOTAL_ROW[i] -= 1;
    STONES_TOTAL_COLUMN[j] -= 1;
    STONES_TOTAL_DIAGONAL_SLASH[i + j] -= 1;
    STONES_TOTAL_DIAGONAL_BACKSLASH[N - 1 - i + j] -= 1;

    if stone == Stone::Black {
        STONES_FILLED_ROW[i] -= 1;
        STONES_FILLED_COLUMN[j] -= 1;
        STONES_FILLED_DIAGONAL_SLASH[i + j] -= 1;
        STONES_FILLED_DIAGONAL_BACKSLASH[N - 1 - i + j] -= 1;
    }

    BOARD[i][j] = stone;
}

unsafe fn try_stone() -> bool {
    let mut is_changed = false;

    // Fill the board by row
    for i in 0..N {
        // Skip if the row is already filled
        if STONES_TOTAL_ROW[i] == 0 {
            continue;
        }

        // Fill the row if all stones are black
        if STONES_FILLED_ROW[i] == STONES_TOTAL_ROW[i] {
            is_changed = true;

            for j in 0..N {
                set_stone(i, j, Stone::Black);
            }

            continue;
        }

        // Fill the row if all stones are not black
        if STONES_FILLED_ROW[i] == 0 {
            is_changed = true;

            for j in 0..N {
                set_stone(i, j, Stone::NotBlack);
            }

            continue;
        }
    }

    // Fill the board by column
    for j in 0..N {
        // Skip if the column is already filled
        if STONES_TOTAL_COLUMN[j] == 0 {
            continue;
        }

        // Fill the column if all stones are black
        if STONES_FILLED_COLUMN[j] == STONES_TOTAL_COLUMN[j] {
            is_changed = true;

            for i in 0..N {
                set_stone(i, j, Stone::Black);
            }

            continue;
        }

        // Fill the column if all stones are not black
        if STONES_FILLED_COLUMN[j] == 0 {
            is_changed = true;

            for i in 0..N {
                set_stone(i, j, Stone::NotBlack);
            }

            continue;
        }
    }

    // Fill the board by diagonal slash
    for i in 0..2 * N - 1 {
        // Skip if the diagonal slash is already filled
        if STONES_TOTAL_DIAGONAL_SLASH[i] == 0 {
            continue;
        }

        // The start position of the diagonal slash
        let mut start_y = if i < N { i } else { N - 1 };
        let mut start_x = if i < N { 0 } else { i - N + 1 };
        let cnt = if i < N { i + 1 } else { 2 * N - 1 - i };

        // Fill the diagonal slash if all stones are black
        if STONES_FILLED_DIAGONAL_SLASH[i] == STONES_TOTAL_DIAGONAL_SLASH[i] {
            is_changed = true;

            for _ in 0..cnt {
                set_stone(start_y, start_x, Stone::Black);

                start_y -= 1;
                start_x += 1;
            }

            continue;
        }

        // Fill the diagonal slash if all stones are not black
        if STONES_FILLED_DIAGONAL_SLASH[i] == 0 {
            is_changed = true;

            for _ in 0..cnt {
                set_stone(start_y, start_x, Stone::NotBlack);

                start_y -= 1;
                start_x += 1;
            }

            continue;
        }
    }

    // Fill the board by diagonal backslash
    for i in 0..2 * N - 1 {
        // Skip if the diagonal backslash is already filled
        if STONES_TOTAL_DIAGONAL_BACKSLASH[i] == 0 {
            continue;
        }

        // The start position of the diagonal backslash
        let mut start_y = if i < N { N - 1 - i } else { 0 };
        let mut start_x = if i < N { 0 } else { i - N + 1 };
        let cnt = if i < N { i + 1 } else { 2 * N - 1 - i };

        // Fill the diagonal backslash if all stones are black
        if STONES_FILLED_DIAGONAL_BACKSLASH[i] == STONES_TOTAL_DIAGONAL_BACKSLASH[i] {
            is_changed = true;

            for _ in 0..cnt {
                set_stone(start_y, start_x, Stone::Black);

                start_y += 1;
                start_x += 1;
            }

            continue;
        }

        // Fill the diagonal backslash if all stones are not black
        if STONES_FILLED_DIAGONAL_BACKSLASH[i] == 0 {
            is_changed = true;

            for _ in 0..cnt {
                set_stone(start_y, start_x, Stone::NotBlack);

                start_y += 1;
                start_x += 1;
            }

            continue;
        }
    }

    is_changed
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    unsafe {
        N = scan.token::<usize>();

        for i in 0..N {
            STONES_TOTAL_ROW[i] = N as i32;
            STONES_FILLED_ROW[i] = scan.token::<i32>();
        }

        for i in 0..N {
            STONES_TOTAL_COLUMN[i] = N as i32;
            STONES_FILLED_COLUMN[i] = scan.token::<i32>();
        }

        for i in 0..2 * N - 1 {
            STONES_TOTAL_DIAGONAL_SLASH[i] = (if i < N { i + 1 } else { 2 * N - 1 - i }) as i32;
            STONES_FILLED_DIAGONAL_SLASH[i] = scan.token::<i32>();
        }

        for i in 0..2 * N - 1 {
            STONES_TOTAL_DIAGONAL_BACKSLASH[i] = (if i < N { i + 1 } else { 2 * N - 1 - i }) as i32;
            STONES_FILLED_DIAGONAL_BACKSLASH[i] = scan.token::<i32>();
        }

        // First, fill the board by the constraints (all black or all not black)
        loop {
            let is_changed = try_stone();

            if !is_changed {
                break;
            }
        }

        // Print board for debugging (Part 1)
        // for i in 0..n {
        //     for j in 0..n {
        //         write!(out, "{}", board[i][j]).unwrap();
        //     }

        //     writeln!(out).unwrap();
        // }

        // writeln!(out).unwrap();

        // Second, fill the board by the remaining stones
        if BOARD.iter().flatten().any(|&x| x == Stone::Unknown) {
            let mut stones_rest = Vec::new();

            for i in 0..N {
                for j in 0..N {
                    if BOARD[i][j] == Stone::Unknown {
                        stones_rest.push((i, j));
                    }
                }
            }

            for stone in stones_rest {
                let (y, x) = stone;

                // Skip if the stone can't be filled due to the constraints
                if STONES_TOTAL_ROW[y] == 0
                    || STONES_TOTAL_COLUMN[x] == 0
                    || STONES_TOTAL_DIAGONAL_SLASH[y + x] == 0
                    || STONES_TOTAL_DIAGONAL_BACKSLASH[N - 1 - y + x] == 0
                    || STONES_FILLED_ROW[y] == 0
                    || STONES_FILLED_COLUMN[x] == 0
                    || STONES_FILLED_DIAGONAL_SLASH[y + x] == 0
                    || STONES_FILLED_DIAGONAL_BACKSLASH[N - 1 - y + x] == 0
                {
                    continue;
                }

                set_stone(y, x, Stone::Black);

                // Fill the board by the constraints (all black or all not black)
                // This is necessary because the new stone can change the constraints
                loop {
                    let is_changed = try_stone();

                    if !is_changed {
                        break;
                    }
                }
            }
        }

        // Print board for debugging (Part 2)
        // for i in 0..n {
        //     for j in 0..n {
        //         write!(out, "{}", board[i][j]).unwrap();
        //     }

        //     writeln!(out).unwrap();
        // }

        // writeln!(out).unwrap();

        // Calculate the point of the board by BFS
        let mut visited = vec![vec![false; N]; N];

        for i in 0..N {
            for j in 0..N {
                // Skip if the stone is black (Check empty cell only)
                if BOARD[i][j] == Stone::Black {
                    visited[i][j] = true;
                }
            }
        }

        let dy = [-1, 0, 1, 0];
        let dx = [0, 1, 0, -1];

        // Check the outside of the black stones
        for i in 0..N {
            for j in 0..N {
                // Skip if the position is not the corner
                // Because it is empty cell surrounded by black stones
                if i > 0 && i < N - 1 && j > 0 && j < N - 1 {
                    continue;
                }

                if visited[i][j] {
                    continue;
                }

                let mut queue = VecDeque::new();
                queue.push_back((i, j));
                visited[i][j] = true;

                while !queue.is_empty() {
                    let (y, x) = queue.pop_front().unwrap();

                    for k in 0..4 {
                        let y_next = y as i32 + dy[k];
                        let x_next = x as i32 + dx[k];

                        if y_next < 0 || y_next >= N as i32 || x_next < 0 || x_next >= N as i32 {
                            continue;
                        }

                        let y_next = y_next as usize;
                        let x_next = x_next as usize;

                        if visited[y_next][x_next] {
                            continue;
                        }

                        if BOARD[y_next][x_next] == Stone::NotBlack {
                            visited[y_next][x_next] = true;
                            queue.push_back((y_next, x_next));
                        }
                    }
                }
            }
        }

        writeln!(out, "{}", visited.iter().flatten().filter(|&&x| !x).count()).unwrap();
    }
}
