// Generated with https://github.com/boj-rs/basm-rs
// Learn rust (https://doc.rust-lang.org/book/) and get high performance out of the box!

#![crate_type = "cdylib"] // On Windows, omit this or pass '--crate-type=bin' to rustc to avoid DLL creation.
#![cfg_attr(not(windows), no_std)]#![allow(unused)]#[no_link]extern crate std as s;

// SOLUTION BEGIN
#[cfg(any())] mod solution {
use alloc::vec;
use alloc::vec::Vec;
use basm::platform::io::{MmapReader, Print, ReaderTrait, Writer};

mod bit {
    use alloc::vec::Vec;

    #[inline(always)]
    pub fn get(row: &[u64], col: usize) -> bool {
        let w = col >> 6;
        let b = col & 63;

        ((row[w] >> b) & 1) != 0
    }

    #[inline(always)]
    pub fn set(row: &mut [u64], col: usize, val: bool) {
        let w = col >> 6;
        let b = col & 63;
        let mask = 1u64 << b;

        if val {
            row[w] |= mask;
        } else {
            row[w] &= !mask;
        }
    }

    #[inline(always)]
    pub fn clear(row: &mut [u64], col: usize) {
        let w = col >> 6;
        let b = col & 63;

        row[w] &= !(1u64 << b);
    }

    #[inline(always)]
    pub fn xor(dst: &mut Vec<u64>, src: &Vec<u64>) {
        for (a, b) in dst.iter_mut().zip(src.iter()) {
            *a ^= *b;
        }
    }
}

fn rref_with_pivot_rows(
    mat: &mut Vec<Vec<u64>>,
    idxes_row: &mut Vec<usize>,
    cnt_cols: usize,
) -> (usize, Vec<usize>, Vec<usize>) {
    let rows = mat.len();
    let mut rank = 0;
    let mut pivot_rows = Vec::new();
    let mut pivot_cols = Vec::new();

    for col in 0..cnt_cols {
        let mut pivot = None;

        for r in rank..rows {
            if bit::get(&mat[r], col) {
                pivot = Some(r);
                break;
            }
        }

        if let Some(pivot) = pivot {
            mat.swap(rank, pivot);
            idxes_row.swap(rank, pivot);

            let pivot_row = mat[rank].clone();

            for r in 0..rows {
                if r != rank && bit::get(&mat[r], col) {
                    bit::xor(&mut mat[r], &pivot_row);
                }
            }

            pivot_rows.push(idxes_row[rank]);
            pivot_cols.push(col);

            rank += 1;

            if rank == cnt_cols {
                break;
            }
        }
    }

    (rank, pivot_rows, pivot_cols)
}

fn rref_with_ops(mat: &mut Vec<Vec<u64>>, ops: &mut Vec<Vec<u64>>, cnt_cols: usize) -> usize {
    let rows = mat.len();
    let mut rank = 0;

    for col in 0..cnt_cols {
        let mut pivot = None;

        for r in rank..rows {
            if bit::get(&mat[r], col) {
                pivot = Some(r);
                break;
            }
        }

        if let Some(pivot) = pivot {
            if pivot != rank {
                mat.swap(rank, pivot);
                ops.swap(rank, pivot);
            }

            let pivot_mat = mat[rank].clone();
            let pivot_ops = ops[rank].clone();

            for r in 0..rows {
                if r != rank && bit::get(&mat[r], col) {
                    bit::xor(&mut mat[r], &pivot_mat);
                    bit::xor(&mut ops[r], &pivot_ops);
                }
            }

            rank += 1;

            if rank == cnt_cols {
                break;
            }
        }
    }

    rank
}

fn parity_det(mut mat: Vec<Vec<u64>>, n: usize) -> bool {
    let rows = mat.len();
    let mut idxes_row = (0..rows).collect::<Vec<_>>();
    let (rank, _, _) = rref_with_pivot_rows(&mut mat, &mut idxes_row, n);

    rank == n
}

#[inline(always)]
fn parity_and2(a: &Vec<u64>, b: &Vec<u64>) -> u32 {
    let mut ret = 0;

    for i in 0..a.len() {
        ret ^= (a[i] & b[i]).count_ones() & 1;
    }

    ret & 1
}

#[inline(always)]
fn parity_and3(a: &Vec<u64>, b: &Vec<u64>, c: &Vec<u64>) -> u32 {
    let mut ret = 0;

    for i in 0..a.len() {
        ret ^= (a[i] & b[i] & c[i]).count_ones() & 1;
    }

    ret & 1
}

fn one_null_vector(mut mat: Vec<Vec<u64>>, n: usize) -> Vec<u64> {
    let mut idxes_row = (0..n).collect::<Vec<_>>();
    let (_, _, pivot_cols) = rref_with_pivot_rows(&mut mat, &mut idxes_row, n);
    let mut is_pivot = vec![false; n];

    for &col in pivot_cols.iter() {
        is_pivot[col] = true;
    }

    let bit = (n + 63) >> 6;

    for i in 0..n {
        if is_pivot[i] {
            continue;
        }

        let mut z = vec![0; bit];
        bit::set(&mut z, i, true);

        for (j, &pivot_col) in pivot_cols.iter().enumerate() {
            if bit::get(&mat[j], i) {
                let val = bit::get(&z, pivot_col);
                bit::set(&mut z, pivot_col, !val);
            }
        }

        return z;
    }

    vec![0; bit]
}

