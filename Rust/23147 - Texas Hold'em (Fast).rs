use io::Write;
use std::{
    cmp::{Ord, Ordering},
    io, str,
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

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        let val = max;

        max = min;
        min = val;
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[inline]
pub fn next_permutation(nums: &mut Vec<usize>) -> bool {
    let last_ascending = match nums.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => {
            nums.reverse();
            return false;
        }
    };

    let swap_with = nums[last_ascending + 1..]
        .binary_search_by(|n| match nums[last_ascending].cmp(n) {
            Ordering::Equal => Ordering::Greater,
            ord => ord,
        })
        .unwrap_err();
    nums.swap(last_ascending, last_ascending + swap_with);
    nums[last_ascending + 1..].reverse();

    true
}

#[inline]
fn write_index(
    power: &mut Vec<Vec<Vec<Vec<Vec<Vec<i32>>>>>>,
    mut ranks: Vec<usize>,
    idx: &mut i32,
    is_same_suit: usize,
) {
    ranks.sort();

    loop {
        power[ranks[0]][ranks[1]][ranks[2]][ranks[3]][ranks[4]][is_same_suit] = *idx;

        if !next_permutation(&mut ranks) {
            break;
        }
    }

    *idx += 1;
}

const PERMUTATIONS: [[usize; 5]; 21] = [
    [0, 1, 2, 3, 4],
    [0, 1, 2, 3, 5],
    [0, 1, 2, 3, 6],
    [0, 1, 2, 4, 5],
    [0, 1, 2, 4, 6],
    [0, 1, 2, 5, 6],
    [0, 1, 3, 4, 5],
    [0, 1, 3, 4, 6],
    [0, 1, 3, 5, 6],
    [0, 1, 4, 5, 6],
    [0, 2, 3, 4, 5],
    [0, 2, 3, 4, 6],
    [0, 2, 3, 5, 6],
    [0, 2, 4, 5, 6],
    [0, 3, 4, 5, 6],
    [1, 2, 3, 4, 5],
    [1, 2, 3, 4, 6],
    [1, 2, 3, 5, 6],
    [1, 2, 4, 5, 6],
    [1, 3, 4, 5, 6],
    [2, 3, 4, 5, 6],
];

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    // Precompute
    let mut power = vec![vec![vec![vec![vec![vec![0; 2]; 13]; 13]; 13]; 13]; 13];
    let mut idx = 0;

    // High card
    for i in 0..13 {
        for j in 0..i {
            for k in 0..j {
                for l in 0..k {
                    for m in 0..l {
                        // Ignore straight
                        if i as i32 - m as i32 == 4 {
                            continue;
                        }

                        // Ignore royal flush
                        if i == 12 && j == 3 && k == 2 && l == 1 && m == 0 {
                            continue;
                        }

                        write_index(&mut power, vec![i, j, k, l, m], &mut idx, 0);
                    }
                }
            }
        }
    }

    // One pair
    for i in 0..13 {
        for j in 0..13 {
            for k in 0..j {
                for l in 0..k {
                    if i == j || i == k || i == l {
                        continue;
                    }

                    write_index(&mut power, vec![i, i, j, k, l], &mut idx, 0);
                }
            }
        }
    }

    // Two pair
    for i in 0..13 {
        for j in 0..i {
            for k in 0..13 {
                if i == k || j == k {
                    continue;
                }

                write_index(&mut power, vec![i, i, j, j, k], &mut idx, 0);
            }
        }
    }

    // Three of a kind
    for i in 0..13 {
        for j in 0..13 {
            if i == j {
                continue;
            }

            for k in 0..j {
                if i == k {
                    continue;
                }

                write_index(&mut power, vec![i, i, i, j, k], &mut idx, 0);
            }
        }
    }

    // Baby straight
    write_index(&mut power, vec![12, 3, 2, 1, 0], &mut idx, 0);

    // Straight
    for i in 4..13 {
        write_index(&mut power, vec![i, i - 1, i - 2, i - 3, i - 4], &mut idx, 0);
    }

    // Flush
    for i in 0..13 {
        for j in 0..i {
            for k in 0..j {
                for l in 0..k {
                    for m in 0..l {
                        // Ignore straight flush
                        if i as i32 - m as i32 == 4 {
                            continue;
                        }

                        // Ignore royal flush
                        if i == 12 && j == 3 && k == 2 && l == 1 && m == 0 {
                            continue;
                        }

                        write_index(&mut power, vec![i, j, k, l, m], &mut idx, 1);
                    }
                }
            }
        }
    }

    // Full house
    for i in 0..13 {
        for j in 0..13 {
            if i == j {
                continue;
            }

            write_index(&mut power, vec![i, i, i, j, j], &mut idx, 0);
        }
    }

    // Four of a kind
    for i in 0..13 {
        for j in 0..13 {
            if i == j {
                continue;
            }

            write_index(&mut power, vec![i, i, i, i, j], &mut idx, 0);
        }
    }

    // Baby straight flush
    write_index(&mut power, vec![12, 3, 2, 1, 0], &mut idx, 1);

    // Straight flush
    for i in 4..13 {
        write_index(&mut power, vec![i, i - 1, i - 2, i - 3, i - 4], &mut idx, 1);
    }

    let convert_rank = |rank: &str| -> usize {
        let ret = match rank {
            "2" => 0,
            "3" => 1,
            "4" => 2,
            "5" => 3,
            "6" => 4,
            "7" => 5,
            "8" => 6,
            "9" => 7,
            "T" => 8,
            "J" => 9,
            "Q" => 10,
            "K" => 11,
            "A" => 12,
            _ => panic!("Invalid rank"),
        };

        ret
    };

    let t = scan.token::<i32>();
    let cards_all = (0..13)
        .flat_map(|rank| (0..4).map(move |suit| (rank, suit)))
        .collect::<Vec<_>>();

    for _ in 0..t {
        let (w, m) = (scan.token::<i32>(), scan.token::<i32>());
        let mut cards_alice = vec![(0, 0); 7];
        let mut cards_bob = vec![(0, 0); 7];

        for card in cards_alice.iter_mut().take(2) {
            let (suit, rank) = (scan.token::<i32>(), scan.token::<String>());
            *card = (convert_rank(&rank), suit);
        }

        for card in cards_bob.iter_mut().take(2) {
            let (suit, rank) = (scan.token::<i32>(), scan.token::<String>());
            *card = (convert_rank(&rank), suit);
        }

        let mut cards_all_except_alice_and_bob = cards_all.clone();

        for card in cards_alice[0..2].iter() {
            cards_all_except_alice_and_bob.retain(|&x| x != *card);
        }

        for card in cards_bob[0..2].iter() {
            cards_all_except_alice_and_bob.retain(|&x| x != *card);
        }

        let mut cnt_win_alice = 0;
        let mut cnt_win_bob = 0;
        let mut cnt_draw = 0;

        for i in 0..44 {
            cards_alice[2] = cards_all_except_alice_and_bob[i];
            cards_bob[2] = cards_all_except_alice_and_bob[i];

            for j in i + 1..45 {
                cards_alice[3] = cards_all_except_alice_and_bob[j];
                cards_bob[3] = cards_all_except_alice_and_bob[j];

                for k in j + 1..46 {
                    cards_alice[4] = cards_all_except_alice_and_bob[k];
                    cards_bob[4] = cards_all_except_alice_and_bob[k];

                    for l in k + 1..47 {
                        cards_alice[5] = cards_all_except_alice_and_bob[l];
                        cards_bob[5] = cards_all_except_alice_and_bob[l];

                        for m in l + 1..48 {
                            cards_alice[6] = cards_all_except_alice_and_bob[m];
                            cards_bob[6] = cards_all_except_alice_and_bob[m];

                            let mut cards_alice_sorted = cards_alice.to_vec();
                            let mut cards_bob_sorted = cards_bob.to_vec();
                            let mut power_max_alice = 0;
                            let mut power_max_bob = 0;

                            cards_alice_sorted.sort_by(|a, b| b.0.cmp(&a.0));
                            cards_bob_sorted.sort_by(|a, b| b.0.cmp(&a.0));

                            for permutation in PERMUTATIONS.iter() {
                                let values_alice = (
                                    cards_alice_sorted[permutation[0]],
                                    cards_alice_sorted[permutation[1]],
                                    cards_alice_sorted[permutation[2]],
                                    cards_alice_sorted[permutation[3]],
                                    cards_alice_sorted[permutation[4]],
                                );
                                let values_bob = (
                                    cards_bob_sorted[permutation[0]],
                                    cards_bob_sorted[permutation[1]],
                                    cards_bob_sorted[permutation[2]],
                                    cards_bob_sorted[permutation[3]],
                                    cards_bob_sorted[permutation[4]],
                                );

                                let is_alice_cards_same_suit = values_alice.0 .1
                                    == values_alice.1 .1
                                    && values_alice.1 .1 == values_alice.2 .1
                                    && values_alice.2 .1 == values_alice.3 .1
                                    && values_alice.3 .1 == values_alice.4 .1;
                                let is_bob_cards_same_suit = values_bob.0 .1 == values_bob.1 .1
                                    && values_bob.1 .1 == values_bob.2 .1
                                    && values_bob.2 .1 == values_bob.3 .1
                                    && values_bob.3 .1 == values_bob.4 .1;

                                power_max_alice = power[values_alice.0 .0][values_alice.1 .0]
                                    [values_alice.2 .0][values_alice.3 .0][values_alice.4 .0]
                                    [is_alice_cards_same_suit as usize]
                                    .max(power_max_alice);
                                power_max_bob = power[values_bob.0 .0][values_bob.1 .0]
                                    [values_bob.2 .0][values_bob.3 .0][values_bob.4 .0]
                                    [is_bob_cards_same_suit as usize]
                                    .max(power_max_bob);
                            }

                            match power_max_alice.cmp(&power_max_bob) {
                                Ordering::Less => cnt_win_bob += 1,
                                Ordering::Equal => cnt_draw += 1,
                                Ordering::Greater => cnt_win_alice += 1,
                            }
                        }
                    }
                }
            }
        }

        if cnt_win_alice == cnt_win_bob {
            writeln!(out, "{}/1", w + m).unwrap();
        } else if cnt_win_alice > cnt_win_bob {
            let fold = -w as i64 * (cnt_win_alice + cnt_win_bob + cnt_draw) as i64;
            let all_in = -(w + m) as i64 * (cnt_win_alice - cnt_win_bob) as i64;

            if fold > all_in {
                writeln!(out, "{}/1", w + w + m).unwrap();
            } else {
                let numerator = (w + m) as i64
                    * (cnt_win_alice + cnt_win_bob + cnt_draw + (cnt_win_alice - cnt_win_bob))
                        as i64;
                let denominator = (cnt_win_alice + cnt_win_bob + cnt_draw) as i64;
                let g = gcd(numerator, denominator);

                writeln!(out, "{}/{}", numerator / g, denominator / g).unwrap();
            }
        } else {
            let fold = -w as i64 * (cnt_win_alice + cnt_win_bob + cnt_draw) as i64;
            let all_in = -(w + m) as i64 * (cnt_win_bob - cnt_win_alice) as i64;

            if fold > all_in {
                writeln!(out, "{}/1", m).unwrap();
            } else {
                let numerator = (w + m) as i64
                    * (cnt_win_alice + cnt_win_bob + cnt_draw + (cnt_win_alice - cnt_win_bob))
                        as i64;
                let denominator = (cnt_win_alice + cnt_win_bob + cnt_draw) as i64;
                let g = gcd(numerator, denominator);

                writeln!(out, "{}/{}", numerator / g, denominator / g).unwrap();
            }
        }
    }
}
