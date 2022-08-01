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

fn pow(x: i64, mut p: i64, modulo: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x;

    while p != 0 {
        if p & 1 != 0 {
            ret = ret * piv % modulo;
        }

        piv = piv * piv % modulo;
        p >>= 1;
    }

    ret
}

fn berlekamp_massey(vals: Vec<i64>, modulo: i64) -> Vec<i64> {
    let mut ls = Vec::new();
    let mut cur = Vec::new();

    let mut lf = 0;
    let mut ld = 0;

    for i in 0..vals.len() {
        let mut t = 0;

        for j in 0..cur.len() {
            t = (t + vals[i - j - 1] * cur[j]) % modulo;
        }

        if (t - vals[i]) % modulo == 0 {
            continue;
        }

        if cur.is_empty() {
            cur.resize(i + 1, 0);
            lf = i;
            ld = (t - vals[i]) % modulo;

            continue;
        }

        let k = -(vals[i] - t) * pow(ld, modulo - 2, modulo) % modulo;

        let mut c = vec![0; i - lf - 1];
        c.push(k);

        for j in ls.iter() {
            c.push(-j * k % modulo);
        }

        if c.len() < cur.len() {
            c.resize(cur.len(), 0);
        }

        for j in 0..cur.len() {
            c[j] = (c[j] + cur[j]) % modulo;
        }

        if i - lf + ls.len() >= cur.len() {
            (ls, lf, ld) = (cur, i, (t - vals[i]) % modulo);
        }

        cur = c;
    }

    for i in cur.iter_mut() {
        *i = (*i % modulo + modulo) % modulo;
    }

    cur
}

fn get_nth(rec: Vec<i64>, dp: Vec<i64>, mut n: usize, modulo: i64) -> i64 {
    let m = rec.len();
    let mut s = vec![0; m];
    let mut t = vec![0; m];

    s[0] = 1;
    if m != 1 {
        t[1] = 1;
    } else {
        t[0] = rec[0];
    }

    let mul = |v: Vec<i64>, w: Vec<i64>| -> Vec<i64> {
        let m = v.len();
        let mut t = vec![0; 2 * m];

        for j in 0..m {
            for k in 0..m {
                t[j + k] += v[j] * w[k] % modulo;

                if t[j + k] >= modulo {
                    t[j + k] -= modulo;
                }
            }
        }

        for j in (m..=2 * m - 1).rev() {
            for k in 1..=m {
                t[j - k] += t[j] * rec[k - 1] % modulo;

                if t[j - k] >= modulo {
                    t[j - k] -= modulo;
                }
            }
        }

        t.resize(m, 0);

        t
    };

    while n != 0 {
        if n & 1 != 0 {
            s = mul(s, t.clone());
        }

        t = mul(t.clone(), t.clone());
        n >>= 1;
    }

    let mut ret = 0;

    for i in 0..m {
        ret += s[i] * dp[i] % modulo;
    }

    ret % modulo
}

fn guess_nth_term(vals: Vec<i64>, n: usize, modulo: i64) -> i64 {
    if n < vals.len() {
        return vals[n as usize];
    }

    let ret = berlekamp_massey(vals.clone(), modulo);

    if ret.is_empty() {
        0
    } else {
        get_nth(ret, vals, n, modulo)
    }
}

// Reference: https://koosaga.com/231
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let precomputed = vec![
        272, 589185, 930336768, 853401154, 217676188, 136558333, 415722813, 985269529, 791527976,
        201836136, 382110354, 441223705, 661537677, 641601343, 897033284, 816519670, 365311407,
        300643484, 936803543, 681929467, 462484986, 13900203, 657627114, 96637209, 577140657,
        600647073, 254604056, 102389682, 811580173, 592550067, 587171680, 526467503, 265885773,
        951722780, 219627841, 371508152, 283501391, 159234514, 439380999, 722868959, 125599834,
        351398134, 456317548, 365496182, 614778702, 502680047, 193063685, 309004764, 743901785,
        870955115, 312807829, 160375015, 691844624, 137034372, 350330868, 895680450, 282610535,
        317897557, 28600551, 583305647, 539409363, 327406961, 627805385, 680183978, 681299085,
        954964592, 743524009, 788048339, 699454626, 666369521, 857206425, 490463127, 477198247,
        599963928, 21247982, 107843532, 753662937, 239039324, 608530376, 523383010, 654448101,
        801430395, 393034561, 93313778, 983052766, 240336620, 825539982, 525118275, 563899476,
        706271688, 547405697, 477082486, 664058071, 353207278, 729486413, 795704637, 999271072,
        540749624, 411451016, 736422999, 879369181, 918733916, 982303557, 512499644, 261033810,
        391766409, 334092786, 931794834, 854181848, 821090190, 751839258, 433126935, 571194155,
        52438113, 552977155, 320805296, 173355929, 969659468, 258854248, 159509877, 374487748,
        401382023, 44060530, 510164669, 336596764, 652050424, 373872552, 517226592, 719871041,
        43959496, 235333335, 304962191, 253114421, 43638769, 361871585, 8060121, 147014624,
        114846460, 430864038, 368951246, 863795701, 36066788, 971606149, 935875286, 486724123,
        73790652, 236936530, 307697424, 753314001, 40450345, 529462842, 166162047, 974102330,
        600865526, 63237062, 749041914, 670937123, 806399597, 776678839, 842565920, 608499253,
        469062485, 842196981, 247762946, 778570576, 237951782, 286343384, 988318575, 147255879,
        905747089, 711062313, 21396079, 826846622, 443781794, 786474911, 400737121, 844768961,
        686214818, 590050845, 855473150, 18501778, 33258755, 398169058, 811192244, 710397887,
        591757177, 775311969, 168256434, 509615161, 489764304, 605188191, 498085780, 164388183,
        524662873, 322602324, 853641480, 205349527, 308211944, 93153206, 734257752, 68829302,
        443687521, 524241394, 591557198, 308656747, 511733449, 943095360, 194572043, 420913382,
        679842332, 684364764, 134540921, 551103000, 700528141, 54414645, 814404379, 3421752,
        316740512, 853118601, 894201609, 877520795, 244106463, 358840411, 411662431, 953845173,
        239397728, 391633640, 745859650, 6417562, 246353318, 900069523, 877218664, 234394818,
        171521822, 184466314, 316351773, 353811494, 617940271, 731132804, 656046921, 2378554,
        305082811, 860468755, 877839522, 884387573, 83314799, 753963703, 702751847, 739819061,
        2908431, 897890934, 45761348, 828368065, 248920872,
    ];

    let n = scan.token::<usize>();
    let mut arr = Vec::new();

    for i in 0..precomputed.len() {
        arr.push(precomputed[i]);
    }

    writeln!(out, "{}", guess_nth_term(arr, n - 1, 1_000_000_007)).unwrap();
}