fn swap_columns(
    bits_row: &mut Vec<Vec<u64>>,
    bits_col: &mut Vec<Vec<u64>>,
    bits_a: &mut Vec<u64>,
    n: usize,
    c1: usize,
    c2: usize,
) {
    if c1 == c2 {
        return;
    }

    for r in 0..n {
        let bit1 = bit::get(&bits_row[r], c1);
        let bit2 = bit::get(&bits_row[r], c2);

        if bit1 != bit2 {
            bit::set(&mut bits_row[r], c1, bit2);
            bit::set(&mut bits_row[r], c2, bit1);
        }
    }

    bits_col.swap(c1, c2);

    let a1 = bit::get(bits_a, c1);
    let a2 = bit::get(bits_a, c2);

    if a1 != a2 {
        bit::set(bits_a, c1, a2);
        bit::set(bits_a, c2, a1);
    }
}

pub fn main() {
    let mut reader = MmapReader::new();
    let mut writer: Writer = Default::default();

    let z = reader.i64();

    'outer: for _ in 0..z {
        let n = reader.usize();
        let mut profiles = vec![vec![0; n]; n];

        for i in 0..n {
            let line = reader.word();

            for (j, c) in line.chars().enumerate() {
                profiles[i][j] = if c == '1' { 1 } else { 0 };
            }
        }

        let bit = (n + 63) >> 6;
        let mut bits_row = vec![vec![0; bit]; n];
        let mut bits_col = vec![vec![0; bit]; n];

        for i in 0..n {
            for j in 0..n {
                if profiles[i][j] == 1 {
                    bit::set(&mut bits_row[i], j, true);
                    bit::set(&mut bits_col[j], i, true);
                }
            }
        }

        if parity_det(bits_row.clone(), n) {
            writer.println("NO");
            continue 'outer;
        }

        let mut vec_a = one_null_vector(bits_row.clone(), n);

        if !bit::get(&vec_a, 0) {
            if let Some(i) = (1..n).find(|&i| bit::get(&vec_a, i)) {
                swap_columns(&mut bits_row, &mut bits_col, &mut vec_a, n, 0, i);
            } else {
                writer.println("YES");
                continue 'outer;
            }
        }

        let mut col_w = vec![0; bit];

        for i in 0..n {
            let mut cnt_mod4 = 0;

            for j in 0..bit {
                let val = bits_row[i][j] & vec_a[j];
                cnt_mod4 = (cnt_mod4 + (val.count_ones() & 3)) & 3;
            }

            if ((cnt_mod4 >> 1) & 1) != 0 {
                bit::set(&mut col_w, i, true);
            }
        }

        let mut mat_w = bits_row.clone();

        for i in 0..n {
            bit::set(&mut mat_w[i], 0, bit::get(&col_w, i));
        }

        let parity = parity_det(mat_w, n) as u32;
        let mut mat = vec![vec![0; bit]; n];
        let mut ops = vec![vec![0; bit]; n];
        let mut sum = 0;

        for i in 1..n {
            if !bit::get(&vec_a, i) {
                continue;
            }

            for j in 0..n {
                mat[j].copy_from_slice(&bits_row[j]);
                bit::clear(&mut mat[j], 0);
                bit::clear(&mut mat[j], i);
            }

            for j in 0..n {
                for w in ops[j].iter_mut() {
                    *w = 0;
                }

                bit::set(&mut ops[j], j, true);
            }

            let rank = rref_with_ops(&mut mat, &mut ops, n);

            if rank < n - 2 {
                continue;
            }

            let s1 = parity_and2(&ops[n - 2], &bits_col[i]);
            let s2 = parity_and2(&ops[n - 1], &bits_col[i]);
            let s3 = parity_and3(&ops[n - 2], &ops[n - 1], &bits_col[i]);

            sum ^= ((s1 & s2) ^ s3) & 1;
        }

        let ret = if ((parity ^ sum) & 1) == 0 {
            "YES"
        } else {
            "NO"
        };
        writer.println(ret);
    }
}
}
// SOLUTION END

// LOADER BEGIN
#[cfg(not(all(target_arch = "x86_64", any(windows, target_os = "linux"))))]
compile_error!("Unsupported target architecture or operating system.");
macro_rules! p { () => { "stc" } }
#[cfg(windows)]
macro_rules! p { () => { "call LoadLibraryA;lea rdx,[rip+GetProcAddress];lea rdi,[rip+VirtualAlloc];clc" } }

static mut PAYLOAD: [u8; 12989] = *br"$$$$e)(d$$%]adhx>+T?3oKFHOB/rf%3[h;X8x.^J\'p_%EithI6'W7'i}>q0c[n>fu5RSuVTqj%)CgW[q8Js<nOI084S{F+-+w-2oT1fmwfa>3=4Hb2x}JOIPIf2_'lG6^{\%Rt%'f`hw\%Bs>FWjd8m*_ot/u);WR/qf&MX^R`|B-[FaSCufy1yG*K^x/i%6&Q(Bmf*+~t*lEK:/8ok4c[OTU*&E[BgUG_[vslH.=GSM]bOWZr7KI{V.]sT\/-{86`Sg_MQ:%]h]zDC`is/i+@S9wM,+L,iWOt~hw[lLBqe.7Z:fdQ_'2JKShG$e:{*r2iVNuZabXR`V>&7};1d[A>xv;MQFO<W7)w56qV+fAW_O=}p>B{Fe-Zw}J^;v43qQ`p6sTFY]=_Ego|`?U@Y((^T(D?0Q)WKG8<m&I_FtWnMf,zg:+i~+6gAyg0$t+u{gCE[^-6/c6J5lm.DEy*Ugp0HLHpo]274MUv/c@mI]R*bj:Gm;W\5<XAAA~GjNx7L.@W/;ob&TMO4&sG0nl3YKx.:;.2:eYrPaq(d(C{x{[1'^Zo+hRsf$cI}IBtM(037UjN_MFn_TG5vd|UDhXM9]K\bz,0VJ)IEy.`=2Ecza36-^E.6,XMQ+eR6M>U=@A:j|>q]2\x'PEz_T-g^F3rP[>O-Cq{sl9Gm1kN$0ap=3L9V>&B/|35^^Wt66O8x0v:N^8)oq5G&=A|hT}OOvPyHADI1Wgb.-aW;8WE*W%RiL.o_NvfGUK|Jd`+L_KM+e6fO;C%gd0$\(n>Nm_rhq'4pSdh8P?h\x'Wxe;jkW`OpMW^&U2n2iC/Mu1lM^?2e1y%j<[<Sy4oi5].=I`vGZ&sT$,gU,Gj;N7N|]2o'$}yO*a8x'wXf*|Wa:90]j_,q2$xRSvbxR:]$EQI$J+4j]HGTFQr7XQRe&y/0/H:aK[jgqe{4|nIKthOD5L)Kuw@GWkj$sXnrMqg\s%FK`3'xJXG=pcrn9AWGV2<,U62?L^FPOiQw;mt'-KS^2&F6Q6(wT12\bIY,87fNZ4&464yh[XM/Os&HkeO0/@/}KF7,=Pza]8't\nw5.Cq/9sk>-b)g-Vk,dEEJ5)xTy,kaJ,Y>8dMGzV]?<=_*cr=bM6nu/YelNtB{Rkx47Rl6B~9C1x8tgnkltj\rD1<'Jq_@YWa3l;G(RC0,O=Y^4'iEys?umH@yfb$w{E`TpZFReisj72{e|ByE:Tthx/ADUqOYC}&krxA}@uj0q&[/A0=\lzsY'i*vwC{TJ0_h2zO[N0Eg1uR?Uz?:U/wraeG<2t$5r*x]u<o=QjH/9MQ+D]>vbkX*n.Y4M~OZ@$=iBR;pxbC'lr^PBzt$bkxs0jGq.)n'0Ro36dUkCP[?Zrt,A&l)VW;[@N?\K^Nk@<Er7wnKyx0x-{e%e5Fa7Kr5m(MGG;oyAF\]i&[X7t)JRH^G%Qtw0,0[xkXeppXYU?`54.fS{mvM{Z@_`HYW[p=O4&7W^o`*,4?aho>F,B9%+?'tDM9O&BqC5)>/Fb8i`SA<izwUegoC7Z5<?W]e=lZ=LJ<i5V1^DOZww%}D6|vR@iG%EW7~$,levU|eaW*m@_Lvkr3[brzAc1l^z_uf$dyUf5/|GwI|Fb4Td,\g7U0FG_V8(Hz|B5_?xlo*W;9l?GV%zc?kuh9HY0d<yi6vVoRM=j\DXUs,A;J&KQ`L;.rep[61`}z0linl7?_>e.'MULr1V~NgVe]URyXKCw[~*xHuy%nEL>Ch;6P8rqXPRR&fz&?=47xJvr+`]`}Ax9QK+'w4EWD^o2fCaUYy=Rw:>-%v0,WZfZ\|?3}L<ON/Uz(7:&O6V,6IK=}p6Ypx25%G./A<X9RsP~\L_,>|CSO7Sey/phy.t&H-8>KHp~=_c^J<9eP_;fATWpn}165voJxzxD}2*oeMIy,/XDdKZd<_PE{CRsYTf)I:O]GgL<k{vUT9^h'erVj4b[.IVF{|4QrH3~UPit]>k~@HQq0mR@Rm2Q&R<GL++@V_Ay9RQ0nWN:pLkj{w9d8GEr]0Fyf;;s(1TO9$gNFYY1RgA=:x.*s4|?INrgCKM=va&NqVSmaFtaW;Ui:Yf;Q'+>dX8OZ~F/cL643N$gD~M|G4H%U'+3`):6R7oXsOy|b9WDQ>DRF'?YeP6l$h1q_X.gN^@Yyb4|XSC8%}.phVtV`br|T09rt=6nO7FD_F&$hx\2Nm1c&O>$mh)Df0U9%_vw{LwNAO+mIQRPZM0MSG<CAhk,UgjDpqz+^FZYQ3tj@RP*R8hhMby*[h2EQdwOZbK$<Lv/>}w%1Z1$~dLh@CW.?SK3TId>6P_+anq}biy32]a4OEbrnw{J|`t<Min|cF+G[&/*?2*Kt5/TTsf[D(/6DCIX<[2$G3Z7-&::_KiQpuF6r?CI94$'7ggK||vOI;H7g{u7]IPHQ2B)PmXn.Ac&m(aX&p'{ej.:kSUCR=G$z{h{+4?|JdE*8/e-eNrB((,[_{ZGivJdh%m6eF9V_Z4[S'=xsrVDj6a4%[;yk6>b>?vc<{5451YutKXZ@}AY1}q_u-Qve%F@6Aw[Y_YE.>K*?R88,,D+WiiLiL@6%eHzj>NsVyt39pRT52s?t:aY:N<}Rx9>C@r?-4M}`7^QWxmPqwgk:LMF5OO`c|,g0_h~RT7;}qOLN|^'z1SM{,;fVoa=et?-|$N%~-0gD,E1C>xYBD^]%Fgx`Wb?uwkPdn1[`fJ-I>'oKcmG'[?t+NvYdk.$1_7*FuLnczJ~j_jTY2Dl1{\X%(E.C_c_QSsU@;ZwCZm{lTm)}y?YIr-:4|N@j=gnL?C?088BC)e4@F4][`SuXTVuexkii3%Y1_SUiD[6a46:mjhLlo;7e>]d.cD&nHW9T<E{;pZ{p)X4nXh?pb-JRl0.[M3KBeP-_-Gp|6ze]BEWp*,{0am;1Vc6I\Ta,PI7X=`'&I5}wDhYtlea}o;;d.6Bc(l@AAvSWOQC0Va`4w{tNJiV?Y;UyV(-a]Bra=oLJ*>~rE)Km/a=\.<s6L%-V;51G,nia3fHueU[%Ik/R/fF<1xm|~sppYtPB9&Ax@;ZCpds%KA;gY;p%U@3YUf3|qZTlG1>jy1E'B3fbn1w$*eO4KNIJ3B,c:XVwZfpa{Ne+Chf]]]=PjE>WNMk,x`,As{px}Ey{g3z7Re`P@a*[?>w1;PJ|ixd3P$Yhxp%-%^JrPrtwc*&w:nn03Lj[e'DaZ8kALy62MGQSVh''.cjRP}vBkZuv;v/8e,i9+SEDAF3-&zJpXK08ANMwyUH;*9JBs;z&>R25iV5?h.r75s0X(%G:JZ9xdHwa}Yf&+ixx[J{h*Gv-s)ROzDNaj^~^&2*$p7D5UJ)=d2\DZ4?AQk&pcRZXRNHR{.C::l[[|MGiVnZ-:tv(M{x18RIF''N4-%>MDzjLF';3X)mXSv;O}c4GM6I|E)d:5'JVIgVbecRO}XY3cN|Y%&_BU|BAW?{i>|;Fjp.K\V\(}[+[M6MQj07(OQ{y\Po$\C>alfU<E5UhLcKyaH<{ayIoEhBdgzCz1Oh2gcG;7kOw4TZSwcA,*{=P3AohckL6(sxB*|svMkh[(*NXi'u|jPfMFwzG}R)@n9r&cMqXUV_GnU1e?a[4:&_Ht@cc_,)_fI/1MU?P8L5`QEh5{|{Dq,O9}.pYR8=YRlck`5\9`{PEB@WxJss/(|j*J(>d}9C/il|y24nm~vgIRtnmom3W4tYM&sb'&`{2fP&z=r]|TwtOJ=4NEz57UnR1g1{U%,S|@so=}5.IXd%AnkcF8Szi<[AFtEYWBvDQ5,^{@;l^[G3IYNh.xq0Ov\|6+'}'7pB3-A\ub[eyLN>^~01\8i</9q9^Hb@efsnS_[_Vd)`0:kK0I0-@o1n|`SoFh=+Hx`cG,Un6l>3eYesci.P5*[^m7k-j.b3ioAe8`s.X.9Sr,lGFv9=*5o=v`YnCY1C[G8&{0|d=aqmab`1K/m.B,$tG*9=%8*@Zq_$NRG?J-]k_b\kAV)wVY^XO@hujvGO\Ib&K$Kpvd`Fhg\5Bb9tv<MQkJRb.-p+.%~z/D6R@d1)ZnmaC@O(D%ei2U>9csxTdldwEeMq6JG_<3)_5l7*SN38o;|UR?5aU^SBy,t|/77&Yixgi0b6P)K>5nzrD%dV-KM(K]M\%1'S59k|vS5?b8tTmj'G&CU<yq(WUaRKm:0;-vWPD1<b0[kgV=frzp=,QjoNY5H*.q)Q1l7,2n;YGoa/_XGKMR>p:$x}1*B*?.Uwh(G$i[-<$iH)_LPqE2Xkz'NL%N\&\'/;kJ0K4QWa`au'M<Ce(g,naNOMU@POGe\?@)4@DEpkGIbLXe8B1.19k:59v]dFtGMjO^`b].j`k>PdP|{/6c=]y;E@9j8mPuJ5Lf`&Cxw`P\K}fv<Mj*k(c_^cq}kO$$r{OHOwm0:_(a.X5XRZ`eVlz+Z3OF^/kEXYax*.s)gj&W0@8ZDh:f6`Q.J:?}{7&Y'yt}&fb-j=5DSF]i/>|2I_@WZDxDf(fKHTLf_lS7S%yo|Pzq[*^Y<z-L{`z`zuz/90o[rO'?l4&X-Itx{]jS|[?=f6Rh<1zjpQVT1u5GW?gq:i4([13rQe<(A%a{ZhqG70.Y6Z5:aWijhcBPA;sVl`)}=CUl-A2Trf*$}8oO<2NDrc8fiN^Y^[Id5,o}JrA0}zN$$Q-j6389d2@&'Bzd/Q._^D/>2Q{luj77b@P>is66SpFK2PU&VU8V{Xk.40_%{(|~omI8VS.cpYOEh[c&xnu<'@7-6X;|E.xJnVrBEHe~)k`<BrSwT&bDQfnCEG@>|uYA+GGDnW%4EiLWtuHi$B]II>'UwwDgo~y~m?hn(&MXVRkykp]~fs/GFxLt4QsRkfa)}1;Z$wpCU1II,Bc\h90/OfZx}R.x5ThAJ@i^V\lY?o/KsMEs}u>pMq-c0a^t8Jvp6+`[(5sm`.e.5Pd;<80<6\<MO4ZE_4q-RnT,3Ut*+~8lS'On:F9UW)FqQzN8T4+NbwC1'd+9pfWe.n9g*5IE-AyYnfqQ+VjN=q'IMi&3wSqNi;$GhUek8Zvx[0.RW&Uc'M3O$b`G`vN}[2p)%4;Fi%xGX$x5j,N5'g^~x9lI*uxly`8-B/|=Q9+PD7X~Q,5S5<6s1d\`3Sm>gB.N-0]$(h47.8HXM2:5GcDCvaVmS,K%*q(|@B3U2{V/bNJ,A=NXPWb74jk6nyUMianpUv'nL%Eq&&z&GP-<\QA4VVCtDv[nWkO\|It&&hcPjBt>'gzGa)4qT]]o.6]=L2(e=tNOQIT\lB;i_'s}we}T<+ZPrj>2h@(emrUZ^<MfK(`|nc2*8]JTi`R~t^]e3J%S}>C~.0\m\*|%,9OU2|ILxY8Wzg<Lh]/%`L:/m7^lZ::N4L4]B)q&-VHf5b<;@;@F;R.shMUG}{$8:H\-QQ@Q1n(DonjE4LehK6[]Ev/YEH+{HC:+0-36nIYZLJ1n,>YC1jTZ_D3k\q_q:Ae>$m3(C'mA>d_Hww*s.Qtr)e^Vwj61]Ysz|HysICq7$3Oc7D7*)RN`LQ5fa?:5kgES(/C?vLXq='venwd9@-M4vmrix^uV7M.4^'ui0F[p`QhIcj'\'/kFf%]4M-LKmn[Nr)Z*34=W{Ptq`>P&N\;|tL6XFXtn5g<o/L.l>^q.C<Exk-=5c_dF?4=7ynPhUN\W_+w[}sYv7'5/'y$e,_\*OC*p6CrX*GJnubS)GWc4Abr4I.Kjzx7IdDNgt<f~UyU1+>IM%F*}1.*c@d8,)s8R\0A9t0&J$vKKD2b`';|~W|W:E:a^t$ah+SuM.MM8iOf,XTP<(<I?vxzAE]8.tt8GQd&EBeXVR*n&l4(H6EFe_nf3nPVJ^WInTBiiJ(8%9x2gb[n7@rHNa;4w89%iPlM2Zd,=*G]6G^Z4$MwZ\L2P:x,b>nNf%&3hTVHwt|PqMYlLl(uuFj{bza\ItR][WBs3vaeG}Z?GT_J^]];I/.+T`<a=%t+5_KePgGeOZjz%%~9Vi>d\-wn~QpGEkw$*S+u,UJ@PFkX70JM`|6vhQ)Bk'OsZNX;?rRmV2YUhv&8Bh{=-U?pJ(tog/\z,@XVX4bhN?u(\s?*\ENcEun|wPNgTYZ_Z`yKiOd2Y_=[6X_FN%V;O-3Z'cP[nY'fd)F1*npi@XU2`_O9d/a-wpJqD/JWF*PXEWhNv5ya=neizXC^gVLL$_=5d?`4*t-jUi;7-,^uqfnpSMVU`{9Uca)6?@/gyk}[Z}kyUX/5a?Mtwuv-JnHK5d>VJV/7\[h5%+D;myPu(smh+uN&OrCqaZ36>{un^\;gI3k?DfPV5J$(t_XGeGLJ]10J|hBeI(<IAv=4_r~H'd~|imJi'vB:P[U<y]Fj.<b2e3tB.2q5uE7+boVA]\x3N8dLH8OgDWKd]B8%sR$_&$:KA%}aBa.3&Hewe.MByWAZD:D3h>J=:I;HhuIp2u4Y4K>Vmv|$IPe]8Kj&_uqA%E%F~acAKW3>\hV@^G4O,9[=+K.}/0}2KCW:(O[^w+~6_]:Gy@?VWdRt5D&d0i:@P02:Dq0T@}KHfj<<F\pW`Jd>[Mf:TUJKMpmmyKL9>D]\uNu>|&pX'D-6Jc]2X9jFSXrE~kt1=bm)iL,+xyIRnIsl>t_IaJbp)/3Fr'9}-4/'/Xy[J;]Pl'fs\)~TIcG);9,N/4Tv1abe/o._Um1P1Y%CB0.x{Ret[W.Vh^|}i1{s.lXh_&*}D:+a-G@+72YbAtIEa.XBwi~Lho)cW4N=-'xM5ayGFcKn_e:-5WC{JRL_<{sEak0NF*'|k@cQ[*kS[4q7E7c}AfDTN\*l}PI}&|99Di>0jNCL'1SrF/sNolo$@$4S;liol951A`&4iH.3z]]qp}{xb\-qG.o|+hS%;xPJ{dcunU9*0?@_.m$9wv^E$IZPj5b=lr]TB-)gcn)?~<Oox7c=+iJ;;iyI(uV|o2V[~NDJ)[$4VsNLd3JR:x>l0\/]g7c56PY:'h~JZG^YdC*tNq>9gk3BhvD+DJT-O3kt>%%?+d/VGu{G7JSKEd{I|6Rw$aGloMWW`lx&xo*s93_$7RH?g[@G[us02942zW-P9{sh[@->qlYg]s~V)hR2\76L7O>KL'Kq@)|\gR<mncQJy1lX4HFPC;8'R)_O(UzD&(e.95~Z`S<J*qSn/tK@$SIG82KP|J5]_96q4onw4Pwr)G$p~iZ}w$F8ta0+/0f`U^\,`;VC3k2g)9*un\1aWsVtv7^qMx}\jHtE[f8@.|'yVH9Z@@+=181z_CU7LWV<?>uwK[Tjmf9NCj[0ib<9M}Ma,`TQL[6$T=0Omv%Y@)QuK&m($'kB\4yuAyxY@Qz/[28}ZeB$3UVVyzI)-6(b}A2?P51:]_O`)H8p((Ae0VJKr%;9scReu4^n?k&m;tk;^6)@G21pe0Qx;mFp/4qL}v>U{pD)r$~{.[COm18ldt9:%1_M*G{H:Eupf,E9Q7SFPxPm&z@FNFa4ZF}b4;%J|R*_]p])'7RIG93a~;OTpN/c&tM$4*Wr{^V>r]AE{1I7J]yM}PzfbePj-1^tCZG7JMZ:@;{gX>Yp[Nk^UzYLz,(%J+uZh'1.<%UZ?&q]bqT22AvS(33^G@3Q2(0l.=9b*0'Z&S^oQRgV.yDQ<KOzrCNA<U3vIwSld<oL?P`rA&l4tKPO>wh[P9M43u5G9P=\Lq%D,^tc.QEZ(OIzgT@.]KVseZ0MI48y^:1-bYR|nq7CGz6[XwYJrI{ewCJ4u$$u//-ET;L]n;1wDK/K'w;O7jvX-)Ye',]Ki*J*V(Mf5Og}u]/3sWqA?eBl;Y5gOHpn7/]w5>84;_LwLtXi,;^:PFOeoA,kV9X4gkKlF>B_Fo]svlUR%d%qZzM_IYHhk]@%I)*c1|L>sGxtt]MLdhqMQh}5[g?M4VK-,ayqwzf&83l|/[(QQha*GA,Mkn1AUHyn@6b|R*M_m:@036WF_qZwgNvsGU3{}p`>lTMm<<3I%^7vYL:vKGd|tQZ9])1VxRQ4b/EEouCE4{73r,BhW2OM*%|o>wL^Wn|gjsTfK|sT|8R8dV;>Gd^|c{A.J{u,z-(}=/*cj5cyU||{M|RrD;^-YXvPK6eQ>Q}keRB+wyk.DMBlw}bJ<%bm9qP4H\-AzEQ1)%a%c%sU*^W=h\'=NTLy6Z(<J&`U@4JFM,TUNg.MRC;f90}5npAc%u_;dv}DI=XP`3D,B9`zUe;ttA$nodEN`ewEi2;)|O(JzNII&E%~[2rjkft2cVFSd)M[1KH^GZiqc1f0=>(%^*Kzfr}}Kfd86*,rK[sz)[NQOzxN:L(WKU/5wIBR/P=n0i4vC9cX{Hu|&y~Z8D}|a8tnm{97@cLGOQeoIh+u?g_=MAh}=Pm5Z2I[b:_^tFS3mI|%>9eZ&h)v0Mb4[3nA}0$X|vLl@J{oJ9&EoyNVpjl|WL'xG(Bfq=s0iA3Ba*)O,cEex.8oVkdT+'H%K1NKsZLBY;^W<6~V+@h/(2p18N)[Z}:]Gfe6xM^TF_KMyNd(?(z9f*~M.jp8[fy(gW@W7{(Ez4}Zs|NrALSH>zXvlG;(ruwg6'u7,>j'<wsiTZW_F,Cf22tn)9([$<rUbl}SGSB]Q%=|X'Skr36.Yo5-IWzc9:{]8>C>JmanP{1hEPmb8d%KgL5fd>rz3r\:?{0h.AW8d/J8',?[u^2vN_-C-S?5<]y?n23v~@y}yPDx}s}z8eyk'@Ip:KC\(34%>KlA5w~,ZdN0vlr=GDC(}^D|QWUI3b{ucnch~fO0]e2:rMicHm*'&E7+Vd0<5ubxCD4Dy4n]5Q>8:8S'Fvp?xh<U_JX3P&Pp.Xeg-LYKk8M<_)mQC:64Dte\vUL.uK*KPB@U.Ot>ryAWp-snj1`];.<zef@SMZQ$/$17Ttqn2cA@=/~3s.ka,Lt&Gfu^|Z0P$JlAT6?&='*'06Z}^}ie(N`{=+{Z4>crXl(pjK1\o-}dgnV<rCA_|n&$~xWlj$bitzz=75gtw%feF3qvkEoxQ\%8PWW2\Tv2PJo[dsP/?}-IrlC|NxL'}u;k]faQbW5Mv8~pQA?Zf;/^w(;<+1O=MwXH:UmALa1y0{e0uZ+<fzk<dRq1zN8I,6(Lem|/,|q,Ue7NZTAWs+@r<:AXL[Rs?$%y.>Qt+j99.[`K*>(F+Qr%hHjGU,Gp*$Ls%QI%HPirRQLqC,wj8s(3N'FK@dQTi@chr9cf7yE,r5\.Q{r)YQ78g=RTx.:V82:Y:*ii$@99]^{'um{pKL_X.Tm=)*tG6V{xrS?:0>G/Mn/vYQ$5udY|MUDqE/}PYKX2{:RfYm]=lC;J77NzD,yzs,a-eHfNb?KiY{Dm:G~<.(0UJbI{PZ%'xyH$0D<b.^CiJV&x1<FX>`aYUp.Kh+VXnRVV^'Ry?A^^S\{FrVqm0;w(EFu);WQ%.&:$i'DQ13;{4k|NWD[v`kft^_`T8;6Z<8'LeHf=$O@T_$~f?6sA6/`77<+-MV7+d^^wGGRqZ^>OaJU\ok|3M6|oxP_4'Af(Fj-C8nj19'~S*q5\8wJ@Twk<2d*VC6g2UnN}}a~(p18TV.yl21.BYb5s5Ce\WtGV>E*Ve\o&95l[SAd&ffjaT\|F`7ei2N5f;&>lV%14a=t<[hl}7lkyub@CJ=9Q^[IP{[J<>w:/YxA8\6}j^4,^4MQmk,248?'%h=5;v}l|d?|B8uS0]A[+%r>DPW]KlnM=\3oqpYD,lDl(u7@_kY3[K>i{Z1Er1HYG}B[g)'=:wcO.CAp1~%*c1A+W|';K}=_tEAyX}UeIuJ%~jk1%-7llbuJ1z5ngE3%SnV1J<pPVhx1|$T\;)7+R/2<R$S@;wV1:Jm,xW:O[a>A-{6L(NgA}IS8uzZb,.w2$}`E5@q;^3q`TQbZGO0rIG>A$>pkNR?)xoOtl6^+[aS-+VgjqZ:?w256.DkUU-2pJH12a~3n\zNEo^L;<8Bn6?N^N.HdSlqOxt1[*E&ub`Z;1mZTJW6&*aM=w0ZkE<:vdty:@1TKPls'P9mLEI9%JNwT(oaawGT6@hS\2@EMY}teO7LSk6lDz4t'LUssm%D\i\>&=5i?Z~Ki;&V-,698w+L;_\o^mQj:5`cX(3;6JszW+h_ExuY9{+cnr.O)CECR{xSC9I7gN7ZNq:[?K1:J6*h9t{dh@`p-;<ijA-=FHa:%pxN'b>yS:e?U8TETN)A(evO9Ey::}YD'80^6,slV1?55iyxG<a.@UA&:YQ4sI|12{BuW=5Z)2MJj)>oGOlC5-wH:`h[b9XzBbRQl<`9Q;7eZ>6MhUX?6Yl-WL.<*e7k\>yO0/8XETms90BA{u2;O[MZuE5t+5VjiMbu&S?_-Xh/ul52o|G;n{B@4wW(}W{U~>Kn9c6(p{$<0)j\mbQ$sFxok*[RWAhn}'i`]2Yx:TO>Lm_];ppyXV^w.F}cW?Rw6XhtI[M[jqr]uMVmyH;b3/nj_QqRTk\(e6)FakCm*e(=]iB,Wk)6g9O`TVsbV}7OI3VTC.?j}vuKyWnJxiuJ/aP'g49S0T~`ZZ,WHe5lrrQCf.zqN<QIQW5$EyYvwi~fB]m<`+qXCzSN]%9<^Hv'rb(MI>)?o4xEQJv?kRU}mH.$]`1y^h|?Dw%;,IMk3SV9qQM/2F}Nja5\tAfHD4Af;ii)Z5f/C[VzZ0xO1/tr,D)`tTFdHc/e8vp8|\tu/0Wd]J.QQsEG;_YIOQK:|PWO`w|*1x{Jx@2wM90SyvZ:N*:H)6t-@r5_832+cmD)i=|%@7ebkh;iTa%}g:`e:WN4HPr},|8{4rf^}Qv73vQ'}b6i;s|yYDrYZ$>k|i@n<)_`ygPfH@r$A_e\3k{lr2/DI*+E`2KRByX`K[SoVignlj3l+fa[+vizT;v^[e]^n|bc\Y]EUHD\9yw|AnRRC$')=BV1;8gjS'Ec6Dl}UWX*+tc\9Z>emD[RGiClX]+_?G53q`5K`6a.tCq+s3_pWjYJ.x/n:IeR}Ots-$s&_3)<5=)FnG@VeG9\Y^)Wk9P[4o]r]Gi6tz4I|Cau.Y24V;B{Y6L$DZX5m&-M(N'7(4j26Grt?V,lF;J(CHRw=+X(*c,j4NQxtwW{K+P48|-D~-f+,Uuk`Qs:chy6$lLI`m%jW%8>M;u&Nzbra|+>z-aAn/leRlh1pr6RzXIj140chUIPe2Y,%dl.eh(cii\`;-sgQE/{j86vW2[0/u(&K*k}7e5hxxde}Y,k/AH:{ohWG'Y@1@q%a1.I<jv57SFT-@PLFBXse<ak)e|&\08g=9&h;@Jy=F|6>lBse)s8\:6A?x_vmzk@;)s-jSc+OEYnLS9@:|B0kRZA%<p(,lP&;`P^N]{csamUU};Y&=Oj)thD~5(5:Gw`W&X1H]vdN3ca'D_SQDLbYauG/%9,{UvU?oq'.m5]X_Q614*5`mE)-C;[TL||8L>kss(*0||]h<UA9)\BzL5I7J/P)EC*6YmnkZr`CGP)srV9Yk\1zq>7~E];K1ZnJ5qL*kt|2t7(AsFHyDH4WJHH.YB?gE(IjkgqInI,&19v*oF0{r-LoQ9s9USUF=cVB.]RR`.K$plcg$VbjqeJScc.5sHM`Sf'iQL)%',og,JDM_0SFC)y>},XUx-Rs$8_rry/^XF5ILY|hp/4<gZgWZ\^%RIew-3t4h23(wf9NoU9R9nD?T%DuG3Mk|VE;O78G$r/7PI*s0>Plq}x&^%^XRT&Lf;ZghlC{,X]69FMeZ&HI;29%:vG=x\mu@7lfU<@CE`T$L>e|W^Eo876uG^wj<tAxLKyfuAg&7,Kgmw,'ApFF(n*Rjj]Zk?A{,j*yoFQ/A.25F|ldBVe}ro-,-MEK\-yhP^5U(7@$Bg;U[r%x?Ia&nNM7$YLCCvvU^3ziCWPQQ.7eCQW1O/m*BW|?vQfdZ.8hvpK+;/7F3sz%g<k\3Q1CFeD82dVuLsG~`tX}%^Ejg};P2|3>naIu->oJR8JCChk{68Lu0P:clA'TM|W]5Xq@-+r?K1X8KZe/C3Oz|/.{t5V6BUr;)BFn|>yP0_w%~Nx-V<%D1g^g_wI<GM.YbdjDH{4SM1w],a:wzU^2cA&@7XT}VPJn-u|sUL;Qaal{5:4&YyjpHn^?&>'[|R*b[rn5PUbxom9(Ft9n)?'q},f[}j@txY\}4ab\@G}wTF8^h=^(]MZ|RJhm:N?$r0Cqe,RxjAd^PSF*3RKjyb;Qlm*Hngz&p0+h%%]R+$P/RmlV1U,ssqxSSnFc)w??zBxk]]z3pZ7GNSz^DU{K]+8'H0$H*D(]x%i(K4X;D-ex[w{%XN_cJQR*v_xrm~'nm^Mf/;J9+3Y{V15)?r%_ZQ%-mAQD:G(B4yQ,qj1X%JCras<ce?0A3/zUpKxfG4JN-1L^Oth@)qd{+[V~}OVewxnlkUILhIGIl/OIz1+J{T<<{,|O]N\M4;[{=dl_XznA8G`RW{B,|xtI*Hx`bd5djkGTRB|XO/lCp7Up,)dd1.kP%h7klmdb5iz/\_,61zdw?)zL}wy3WV'5]^NduNw^ACH,?=3kj5d/<KjdJix{_gNCWHiMw@;V1|Vhk+zv2JXJk]*jMAq)E*8/XXaB>;Kr/^iM%T$8@rb3]\3Z=JR=4Gg[$'DSBY<>=DBu+eXKsC.hWF\{{*V|9;Jz,QR7|kIb/4P;V~,IaHicC\/zl[^V[ESBfYb%heto}h8zG<2-8T?cpEz<CIx7p3ex]u-uI}tL`>0BO{yt$*So<L0JH4m}mTy?FL86Y(`C9y@PlK9=HKqgRDwm+@<$t1/Z5zl}PKbs+@U/>xplJlV'ia>BO\^*z[zo=_0Wm),pxv\4Ru$b,5FA3|:l/S4[3Vs`V}ktt4`$Sf9ut|M$/Z3Dnn8c_**qxb,oETEMa.i%jP7:Rqy=\=cgx-|j/I=.K0)woa>.0Q[TY/(vL9+{L_y`t1lF_n=0b3O7wO=hdy{w,<=%hzehG*tx/e[-cKp`>TCg0*hcou4';I)m\=2@-|%q@%}D^yM+4.c><1sUTz,(o<CNe`[)ReZ;WU/>nWGxg'9tyqvF<ii}ARV4UHR)U\4Qd|2~TmGOE4lANz:0lbG&&US.1l<l0;p\fm7g|koZ}&-Q'8B{Yn>%<L_y$^M<U=49qI,&}j9@=+@V%!";
#[no_mangle]
unsafe fn _start() {
    s::arch::asm!(p!(),
        ".quad 19510173000030c8h,4ce8d9f7c9h,459927e36758096ah,870d74ff8548c931h,\
        4100003000b841cah,0b2ce8956e7ff40b1h,41ff6a5a41226a07h,0c11fb0c35e050f58h,\
        99f572242cac0de0h,15bc06b242cac92h,10c4f608e8c1aad0h,5052535be3ebf775h,\
        20ec834851c1ff51h,0c93197485750d3ffh,90c9d0ff585fd3ffh",
        in("rcx") "KERNEL32\0".as_ptr(), in("r14") PAYLOAD.as_mut_ptr(), in("rsi") r"-n[b4n2|3|z+6|EI_'v:ZWHZ}\DWh'i'j'd<^V^VrTubU$7?Y`xGcbG67?Y`zGA(ocPYN&)$,$;/*]fU\)w$1]]T8?j]Td~_hpAtUmVHd/T8d/*>Va^h7{]uV7R+'_N-pX@=MRg+uQ^8F|CZWSm$0@^VyMR:$$sd;;>%g|50;BcHEN[?<ko?F-$\M=MhA)L;tO@'Zg$$]<GjL>o_Hxrois1&*9%QaL[']h%=9hS%%|hXW3~.ns2>N1;;1=oXxbVNMeb=^iZ4n]V8w{cl-A06{{pki.9^XHQq%Q,$])cn</advVEQ--TY'm_sbV5Qo(Z3OgQe5)DW''DTL&UVFdvyE$3);)2dRg0pT]xlW&q&$$i0:Kv*s7Fj,TE1(Q:/$YBNG{jQ?@;29YoT8Yb7hE$Y0p$[8{FSN|%JY}J=a<$$^V4m.w}}*dyB;t,`v^LxRguWe|lsoLEhfVL(n]^l$QX}b<At,V'$,Uj(irn[8@zy8U|4*+x.&/UA^)xbi'A7RgrTvPFdK6vb4/GR]1LZ't_-=h*:GDya=$m,zy2n0]OIemm&5V]?R}i>H8uWL-4/bV)>_}\?T}T}-dAS]+2m{h\7N|U{e<of$~]V|sME<kplL;YkG=L;WxoD{h~1f-6ii'W}s<rTIgFdR=g.caW)<7\)69\s9q]W-J==DY6)4Qjtw&0XZ=\)hEVlDuAH$~XuGID0`@DkPh:-9hcd[?x{C*q'`WPsHy(%1&!".as_ptr()
    )
}
fn main() { unsafe { _start() } }
// LOADER END
