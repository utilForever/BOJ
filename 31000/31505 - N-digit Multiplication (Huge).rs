// Generated with https://github.com/boj-rs/basm-rs
// Learn rust (https://doc.rust-lang.org/book/) and get high performance out of the box!

#![crate_type = "cdylib"] // On Windows, omit this or pass '--crate-type=bin' to rustc to avoid DLL creation.
#![cfg_attr(not(windows), no_std)]#![allow(unused)]#[no_link]extern crate std as s;

// SOLUTION BEGIN
#[cfg(any())] mod solution {
use basm::platform::io::{Print, Reader, ReaderTrait, Writer};
use basm::math::ntt::polymul_u64;
use alloc::vec;
use alloc::vec::Vec;
use alloc::string::String;
use crate::basm::utils::F64Ops;

pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();

    let mut a = String::new();
    let mut b = String::new();

    let n = reader.i64();
    reader.word_to_string(&mut a);
    reader.word_to_string(&mut b);

    if a == "!" || b == "!" {
        writer.println("!");
        return;
    }

    let mut is_minus = false;

    if n > 0 && a.chars().next().unwrap() == '~' {
        is_minus ^= true;
        a.remove(0);
    }

    if n > 0 && b.chars().next().unwrap() == '~' {
        is_minus ^= true;
        b.remove(0);
    }

    let mut a_converted = Vec::new();
    let mut b_converted = Vec::new();

    for c in a.chars() {
        a_converted.push(c as u64 - '!' as u64);
    }

    for c in b.chars() {
        b_converted.push(c as u64 - '!' as u64);
    }

    let len_a = a_converted.len();
    let len_b = b_converted.len();
    let conv = polymul_u64(&a_converted, &b_converted, 0);

    let mut idx = 0;
    let mut ret = vec![0i64; 10_000_001];

    for i in (0..=len_a + len_b - 2).rev() {
        ret[idx] = conv[i] as i64;
        idx += 1;
    }

    if n > 0 {
        let mut carry = 0;
        let mut idx = 10_000_000;

        for i in 0..=10_000_000 {
            let val = ret[i] as i64 + carry;
            ret[i] = val % n;
            carry = val / n;
        }

        while idx >= 0 {
            if ret[idx as usize] != 0 {
                break;
            }

            idx -= 1;
        }

        if idx == -1 {
            writer.println("!");
        } else {
            if is_minus {
                writer.print("~");
            }

            for i in (0..=idx).rev() {
                writer.print((ret[i as usize] as u8 + b'!') as char);
            }

            writer.println("");
        }
    } else {
        let mut carry = 0;
        let mut idx = 10_000_000;

        for i in 0..=10_000_000 {
            let mut val = ret[i] as i64 + carry;

            if val < 0 {
                let val_new = (val.abs() as f64 / n.abs() as f64).ceil() as i64;

                carry = val_new;
                val += val_new * n.abs();

                ret[i] = val % n.abs();
                carry += -(val / n.abs());
            } else {
                ret[i] = val % n.abs();
                carry = -(val / n.abs());
            }
        }

        while idx >= 0 {
            if ret[idx as usize] != 0 {
                break;
            }

            idx -= 1;
        }

        if idx == -1 {
            writer.println("!");
        } else {
            if is_minus {
                writer.print("~");
            }

            for i in (0..=idx).rev() {
                writer.print((ret[i as usize] as u8 + b'!') as char);
            }

            writer.println("");
        }
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

static mut PAYLOAD: [u8; 25695] = *br"%Q$$|4s{$$MhbdxY_D}@nCDgI&C=%L<GFIZbPA@c6n/.TN<GXJj>0Ud[m9yj%w?j,feVY?8-YeI:VR2}~dUh)DU$_9XJac%`pvlzwy/2%r@7)]aFjuXviG,TLYXKky6l.3k;N<:ZEX:AXe1WZq4cEa3y?rq9$>Ryrz.4zg]rHJ/9i(xbcOH1HM.J*sG3IUUbe4XTIxuml<O_sBSOfa@6e'|oTz-|H?a|t,kb<71A`7=G.t'o/;.Hzh';BG\LgtLi:P%h70VXU'AKH(z([AMR69E=pT^PTW2@ab5V$^+8q73>{r\H3haGYOvZM)@X\HPL]tf.m^u9dQm/E@,cZ@$7.>6_cWya'v5lvhSzm-r]?fhOk\2Fd,\VIal9TNO=yTrtG**s6L|(CP3[,ywi:.g9KFa+{J{W4ZAa8Zo6C}}o)=K.DOfzrbcvw}yEM/i%[Wi*[OH3Lm~Y3O.KV|KFUd{(teI_=Amvfk}S0?rJYQbIW)kCYkMu5B4ib3zI*'72hDK;&).ghE60egJqS<+scOEa.jxZ9dZ[)/Klx&hq;uZQBQQY[4yJr`Tm+{UC^RGGwC9@S=o8SgnWUR*ldi~3lU('|[Fsowx/Sg%`Srip>C44R'6[Y+%Y]oN$-,`>3$'UyF8KR+$|%}D|qKrrJZbL,,kdJM%DUsILy7NE,SoQ:`amQ;K8f8F79L3@cw[F1\kBLViCpdX6MAM7FUCfl8CTNiq03j=e\vT=~3pw\RMcKeS+>VqszA>A'V1Lsu}|he[-3O'hiO|{A3j&bmEn;Ug@Mf4}'L`<0R~iXD$/0kKB:T*BHA[hQ`M>V*=,{Pg'w*x|}ucCLp}xPHd$=ddkbs%Y2<?6%MRB\QTS<p&^m]5&^bHuzaGgCi6{(VY'pOXS2|%NK2S@&t<lJ*Bn0}n&E8%jrmZY>'Sw0`g_A$lM1K~-A[Z'\rff/f]}n_&=vP+*0{@]$sSH<[6M=Kgtro]()PUfvsv5Os\9N.&?>%>oQKpWaj3X'Sybbh.lCDp=n/wz0{)PSLw%P'ba8kLJiUaM5]mdU]E{$?{7@n6]A[Rr,.{B1lGi5;j_6?zvLmI_KVCqV3C0.E[V8HRUUL75yaLl=eJ'd2lF\XF/LMMWmi/d|bN)h`~0(}GaqK+slX&KU%-X-ZQ4|_1xgz:R1X5AygD(y|j5cO'nWBmHHs}Q,E+sq7z.Pfbac0h?'T47ibbSfomgM(Oz*j]/A*3Oay`v=sl-)@_sZ7tT-J<p}H\/Fz6JsUwGQ9XpXd\C1uD2Bx=VWvUf~V*f,Uh&x&gT?G~SNO9ki1pA]4_S3NvQ8RAq~7D=SC_UPk)BcX5ZOvbguZ`HJutmA2mRR[n+vulH(6TQmpI<Fd{r{VT(CY5$|yNuP7t&P^AGY_bg7kd?]OqP,bK<?6\7X)p3WtP;m(o9qRpQY86u[S&gX,\i5tkRqj$d?XOL?kV|8gen7OD\sE|RL6>]<$=U'c.iF5l/@naQNq0LfD{A-s)F~KA6c@`}l,sFczJd)d0]@mH_05*ln+I3r[=k5iz_U*uljRO)rB=tumB-R\bpm>'&I\1{9o(ttS,BWxn]BT%}g<P;BLz4H}o[[%v;HmYg.{Pt*b|-_v/\tUN>Al&Teiqpexx{@-g:c<v{=pZUPr4LaGaS8FPs~uzg/9OF.sH`\2?X28iuWxFL?ik&:.yP+w3U,zmO2Ju,zGMwPU==cB==Q|bzsaWzHOoKPn@aA94nE7<w$T0twh5O-%pBo3GxK;g0|e.KFFOIEsf.zEdn'm[HE.$_:5-qf.*ZBqN9^Z`qFS5&0]gG?L10HcV,<9[$$|dpz{2mtX`Y]BBrQ3y^({X$umU'FyoN$81`3n_/wn|B*n3CqW8J$yow%H7X/D1ogxv&W0*4zPUR{J6VqV_RfTFa|3<9=a=t;4[226_6D_jWegYJh+dXZ68]@JaN%ZcBYtF(p@r\f:7}.FWiL>3ysz0`LO|Dc*,]iEtTSlIU]^IE1,`g>Y68,<d[U.:{yFdx,ym8,5^XKh5GIV?mrV&55ZQG.r/E32`Zdzxs?deRZ5_F~a+|g,`m~v>;{ZxFUW]1*Uk.oX0Q1]=E{NU&c>E$C`XeW'9C5D,Ef.Q(-V>cqvNZ-4hPDU4Rl0az8F)_F:IJ;*@^@jtpn5)p2/LiUX9ECp*@;dmiU-E=+z64.}e@@Hw&gZt8wVn,I/;C&F@/_+&z~5V\|q/`,=kJebQ?/d^,*}@L2h3@%f5RQ[P+UgMhZ)S6F_wC,U@3>Qx%?[~:Jroux72<_tjte85X7(MiWgF9mHVR5N2cn-b%<Go2YKGw/NNs|&(,@wpS{NzLlaPjm`RJ)\|fz6%cj`>'M86,VlrqFy1<+lgkpyt?;FY^q4^'twdXSr$_fTWSDVQ\2H{v9c=Z(rG5?$Av[pXY3PP5k2=iCv2xNR]>phH{>m$Y8N$LQ-F@M`.G7uy66Z{1S0Og>)3TaeUQeHQoDskE,4fF`k$SmjvaUOUmQDB^F88_Ej5?KORw}3FL.czC<d3&B'/Ck4xpn`r>+Im3B<lr@V|Y/K1I;qjKKu`fBumow_rMHwd.QC@,=\:ZUD^+Z4%j-Bp$n'VFT|C,n}Q{pU?,mhw=2&4PL*oAcnQGvmnz;:`&Fvv$p,Ubd^\Mxk3&wQp4kL>rrF?16H0%p'nmRt)1cw&^@Jyxv\v]Rqgk^DV}]sg0'Y'O+Zny*Cf=wo|DpjsEayNO&U[]@|{]C&Nu1uf](xbhUvKeq<fZJ8&6SxCLG(|MC-e{3W@^-z9ghD&bF<&6-1H$X[w,3vP7RH[6;ePahrj<\FP,FDj-0L-[Bena=ly5a,Lak@,A[j+6]_Urg5($p?%*[3J4S@t<>X'2M]tOHm*WG{2CU,o_7ou}^\_,hshK,%^CRUU^?VzzZ2*_WU&Z|-NGfjZH,K77'=mp-o@9yD2|)cSyy}Z(CQVRfxaL>?'?u)eK1vdAmTj,mMcIzrY^:}{B76n7qH2R9t5H0@uW|wEdgy%fP'>;y@ytDvpMtbqbj{b\%1n<pjud.)`skS&:oCN,ySohMb)mn?^TM((:.6CJ)H$t(8?&$(aR(|F{EIv}7qY+8qhz?QioJTPb=3.a=9DD1J{o.bTI^f-1&BS.59+>eBNQ{FkIv2YYw=M?ht>N+,3J+g+$3W8/,3oZWIURjpJn_sGTOFKR`eHh.{5K3k;'p&RIa{AsGr4mXoMbDQ<0?<TURM)XsMKWZfid?iJffjju_gzA{16wd0DY,O`K)ux:w})\mTTH&JJz+I?y<d+P~Iz6W5HI0J61y+@rUQ\S+[q.Pvd)],JIo_awnnnp$TsB,vAn|Q]pv8]n{NoC99~Uo%R6)T$:W3MHX5TENtwHTt.7Xr~&sO0<e;X=BOx'wuzF51b<l0$(%g/@{J~7T*UsQ5W`qd[u)Ypk,s^+g1/MjsnSgw<w1p]EF4oJgnN5GA`Y`Qs}Z1tAxZ?x@\Z.5QcxzA'rm@4flC3v$U4_{qF:N7$/R/Rg62nonU1ioS`x/Hw<}oaJuz4]BB:'b@{TTqY$_n2btUCIaY@p~sU5fx]Fd@}elC0oA&kt8=vuwWx0>'[8I29q]03@+}=-(LI|PS9Iqy{0+yE+sUsjf;lb:>HC(zvES-NSwMCA*(qf;wF=IEr.&q8KQe?u@&q.m?Hjo)3,u&[ooD/_~reTJN}gv|>(EjWt%h(A^Q-j3;8[hOyb>s?1Es8oH'&ga/r|s-F5nn*7t5>O)8uNr\MXC01P9yS-%<:8y6F=,R@LIKnpN(H3cM7oT}[q*N*e{?TP)bD[(nM}l&=3/4GZAQp%e:jLU&]:en3M*j;EwK.qgZ8u>d[g4)R_s,]|@Cz6C^%KWjDUBucGELmcOg%'9E;NuJ2BiqH21y-r=rBP`_rCO%9AITfVnihU;^u=pZD^K9;}v7nqKHcR$v3*rR+\Z|w2+cj/iH<x&{wwax15qMJirABS*B>kkrf*Vqp1^3M3DN{s{W0{)TKzef+EmACxQV/581X_kw%[yu(i?$J7vHF5usmB5d~;bA_j%d;W\(2:CYO*[Gu3XniCgz55]m7af3m`-c5>$vu8^hZmd8FSFbuN~dCCk&k98Y]&ZqA%C}M{0(GO&F*/IkplvF`o3/S=E3rs^5\SCUbW_]<3f-.iEKZ8}v@|^5yPtH'fuC:q~fMbWYM1hxyzHvd{5WJvD_bvg'{C<Rz9.M+ZUH\A+$R?E<v,d;s<^pXeiYV46K<w:[=`j,k_&bU&tnn<Wlt@wbcI0JA.K6=cg;xx0J8+A=_MOp71.pM&dZ+gjtnvXYAT])8QrkOF2?<AY12Z%9B|8vrR+TAIi/_2?E9D<Dj=p6meog(J_U7VU(gqI9.:1kX8kdK^0D7FfrbJM6DubP.UzJ&sz&Y07Ndn&Z*qUjN_t|\79'Q<58c%h;'&&\GL[LP7.-JMDZi(oQ3mdtgme.cB1NRkxgX]-B9f~D2eYpf^,bWZIY)WWmB?M$:x,>uk'^f<[h)D`Q,OYUFpah^)]]GAh;*7kN(J}|p&%`\VSX3_9ho{dwv%aU':\a<U{E{zceZ\h[dHsJ9_\]1=26ro8S|Ns/tch-189xAFVe%lAT1q.JN{]QY3PPA>0K<ho(A4jD~s456f+|`u/5SC&n7$/ci3SBLU4/dXU}}`S4+)},:o5Z9c(ZyMedDu?V{0M@6dr4TnzGaP,CL{WXZ:M{k2ddl24O=]-2cpuk3OKu>C)cFI?(l=a4x}Q-.<oyh*yoyQwlahk`8y62G0i)9RV8;nfNtRr[$]BlulLD&Zq\7sdzQOY`7h$K(TsCGzWOmf>+j?3*i[d.nEX)G<-W=Yalv'<D-_paJW|MK4X4H'AM(18{Pq+TYnn<`_at&r*ghm`EZG,SoMxxXg(K(ila={W^3r^X3M<DZ?c>@&zX4>yjGSB,{1NZ6x=%HmDS^FVn9SR7LS,FZNO$}k8p<{{0E5@iej8pqlx(8]Qv,2Z':[Na4y`Brvr-\:+bX)[jYI/sv:wNFgn{&JT-u\XjI;2:L[_='xadFh@6N-7q8XdF}{GB{z*Fw8L)=IU.ODur.c*YT2>:;s|}}hQgcwt&1qt,j,a5.(X<c&lDP}936P~'q5J3A0=jo.5wqA5yII:p;^VUs|-Aj{A%)<+M<D9?H/z@9lnyZTC_9AiP'r64ku@o5@4SOY59YO6V7xd3tBP|jW;v~ta90rL'nn*Yt@}Z[mC$EW{Ou+qwDtQdg5sO2x;H[JI+a9bMaAiyhQ/pj}Zr2Z7ZY\8b-s_m~ArT^0T1mqFkM2hBM/_'8.Shts-V|@FOmubd@;zf[l]aL'I.m%gm2CJ>B]:}86'Sm\M1m9QX[w|A]Ahi|s+D.m[C3vh/o[EZ~xV@vrntjPx6u0YC@5vw,9$v,NWhhF=@j.u@Z_V8UPk0_O+XX8PH=ag*D]-j4HD.Xo%=NTQzGrVSpzO19ik8Os7=qXDq++Qx);l4Vlb;h.d@2w}r)UWa+d[n:X%zq`-I3h;f$]|D9rtrdHc)myY31F`8:DFww'\v|aDqH<dRW&mO8)dbLdAM+(-d[];y>Rm|C:.F>:qP^&00@B*Ol_+`0*iwcJs&_i[\*bJI_4.tAS3NSs8*5<V9SJYdO_J9r_$RFha(qC_rv&t'BxO\DbX}[48*^-77|RVh$%LVw7Z5faA1DiB-2S<\*nXom8A]3`WZ;6'W$B>%hKd3-[iMY3|w%Q@wq`$;q\Td?/uB_V._p./hm9(jF+EeV}<pUKZ;t0)/Nq;aponTC`Dw&zMz5OQ;<fW%`^t(h88NP\wDNL6fO*6VD|H-*Ll_{v8oa>6G,p^Ipll[)(L|9h0&geQ*kk9NZ9W`--XK$DOzTf}v4@ds6{u-ljS7L9S;C}Q16{mgnQ<-f[Dmh?j,Kli&LjN9FEzXXz2WuHmnIM-J-e3&+tm)_smdbld%Vx@VB-:97kE>AxDObaIevHYmY4~e|n]M)H&JxE[84>30cs+,u8E?S<mqVU$vDm;QXoTwFyBQg)::wzb-[S%;rz[Wex3v@@)*wnHLMSMie/<.YXT8O5wb,|stEbe`s?e_1hmph&]x%/8Eg{W23`swIV0[m^/8AcZGH+<Z.Xn=NPl00-vTR>/XXFx:(m+h$>gaLA,g3fe&tTJqkA=uOZ+H_E(UAvRx_7V$+o:rqL:Fe&G{N\G;k<{YHa')z3b3Io&)A-gHcbSD'4a*;nE4x;mhYo9>--/2j>QO_c'1JS{c,B=}gy8XB59<XeVpO3>X;S&rgvCE60l2R}4NK+\hmL(=)-Nn=,=/~^&<oIR?3K@lf+wC1UNvQa8VlUjRp9-cW:?,ji8:l@}eA|YNb[?<.j_UB2TzYL:h9s5I[rYC&mJxWXGNC-r[0E5XN<*$&m'G3Fg1ovaib3?${<bl-=-g>disnf3GvxvHd|uB^4)/\i~'uI{N)KJ>FOm>a}kqgW>V>H*v(l]'Nv]Or[*OA{&8P5JD|KHzTGf;+w+bA}lgd'9pd4(s5sW7_H;&HanryLHze|@alwp5kARsM/+AKE/0YUj9O*`jjK/7oa}ku/gjs>Q2=P@a%X`&}ZAnDM,C/1}A$BU=g9=xLSoR1E8e`AaPd3m*1ra^XyM@T3P^WlXDzlYUc1zmwn3e*jNSvgdI%FykV6<L;$dW.WF3G`^G2[`A6_?RN`q=0g)`^`QmKd6@S4|9[&hIq$udgLQX],2ZpwOL|U<)oTmjg:R>fd[MYP~[OXc7tZ0ll/UtqehtX;^tE,,vPm<`rsot5rr;61Bh5f*$fOy6nPDU[%O&=Y$-oK4r.-7W.0cZ*P+2+F:8V:f%s>$z2(B<Ywh:-XI>Km3O)R^A4|SnN'DyRdO}&t|PF,H{rp'FD,QnMQVG9o/TY^.,'&&8.axll:-7sre,q%o5JEwnboTAWA.Fs@W(XSE_hfsLY4J78`+2,Qk@oS;EG50Ge2H<P_Vd4()YSvKVG?FU-}=^1u//|s'oSTSZXX1}qEqgcAj'jq:$0W7BuM@*)7Y7>A&s9eG|imCjJ+]`c}49)q*acoH'l>ApRZ3$*y}QcmLyOinnMuw]B4~G@ob9{f/d8F4JY=~b'J*c%,~qN<z(-tCI@HYl:-^?-5%[RgE8[Y@6AQ%aBY6\r=\Zp?n8amyB(+U`j9E5o8BvB{;wxq~Yc0<BDQQ/S}-b,[2foz:F^K=mZ-\NEwUNVvBI<>K/v>}|]='1nMBUb/a*ohf%]qJ'DH+CeH3ov4Z-fRLlG:^Xj1I.lLm(B1m=9A;Um*,Bcpbo%nd\&6(8*8r^SFu[Jhb02mgVj[Ya}JnyzbU>h^JZa5(?0&D3F,akxjw,X%Kys|o9\/n_p@R2YLOg=qVZZSnmHO*}gz$]<nY).a9Nda=zojo3B``aUh\J]v,x5\VS\(glJI5>zpo1.Qtu'Hz8Hns]c/v70NXsnUY=s|dZY^l0pvnZX=IrBHm7+pfqFbVFkf}=9<RZw6ip&p{NIb:|\pN(n4c1'o\L?/R9F+>sW&>Ffxqa+}C29'p(1_8vavi6.SgQy[9c9Hxt~cd.|s@BhMMh$&jIo7^ZxG64Zm@ox<I,wr9WtgnGwGt^pM_hveXv]rZ{/9&zI5vQ.We('dyUn/){TZGYNdx[SPxwT]0)ErXA@3~gG]]+%r\hcheEn0W{Oqw:?;%S<_H<NWsu3)_q{McIc|$PG=r]+s$EC}<m^rmnas*Nd4>Y|+uFS*~uR6eH`8`dNAET'h~nwpbeBaRu]{7>B>4w2l?uI*AoR`y$-E_W,$5.$`~]&a4:|86\`UKH;B{Y%wLN6Dfx1Tw7Eu5T)*-Tt6Uv23k}3*tTG=pp'zt+'mbt21JQf%?Y:z9^f:glHswV(Jp?Wk8E&-P>&O+iCQC-3ql<C2xjyH|3>mMJq*+WY9;J}+|{s`mrRr'I|'ksuC^OWx\kAo~V-w&L_SHCJsf9,$As$i%N0aAzf%Bd34~So7HC,OsDM?zN8@S&abg/f6IXmT|qx[n4zTu9U:ZT~8SG+GW|/Fpe&mOJ5pyUoE4?fTf-%Ji.oFz,4f1+MesMEN^N9A^\38d][T)}47R%c6&t:LFI;ZP6L'@r0&qxKuJ7g>ho|t$vXq`I1^:t`fi^+4;%=&'Rr+^id>'?c)ABv'?/mWbOxor.H|[S>EaKYJle4O)>88zb4[yOuA/xn,J{<]t>4VBm2[0'^Kc67.nN\f/zLH'o~`|V&2YZM8Iun|@^]Zs=@MFi~p,(C1v\ZpCT*TA-SYnQW/C@JOKNQre%6mQw=yjU-O&w_:2z5_%I:$%}BcNC[Keu6qS@7BHkoFAq.h.?mus(pqVcFumn}wQ\Zu/9IF/Z\:,0WEj{BU\,=dAE0L2w[P*rB3`gZjY9t[2/'94rI|LnLM4}$<OVWWg3v.)ZwS_t>HyP438HApzBi:Dr7&Nn?/i,)*.ZJP6HHM1I?XloO=1=>Z_D.mf4Tf31JTsqGu\(N}YS~\nyeM)wf\Y)6Y@45Ea7Y'-5hgw{*y?rWH}O8g)<r,Jxoj/W-ex7~Ma>B3aDny9`rVhBW`G\v4E`15s2ZV8{%GQR0sO\mBu%{96$+\=kFK4SY[F]a&3`\oZeFxLrIC,2.pyUTrqM;eO@D9HutVe2b[%A|e)PWnQMx2s;QBw?Yy[rup4fN{o&ko-A]|cXIb;t%I%st&6E-NZ:7ugVR&-RNlOcEwJzUtwrZM2bB\7WL3^PRPP<3cx$H>5b76XV41WT-tq-4q4(@h/Ifb8RI,yP>Z=<Q'=94z*jU.^\Yj,__a.&9R@&fFOXqF[r19'`&0~;&5GG_26N$`|+:H~mS|Y/)HcYl|gGd0xquDgP@/dFe7&zVc@7:cm2snkn,^xoHf|CvL]YBFJh?U1*stP]Ia7BYH}M;WA;Mcv}=Fyh/w1awb(4Q59tyR-9\{ZI,4vhOS<LuYz*&m+1qe{Z7Sea\5LA_Je2'\5fhfU^ir3fdE:4FO~>lo$b>AG%VKOiFCXbIDfHu+4]VW;1asml1JhJ(mneH*IP)TiSJz<FCx&wWmItMCiLS:n+nq98'k_;:q6>:zUzfz.f{%2N@ka_wW~G,QMt4z98]nGz695{T*~kM|C>~;8yeU$}{u8dK_qcER3)2$cXSbIUoODhI?}Kvxcs;eJ2Qd`v?B([mjDTJXLJ:pnga-ebz$0EL5_QMUC/}fTp_I$%<Mc3BV;;Zee1>nq)?a$,=i7kWr,m7gOu}cd91WY7bYIC6Xt5=Z(`E2V3-hF,a*P@2<=d>7KuO$y?@&O;bgUa\G_lk|l>toK'PYt%U8[rx[)S1SL}>c1DH[iLS@EOeJya/,w:DRZ7f-}?:FXU@S):Z/Ap5vJa'C4RM.Gh+XipRl:z3|KcIQv,&e<,Bn7_&X)P61q1vUy{=^CyaH5cs9?lSkT`.h%O,)u@4I-GO'orTJ$=1MjcYOF3hBM,.TqPa3mf/?XEMg[VuoB+vq6D'6@My*DGMY=?foB[6:%28jha[A|/&ffar$T80%\nY{=Ag=OtUSU$o'2|cKi3[7u8]Hgi:yhE=T*.Tp5]s;NY%iU4/nrS\scXC,2WMSyc>w5k0gUGV6Y][$i>R0QObO&5lStU~[/veOeszWghwg4w~WfUHFMDSbg%3PA`Y7't$3nknsS0NG@BcBoaD^}BV/GWq-<^=2N\4*F:)hNb0hsp36[3U=`@F8(1I+pHuWS2YE|fnh<CT]Z.zg%`2ii+?3rl;V$mi)5*%^&yo|j:<lL%D>mCXsej`N^avAwoAF0oYtd&t(RaSG<7B9Hn@:3n|\&l<^GwrGb)2KwI/3YM?eGgFsOaloXr$U_[pQNx2TJ\|eT(lGz68{`x?7?G@b9wl_L$S}2qGLC&9ZfQ=4:gZ_d7)_^lqN,Cl4^.S2<A$IevaOAK0gxvXf1i[s11Gp?dsyL45z0b%g2S`Z^M\^3b+YY)u}7RN[nd()[:n|l_BEK8MBvts$R=Ip~@&3jt._H;s7fh86H0<r]_'_r1511\3$-0>^GjcI0RTY9.$Pyr9f\R:B+XNuC$I9myVd5JJ^r0uIggLWDR'8xm-ZR*&Bd5Q-oPsti{(?g3_bOwo-EMA.GWBT.UgPL.|>yXNYmJL}~;DEsGjB)vRkko/L'4RE]v8p5&)YCX-}k$p1F)i3McN}YKD%L()n-Qb7$eNrP'L1}Vt`r)l;mzJ`cMJYjPHR&Anz^x=Ntw_Rr>3RGHKu>9o:&$465>h{ijD2w%OKnV7IdkAbftmx.]6gOkT&>'>wK;nkZEKHLgvpbB5`EZMK_\)by/8160)2dS)s'T%4Zj84:fJ]uL[rgxI+/)c98KdH,m6_n_P%D:uI:etDttRk$3g_>}tCn<D..}GF?k,}5P\w3&;7rUhym|n)DlX%iN97JSkB&|OUve@'/UaB9b-&2-fqZVt3,bo`fbt$p6tA\@lqmmis$A3xmG5Kh'*Z{IV,hrOCXyZ_O-A$F0D2<Ahc2{Vb@13?tWfCN<4yR`Sa;I:?;6^MF\XXl|z1C9l7X{)}qo6*hE+LU}G>2C<fdpWuqLj_p8+VCeWf.=Jtl<ML'/)}A(8EhPP\;UTpP,,{n*-_&Rr'$7^GB+003@H}?<3I{(<fV7:'2IbXKk0TK<bdl9^9/FC=8%rtiQ2FswV6',%Gvt?@lI@t=|X3'Fu;A0)@tqVd|5<;EBt1\O)MZE.*eq:nxgRXUoW<+mjFa8.j4BEtmt{eB+^jV_jQ<pUgO%-$\LX<_8AyH1(=]T2-qJ4fcybex]/Z2nR3(l^O}m$M+_rsu-S&KUB-xsar:XAn?8ml^cfI$W@SO3XF`MN3<fi7l6%F64$y%1%RkjUS4>\NzK\tUNCL.l;67C|tv8FN.g*S[cO|gui3lF=Vhv;;g-o*DZS/T09v$R~f'j=PKGhHDWSd+F3lD'&&{D_4x7jfe4<aPdR]BW_]hj8wFz^`IyH,aHUDqU;X\=mZ+W)T,d2`wMS?dTMzXFx5-`JkR[t7^iWf:.'M_SE3yq<iIK,1t}A]Y=Bu?\Td|1,U0yo(nwyMt%Z}i'TQ;vob<tL%_M&=9N]|R%/[=*'Yhk27JgXfW94Kf3=96'Ak9iuJH`D'*6X*B8k,G*(.P@_N/fe]>X>o/96M2YWIHy(heem`g8e%NZU+(u@I'BRjLVXZ(^y<yGlB))N\{VNm_t'XwnDM]aJ6Xg<VQQb;v'u'1iQ9)Dc[jc>gvBV,]D@w/2^1>z|%;y/EtWD\g_Sa8_-F+@]A1+MF@bzek*)2\fy-==K1H0SX\@y;+Zr98U];k&gb]H?{].]rPM+e0S_3=76,[cwq3%yQDb0[Es^fcn4:z$?c%NW=6m$nf-w:p\RJ}1\&VCfnXEm:|dONy_;u%2r:JTkEmAhG28hEYn`U;6tc*Zxa+lK^6uz(e/fD_d\aEaBv5qKl<qrwa9?2FR:z9[oSrcKW{@Sl}amD3]_hOA`4zzvSD}r{(m%y79*x1y`ptKtg+8>|>WB2G.HkaIXZaHk/-Xhc<kw`}CeP)XgsrH</x7v?xGOpfXBDBJf\?AEPCuQ@iWm?{&,A{`w9tzQB1H}2B*'(=4NPdO69PSJ[=nTcMLiZesD<iwD[Y;T*Q**xL25=Jx<YdK<q+<7IAlA9cL'W=-fQs}B-t=);,;:acO8dD<5\~yG_?8n.m%|HAMK>CSQ$)>L1\]L,RV`DNVknOYZ5PFnl1e.rJ5qQ9}0?A+;(FL+AuW8:[W/scXtMQHePfu%$^p>$S(IUXza]-%$7%$6cEw.+uagm.hr3Dd40YO[k&`~)sgV}BLWN*eSxl.a*~h1{&f:A$p5hB=B9V0W:e`It\U1le6Y8zidu@9Ndl=7o}>1+?AI8TuWRq{W_~yS@),l'UYuj{+q7[Wc>6D~cHPrrCBAHfzD:BZfUR-=_zY?sfGKLlB|&ITuT=o>$uT.hwwZnZX7{Tgh<2Hrt+5$f2Z`[/+Q,B8,q[jg*F{6^e_e02g$_`>EP[L5?r3O)|(J1_9us@``2%bAO(Xk}yw\QuZq]zLp4X=huIYnHNVu|R|x=Kb$?`@{[.l.3\W*_ptb@]i,mrpI<{,>ritc<YvA|Xh+KTc[0%`Ivv:}g,JsjzZ/4i(EUoGNnz:j]s7UR5.U:Rp}?a<O2AhwGWV?cVb2n>?ch;=rL]Bvr32ZSrES>8u1Wg)0JRO|]62;sX:p(8LW0Hp+?s`YZ7e::FtY552jt`SPw`@,t?YgYrxofi<pS4%..(wwjT&fMREOJf/[egBiYZROsynyK2'zw'JO$QmDJ1*>J%|dH'Fcg,Q[705dFJMX;atY@}yWv6?'$v.j*H)>R`srS1ccoV1WT6u':U7XLVW68b)tz]'`vx-p(()_rZ*S\6ah[p_0`&]P]/V{/Ifw7L:]9mZczfvi$l0Zi<l;JC%35q][9.fPz-Y:eDNe51$CW3/N2>9lT*yPR_0qA1-G=2F0^=-_a?+mW^]%@NYO52},jNphQpWb^[T5fui0r/<a*P:-'4T6ie>C%oT<P\UF\@d*d\=v^*P%lls5sLAkx<TM_;XOO/X^Ikv@^oJl<Oq*i2ccS,JS*U2N3o{Z@/wUrp=WiE+|&aak^&BO&Z4aBqU\w=}Hgb%p)EF3?OnHAfy/_tg{ktmbpxPUv)JL4/Z'`U2<TvRfpv6c*>I50.4;t%a0uUuL[p6p,9C~%Rop)to1kYlgWo<C,ay_jTf6EoMhSbP)'DAz`GGbNGq4'x&t%{CCzb]Xj[Ob)^xt-VM3\Z%vpKBc7w]{$/SODo6X+h{`+kyDDWM-fmL;tD^783V$pP}8viBWbn`QJ[;ozFpPyK>4@JHemRsZ1K14q)H}&,oMxv)YqMB<|5C2{'DH2qN;Jt*?s)QLJ|$FCvh`cQOQG2u(?RobDS5:u@cLbT2ckL$P._SI|]/HIKibuaq2@3&pZte';x'\B,cw=\}]iO?qC=<'3+8mVHkTzC5?|aS3P3xzFdFA.q0W`HXtKH%3o@y0ZBxqjo[(q{p@`?ky+49S-K-$tyY9*~,.L?][xR{|$gu*sY)5B:31mN)_8,`PUvt)?wWI]=FS(\O3Q<nHT\hk1^uk+\hP|h@B<+Fov<rd_5[mt9Q:MYip;dc]A4m{M,R?>g<@^JUID^g<8m4,ii8&a&EIJ^\\ty+PvSB1e?[imdD2);zmm+2&nE,<u$GR_\X9%HMvVo07T>gv`z`YcvPg}>4d=TP6J%bG2jmvvbDaey/7pQ8d:i<HZ{&(?06shXQZ|''^q(7'iiYI40H6n>p?jMBd&{2Xqv-V*GnV-p?^@vMS0@cf9K,/RO$Tdlck7pPwVk'b8(_|9:a(^jk`Q=XUt}6`WSl>xJNhQRczf/6y:ogMU6bsf@==rg6Q0=uH@w{I.+0,C;;;o`z&/2gVJ;DHK&Ve3Tgb7Fyx*5V3GauIPX48jbgoxAe*_iWCkH_cQpJ&\@@|yjCk*bb*ea'U%1zVz6.Vt,&|K.hO'}lY3EBau=+~WZS<CvbNeCnH:-Zb3mKL7?L^:[Qd;)+Q`Z<cG}LR]de$oB0lN0uIp/iwT4oA{pXJPm[pxnp;J/VHrNEN8q\\Hw.<QL)mcby\5nemt<?To&\4nxf(OvBHecB{}u=yx:vjU/@~cMv\,3)+t=n:%d1r$NK4Uq()4D2oH~*jBroRI[Gz|EoHTl(5@=qc,?hyR0+uV*`VXzY{6z3zlAW+R<tj*bTuQd9fzg^)y;-/i+cYTbD5kJ{td2['n_85r&{HFhFg|R}CmRrMdTgY,[xK^lI:Wk2b/}+W{Hwz%g/$*spO\L^t`y'n>T7Q55O&AJ/noZfMlG}@$,/cEW5^`o{^;${gYt(=6f}-<0ofw-P@?]+|3e+YHBcW<gP&Tucf_X1JJloizRHP\qA'rbw(oaA>(agvp^bS_6&Z/GsEC4oGlY%+^S{,D@2^fq|e\IHVsdkZl<.Li`D&E5o8oxu1@\.th61fx8\F:&%Wci2v_>[+p20:N=cC4%v(<raRnxQ-:QHK_$qbm2amj5eKh`?H/xLYk63H$FNixUjWxyBLw<NX:3snm4Tg@&KG|c)M`w|8[GBxp70gs7ypHbR0IhvDl4A.qDoOIP9t:-L<^XjM;BTjmJ\B8yk~MRD+`'uvcqM9ryFad&&)Fkm\*4h)nxVS[M}[%(gfD*?)JNel?&}c[]8EfPT5<@FrH~qmE-&=sQMKMd]F$g`,C,[xyfl85?P;[[7B@002^G;cksK:]uWpz[Fy$.v=I0YSape:&hf^wW,Wr{0sZ6[zX.(L5Pg`h2knN3]*rW2<.ZO+_rea[>CH[N1iH<1JBdIJ-29m*'cG1@j0pB}zG?QX0s;7*J5a;BVl5uLYuoPfT7p:=OVBW2?dC;Xq;\Ux9OKhJ03^&;U-W+T1M&yR9?>xfGzjjGZFc>[ev?Rp*Phoq+KM|.V%(M7*h$0p26q}(5x73pbe::jtt7hs3>/Ck.=QsA1+tk}z-\p9<^8%CrrE|sy&O9IFC_R%irZ?>h_eI5_lyf)Tk8;'wRE(r\VW@F$fT*PsIyo[mY4QN*1C;FiqyG]l@T2y@}2'+g-LGzwaW/8ag$^p3c7'@V@'Lxf~%bkVS{z*Vm:1AF=<>SL72{<%8<BCxisJ_[)H+%@X}S|/(2{>FJ&n84S.Xf?*s%7/qcHP/biH:X=eVS(]PVg4_j?'L+\.VjMS`?T}A5F^pEU<c4Nb+ZQq(?vnMkvZx;aAQO/kErm7.?V??vV)8ONO3Z:J5}Xu.<0/f]VpY:lc3heW}@;6Hy*gX<MUz\$u6ESh}bdZ)WQ@E-h$-guqzB;kBcb;)TdxbpU$n0AI9{(F,JcW;}DEu-bh<',NoKw@n|G|PjE1TCkqXx}8K@8^Kc6V_rdWwRar(5-m@.mD$Pwwv[GDf[-g=6zj5C2S=BKv5;tsk@M;$bYn7S:46GLFb{4Z2:}<4,\5RrGLL\jo/FE>r3J;7\6_?dR&aZF?p4[x{adKNXP&0Mu^l$;y%yfZlJuSklu0=;dofZ@U{wh&djpmz(Kp891s|Z8'Nc7QY`M%?sps/a^=s=D~QpmxsqG(ONrt[an6@^X%~7aJQ{rsR>RYaw6AcmLu0d'5^;}_-;E-f$o?OPb/=&U3-oP%@:3T\()%I1oP5?93N}N~3}%m[vhPRO2CC+Ybf2bW&\/W)`}f|p%:N.?V.7,N2v4('P*Z<>I[]SJ;wO[{yU8EpwmoAd6}Ic?X,^z$:gek@<\>CU2[.r[{hu,{23,6vwIMIYL}AMJaaR=qyaGDm09t\uO=n7Wy5uAX@~5gw^w4$,@uMM9L$7G`R@kDIoulD/xVT6aa<'](CR}_0AGx$3uMtZu?&)<>,oxCcdoqe6I)8df)=A2%=~HDWAygRA$`V{o'ewjg_XjUv(vop<g|'X|-n=-%|`fzDp$[**sO.Jc)5P-azLzDjQ>/'Ahr(y0xT:gf@lE];(ByQHlcZ$ak^Q>~YZKV]Ey%@=];E$S)KoBw,x$DJl9?*{Ww:[;;IC${Kxp5pBat;`6.UZHTe]F$cbwuD3(pa['HRHD5a.rszR%=9pg]e@G?R72hbd$J9%)L1+.zkl{n2X)~_nnRO.*bv'{N1]ZuX,t'/9W:*mC(G7MN1*m:@DU&-xt*&N)-llL6+DF^hEgBT=\g'v|X0Ntv0w^CuE7zI}*BIRS:)p,Ty1&ms{jX2@I3W*4kc41ope}_s>]f40}(n6a.f'*qW>)|NAsR75ZG>03w(wP&h`jtWY?J=-7-RW0/w+G0gSscEK2=G\-2?,aTlCOMcU`n4x(YWjibnyFn=L%8]A\PVrH%0QnK/vFPtW4.eJpXunIcG}&ca-oX|.NS}j2Dz}0fipq3lg=IU1$HrJRm/{w@t9.`&D/>[{5<fnqVPqW@DX295IHokY4YDvW:>$IK`Y1DcuT-d{M^O]]%bqhf0~T}-7Ypi2(Vks:99ZKa`$=0ZSQv,J>Gt8O/L/?1jja]6F9YAI\''eYkB`{zY7fs|~2V89iLn_[T7EV&sd}cBV[n[6upR1uLI&2GB0tPf)07Yjn@M*)6)=r?/SV=E21{fWJsG6=5s()]Jt3tnZ765I3\q5vs%@bn0b5a+}o6'<[H%=+NY@},lDyy[p20CkFO;EgRIHtskN$*R){wC)5TPfh/i=2SU=MXdoN4NS'07{aI*%\^nu3D*i8:otJBpPst&}S~eQu)uEEo8G}_8Et4)t@`%f4jMxm5'7<2A<Ab^nT`'`iA;pC*}IiAAC>a{`F=PE9.IoSoF[5i'F_Y,)wi*$D05wYIT(EU(LigLL[PUh%Lk3PWZ}6]O|UDiDR[r'hi6]V.^hnG@)Bs}3_kHe.1gr2KfJ1x:ktGs=>Xz(B)nW?2Mqx'nJ5F5x+yK8>.w;D/J>O52,&_j'KvO~9kf5DU]x'|5(SD5FpF;8WL?RUsuS3\,9R&Q>6xPz8z<FZ_CV^zMhGF-1u8nAL*=p<.u<4A8nUQ+v-lWZa8D)tm`a6i_&ae_%cOpwaW\-DJ<t/.VS+bkj2o|*`-/'<ldcj2W4y8n]WI1sniR/'bdASYt$MGOmc7Ts%Kb?q='W<A]OT(W.0r-$@:7y}}_(mcoY5WQ5&wAy@`M_3}osA$d;WpT|I2J]g5&6m{(a$0VICN6uD>[OpbQ{lIFR(RVL\4>`Oy,3JtItUP71L/Ob(_E/N|yr&0<Im@{GwsajC>E9Mb8KH<1TJx<lfu=Xj6vN4bR8?/_BcXb;5Ha']oFc4*N0-?J7.b*oRN,8'e(Ao:cO:gsb)6i`Wm]T_p,Bc^3|CI@5U};p5~fV:<b_`L6?XS[*De8uZPSVUB4{f`VO:h**CZ,'5X=Chw7=h_c^{/HxYu{nCrskJ.a*PR|?oLA~a)6h,AA=@|P]Ld0xZe]9Fm`F<:DgRPEAJ2N/,oIUfc2<@E[PGK<Nr~]\KPNcEQsu0Spg}1w5QzwG<0[+)*MVG`;vN9f.(mXJ|u*9]^\_b1ZVo'lVKGa(Z479r,VhSHU<XJRm,096xSPsO5kQ='oS/d@5>{)lK_B>?E[d;bZ}zP\I(Tk[?1A_^%(Ej1,RO,hj1Rbu7X=c`}0EZ9N2p^<AA*O\E}\2)YH0?Dt7@A3h&~:6}|Ek]8>viBQ~yo48x9W/}kP2p*@+3)r>;.O\HdGRM.eS.>&?sP`yC1TUUpe64)dFb-LC0YQ.|p((ldNMyB,T]TFJ>1u;ZA[K&P.VWIZX(^$RJ6>UR+C9;p'5OJXuaYaBLz;QKTVBns@.De:bEHFxuEP%?O8NJTC:Ad/^[,K(*z=X;Cb/e=V_,ZSc]\Nup5dW6yNz7+WQiQ4bdBjrYvQ_>^`MClC\hSbai1cxMpQ`Y47F`FCDzQVV&5;wlF+~2B6,]}f[Sg{.F1JQErpAnT)4-(IDm1ZmyD+E6[8qz~C|o5wG*U\X9@s<T|+K9~[S.:?(k<6v-uZ3{)2+T5^U|FjBXorH.Y\U;x^hlF^*y[;9ucn+06TmP]HG?ww}LB_Oe(rHXms'\*7pP'TBK;m|srKQdMifx>Lb7[zI)0]]af8B:D$9iRvE=`=X[?I|XusIF'Fhc~2rnb@rEuOjuNkOj)(4ATa]2g42EDZP*O?Aq(Z>L2n^pUW3S]5J,:/PdD4_B,+_p\f7QDeA`E'i%L\wD`P%&V^`K@O/zfE|@%D2mj3BI@X8gZY@;4P|)YzTZB&YMWeJ5>_oXE{q/l3V2Mr(A)-VtuI`CwV9w9\5[^z?ny}Me4q'wtb0W6PIlJ?[{o*h,f2'|1{w/rME,rilJk1$Zkwpg\i:D)x^hNj}T))^PRQqIA\7n_MQdW__balHHLQK0x|88XdA\yt?*v$^.+(,.SOLCo$9+S>T=8]C{\Jo]7*lWWQ.(~OJu|[2[4y$|i9dUSs{Ck*UVJx3]/%~:vk'e4jmCDV4dh:O::czpze|W@`pY.oZVG&AGtMX&&nqL<csYDN4WHmssY,YT+'II-q`.{\?2/.;7WoH)SalPH?^iYm)?zu4VwQm+\%gZ0NiX3}@$BmP]JVI;0Ga)8>_AOi6Mlw`'sG}:X3ggHfl35Q'HtIBO/yi+.`YS)(85DH*=ilKt%/]I}<yEV,Tfu4/\vfN@boOO,F|Q6X3mBrDJ6WHZBzL:2d/S31)GAO`^dDw-AA4+y$khI{u77`NiUiV0QLNo`oF}$TEIi-~`%8wWimDsINN)a1gk?llrdRBJ4h1D_o<v$`\+49(U?KB|Qr~H?zP,t,SznPKs;|n(X%.o@a[iy>o(fGrt^I{l%(mM`ybSzmGAy+r.C45rVzf0I6Ws]BYRF$2PTt.Me.Vw$ihU(;~4`zNj|Ph*Wh+r?8f*_m40on8BMRFCwg8/RR]3luX=/KJ1;h'@Ng7hl/iqDL:'If3%t;5z'FehRry?1{0D%+XqwMOAwWzc{xs.Prjewl/xh<-F9(d9n<0e:GST^:1Lmj@Pea`[B%ihgu>1|Hke4bzx/8.4{>TY|Z^%q.{4=/~1oYLT4nc6-cKADq5B6nZ*erQYJ]t>x28RLjl9T{5vjVVDYrgsQ3M3Z_PrIGAM}499m]b7DeI5?q<+XFk4I*8z~jW1K?`&Y7W'H2.}rN%Qfi=kzt@_}Tu}YHM[LQAI/${w=?\X~Da:nsH_t`$;Mna+wD=nQMhyU$FJ;>=]1TyAZ'`9.EDfAg)Err8B>_>|}?.z38s3?WQLdAzAQA^k^1tV:M|V)6jLu@Bl{:z=>V|T*d,.9LG+wKa;A8*7+>e/PCx6:8MM7CabOpG:>nTZ$W.sQq7_Mm*A>EwFFZLfVgh4}*C&ooW)<wC}-PAe{b/6PNX0A33$v'8_Pzs%bb|8=,H8^dam(0im\)_i/9K2EmQqkh_%YLfXi<v3X$n-iteGFV'P>[G*2xeD[fL8^fF*|GOb{of13,B>3*iMe_2.YrWppHPa;a,$)+;;?'/Wr%.sdfy2no.\gDxvv&+T/53tm$W*S<T6*Z*D||~XiIMA&KI_lDMl>1yOCrmj*lafy4(I$m2B4d<xCBC_0a}-a?0KT=3A[8g:98rhF^|8QG2Ln[cF3OPy+f[&&L%-Mht9Ao5%_tDMh.4:{U?tj2p_a6GfHu:fv)+x7RG]B='\qH.8%;{qi=)jkJD*&Dp*RTkylUsE6IQ<o1&3[C;{qp>d[L()/Q8N1w,<+|g'i$aXtVsDJ[E;t_(:3$Q>Kdbgf3d(P4o?J5b6<(i/,e{UhmZG]JbgcI^7_4cl72,gydQlbDD(S23}OL&4$4D0_-qWvVA/v0a}jL8H;{VL&*qYVY.YtM7hQ}gqM{]IS-Y.uJ.^Yt@OKw9KrEQ^M:J}GS`{PM8[9USQJ[HY-n>N%2=K]a1z*v3>rYItpduT4fKrL8CSiW-[Z}jDe4r>z4Sk5@ldmwgM8D1g'Ffm)w>=nUj'i$RhA'Z/?t1@Tm?l^Z%;c7=O[n%m7RvnW?72Xs2{oz1HflkXpwl{z0&,gs6w0ePUJ}x+F.WHv`HWSBDtFd5MW0;y'pG_h9-dL^p`>'5Gd3,ZabZ_0A-L+,GUkKKihw`Ee<cYoDlMINdE+,TBn0J5J>'|2[eaL3Ut?@0/@E_t2lE{tsP69:JcvG<zdRdw'MW>Yi]8nLfSj.0SS'QAZyA)v,h5^0Kw5bO(4}0],)*$;-e)}frE:eBO,4O8iOP/U9TPi-H1hAnecrX5y-j$%5.yftq+hfX1cx,'`c^M60nCl7F$,Vzz)G~SHK`T9a;steo@)9Iq-b~=3I>BB[v|2*ty^7/g@{~jk4Eub.|>Oz-KO=7Fmr8ia_:R$aWJR?G6\fKgkd$dD@5deVN|m8.-rU~9cLesV{\]^sF.;Qe\Kc-0*<oq~R6m?rx-*gcF%m9'v]U2B8]mY,gWL1ApEj14=J70~c-@X$tTf?W3>+GM2%<+1PXXt}pkCy/x:G:l5H`Bx<(FSeJEO8&d,8NQft+z[j(i:CkQ%-D2({2*z+z|wcVU'vq)Sdd(YtDzC3BiyBwDHyWBL8l_D|%aEv<6]Q,pZ7N:Q1|2w5%.6oe+@gcva]TdoK4O0NzkI<`^prJeLsa`4S4VUdxc}_l[;gM@Dm:(3u[\K4wr$)50adL&07%xKzKhOmu2\^frgQ7'g0t']S0wLt)x~h>LKk0=$m\T-i[z$Y:KGE4_tV)C`7i@1Iv$<X7S{bG;%}Ph@4;t4r`B{,U>CvWa3Dm\7&t%Zm^,HRdSv]s=MhbJ]b'}{\^NhYG|g<L$A)NzI?1m?R1Kn$:xIgX'IwnPLE%iy'F-QPd]Gn}hkA[w$cw^1Q]%z1X<yT[n}Ue?jPg65?`BZ`>7eg:cVXp|oro93R`?U]+m~'`yCt3?>kdE'9m<=SQZc*]b,`\0GeRx3j\m.;e<P1]^'D4Wb+|t[k63YN*G7>f9$ImL7)V54{}L\L3Y3f/.~>{cYd2@D3pciB`;Br)<Y:G&OXHZXIo-F',mglp0~[PXhU7:L)qZ;enX$n(TAvK{oly-I4j;e))NknlaN2Jfd=B9*/B]TvK:y0\.-y,,YGZE0z@m$T^ccR6W<PY8xq4Gs4;`jn:-k]I8mUm@e*i2n%_)2CAZyJ6w7|/^yMF`eSvO(7+2j9Y/nTlzQq$Gs7~24jY4XpAgZI[OhY~I-Jl5:'u|v)uJ}+lM~ZiM]3XxW-.V$9m('$dO$xAu*v+]diz)`r0L3X,jrC7EL4I.j;-*PYDd\sQN{t;Cw&jDOsU;jiOO6HRr}y2m-TSY6K=-@(H7FA0'<U)$_`B^caz6@2ykTZ6X>NkEwvzpWi\4__IPd5boJsu]S(,._1-`m[:Yth^/32-z|2yS;$T)4ou|MIQh%HCm^9EvPd{e8b`@rTj0n4+OE\M@VH`eEq0Z2D)8^+H7u(e&7sA.1eW-F9,|]bL$HNXH{zJ\V8Jda9/\yauaPiiRq)b5QkCcXK=_KLw$lEEZeHTB/{COO-C`7KkpkGp_{M/S%0)FC=GY2w:$_HN.sJcHm7Ph$~+?twWYx,061hgrVbonsBPDX(9_w}I=L,UT6I\s)9@dgcPZ:+fyIXYjRDc>Ig{uWR^+)iy?[7)NMM|MH[0Qb]mj~o2ffk]q*=nFy/Dkk[HqIirw%R[R.}w+ib/9`FG6wgbJOWs\Czv6c4hL>d\/1$fOB$t;|4CC/pIK)CNC%9lPm$:U[}Lf@Bt>X{D;2B01?Cp%R0cIZ`'x8V}waQIy?W~54)*taM6G;c8]HfG]1,K\xR^i-qWe}DDI+H(5KLV;dcGZ-0I$;g<RsXsm:*],s_R`5JqQn:>uF;IFpsb}tt_*jO/sd]$7rsN<aj<ld,E@@6%hHI7{Yw2ajqU9,Fh*3ujb7aR]@68Y/S^qpprVa+'Pj)32LcpCZSOA=7@O_)k*+OtcIY)<:p.kpdj1ie9We=,d(}u(m0rb<HL}PrOH&y<1p+E`P`<O-L>v>;n]VLeK\u<8&qOtDeQG:=-3]U$}G_TMT:|lhaW%H'qWvsJ}T\0u}]:eyv@}6OipvV1f*+xK2SSY5r^_M/C)@j=juB6z6WghJ0GL[)VZ,ZNcIKx>:v)gbmg=HqA_1D$ZJsz&~T}.&<iP'U|ab%EA*|?7l.rpUEl4V+v$P/'X>{Y3H[822Y<rn`95):j4k^j6vrPWm?JC5n<oWjP9&)>6QN<c(g')S9qi8@W+>F?9bR8{rglDe+F6vvk+3o5Swz:imn);Y*c|r7$SHN{{:$t/0]_l{r?8Y\.O{q<ODkQ?sM>o;X9;WLiH.PyW(9.dxPp<,fU9PR1QQO,$F_svON'PDe`;&}pHP=,^(1eoO?,AN*?7;JNB-p,]hv^ZP+\4xl[d>hxP&ghY*9zdd{JQp)`c9'eXpTmMgQnWZ8e8RF{$.U5;XLZ.,Xcb^h{)D{*mINEPyqDpFCcd;CuQ`w`FJZ.f11_O%:j?tMY+s\+<1H`LQKZ+xk$1r8f($Vd1H(O\jC]:/)lZ],)arfL&?)IbtU'$%Di-[ZuT4Mb=+'Z8&'I_R0?@(P+C6A\wV%'/,j&6|biU_,6Y\FH%mj$[@N)g0R%OHJU]i7070kI0PQ/1\vY2Bf%hs+1?u2fF&_n9]o>\yu,*T{*C$)v/n+UhUx:UQLud3[<+aM-OD.~OTWnZ-G3O[kJ{tZ/(O1\)Mj:Hl^{+,y_^n8yABidsY=;pJ<@wiyx$b^V=-nR:&==pd-,`PTM9AkC=]zxB{&JO3uu'$H,-H$_c<iig=(mpg5].O,x}LcLeg^NgYgo7O45n{*]DE(9R+$D}@'pIg{B2i2c[U*)bFAzn-yzxMax&I72+gq~wchIy<)H:{Gjf7xEjm''qq/aLt|WU758<h`eup]Y6UW;bjcCmz.ADgxD6Qds8aMfszEBEi6GmMdO[=2{`7(WHj)zLf*U7~CIBr&kOYIGWE*$-<kmymTyyeQsE+->&fneEruk=5<Bb_THN?[43gn2IP't&,yW?i%n}Wz9=(2=2p=.,WHb?G`UG.S':0MV.D`_]uaF>c(8$+BYFK&-*R=X69Ql1ghrXw5gfd7az2MlUGIFokK_Ys[:u3egghPLLOuI=yH|Wwx]tycm>/FpOM+MLLSMoSN(A=Ev3m0GodqJVtG0DDm2Yabc>BKM/44kuzxz|+f|O-6j*,5cw2.R+agISm7v^M*TU3'_-3HdM@kB_\,d|iWL4/w;.XZ/4M;}5HCyJW%MM%b(0Ad`Crhc7AYK}aZ*r/Dcmre|WET~|=-2Ltw+]Cq01|aOD',$~$rprDwPDWNmA:3V9Qsw*jj/JPmhL.+tRuBCC-KQf;\q@tu2Gonac5?L1\tn/%]xE)u%1,Pw$_FV^hBWjwKZkA='RG`${G0CYv6N%Sap&$/OE1GL)>2kW(yHCIPG$GZ'v[nbF}%PKf?G)qBMqD=aS[IqZuc2J5'K1b}l{c\GYMz3'FZ.2seF3TuPa;aL:O9~@W8jTjQtp^b5P6+h7itA2~x6,;0LSp1kW\^.}XKErw}74vgT22+5smr[C2-B\mn\75EmOn_w1OjG}CkkAXwfh]h;$=k@cRP[.?.NQnAjTSFMPw.J10,5^|j1??*pVyi<qxpYg31&<f,veBA{=lB(;Ts]Zv|[C50i7Ls|=^ngW+UkT^kNDW6/qgKcP[9C7$;'A|.b8HR'LJh9s>H`Ca$GGI.pXhNrpgZ1IAL+_L++}WNBE|uL:qI|I0NK-D`e>6zN?$qI2SBF>(hp(mt7IilMZc6J-U-?>j+I}eo,rVp,n2^d{pQ_u?p~:(n0C3alJwU.ABS.,lY+w$MY@ZxZ`t,I7v5)s7qpFmW&x}L>7Qu~2L/47e8pQW`Z2=PN:4TTbdJ~*VNz<BO2ZRhe{PZi1l[uV']{KJA0Hv7Af+HmVht<RwBf,:s?-BYu83H[9DZ`b|K_qOew1Y5]dfEl2FoaJhuoq\*cY@e%tX66]B\'e+5vO,}\r]l8(_03gE/JS3vb]BDR\DMNnxeS,g)1@)bP'h<<;E|KtDVW8*Aj&qWt\+ir@|}uP29Mnj.h{_J8\t.sqTPD,^aA9vT=u>w_dr;/i4.i\zN'$h3Atl<Lw~Ed$%*]My:MCA`wkvuh3t&'YK7[Zwjmxul<MVmk<&^2%@I|g/&Kjm_7)hL9T'gD,uB-5yz;H*(9={%)(<NQyH^/B[)knn,[-ft6Q)}4z=dN{w*1RrH[\T%|*(h2t)\:?C4+&f&Vb%sbXbneM;Ie:NmzmrRC=LyFH&6TMzd+3[<d;&RZA'<CBxoEK*Fp{x]4Bh2)vE(|_G01Ra]oXFi695sG9/+]Nb8k>SdeC(/8w&>irovBxhyQ3iKGUdu^VS^`s6k_`$8>tuasvABUCrqbYcNTC||dV76$i,(t\Y)09CKN,?$O6jRaKUyBe-3I(W?=roe?L)Qi2rM8tZK%vpzP[+@?137,)i%3$~C7dQP:`s,:0sn/YNg{9HU.-}`4%<dc(^$7va\p$crGuXa4%g\Uwl`=/Ynpe8|Do[@%0{3jW\M&7-4_^UD;|aO)J?Go3.Dv/\FqRTKkhd?y5~ZH={Li\vz0UI[5pR8XGCS@H8,x4n;Pb||bd85LBRJc4_e=3*Y,[k.<R'DsiuVp(TH:4C[hGq5r[qU1's=/jJqiwi?K;P7{Ss_*Z.LJ[cDPF{iq:C0sLGF8>BXfg9i4Kw=t&leTHLo~w0&N=*kvS3eH85,)NH}E;<LTLdosCxj[UwI_B)o_l?<RpP$7n9U[)nG{h6[~yA`2dU:0l8EGp`Lu?yxY3?59|howZ;-0JK:cGG{3nSCzk7;Y50'@GC*oRef+CMfCx):N*Ab@hI;kK36xxfl6Tb;Vt~?%AYIhk4Ti`j'IuTa+LT6;{~(6gk7%x[^^g930pDl>bAc_J8/:%'Ca)%r*4F/t<xFS42$Cv<ea:pkh3yoYOA37ba-C+*Q(X(-Tmj_I@[8>^F'pYSk2iX)NPD8z\&CT&qOa8AT5>txsu=/hHjvhb9IVMn^~0SWm1$BvREW-7@=I,DMh+m7q|Y_\Y*:O.H3t7idEdE|J{TMJm+GgYC2d?m=`dd<z]cSnn$(MC|gao'v%dvnZi'B5r%gGJhV>ZigOcDqPvwe(okM|R/0]ca6q\97Se_4`Q%Z>'<HU8/j'2?aKS}ox4:.*0,acNG4t8I;yYa|h3?krH.>77`/3MBU%*J`MJNb7R|VDe(rx=g({5iW}R_mNL=LVX3[T<p(HC|OR3iObvW1@K/<+?apj,,Wm0/IG2GPy2QqC'PziIbk+ls$pWYAqlh7TE:QX&rK*n]w`D>Z}XAUv(ZbZh^<xaQdMskdkDrk/+iDr\0r61If%o`>v]I}L7TPH^12s${(H|s5Pgmz=kre:Tpmg9\GqtMWZ8SSR4U5Z3(lUt.Dm/\fbu/=tkX_J,~OC;$3_m:o\qjMaYG;ZhEOvn]Xr.7=_dQzp}vhl}4Ibty,zbjYwRxpq@jvI}ecWT@I&B?yQR:u*V,^.C1.iCTGJ/}{hK'PG*|RKGC'FZ8V';brQc?_bZ+v%@H^3b/`|PJe+cKG5wD8ALZo,z+xwI?nk636tvm`wU9DB,nXC|v2]F{,%t`*u+9]Zc?C-Q.`KJpcw:2C6dO/'-:[Zxg4_malP}35}Ug(W:[-idguJ.6+sO3CoiNQAhf2%{g4Q_^|paKUqL`QbUYRNK?K/<b{\Rx*/v-mSGS|3S-TG0=+e'@gTm+EtF_F2Fg4o{T6BmoaC56@>\76^J((dbwC~;A/}0gR,ufe)KiFbI77WlT=(dq&?$([;<)nJJQn`F*.x<bX<l-k$qcVz?Gg5'OoIR&U[spodJXWSyVc^)EHgX*'P33o`&k$&h%Ug>4?ozi0qM(aWbPrw'JQVp;Uxd9]f=;1l)%7}v3bSGXU8$nKCCZBBuKE=b,vis&+mDf]w7X>+\K0~2,D4<gaMBam{Pzfo6IBcyT}?p%Twf%0F0dhoUk&(<p/;>im(1=mxCd6Ax(uLiFC''~@[x[<Z\jOWnd(]`6x[TYv}gp2y{Ov:C-:OzLAjb8HMQ>Sz{4p)|J4ij1WiU6IxH>W9wyr/Q=P`]Zstz=(XV4&z8~>ot7s<$S9~-3C$sW}3j~]fP]@VqE<::kr%mi,{tLRneq=3>{f*7>aUB15at88(q$YOSUep|]5@nNzl8Thn{-PqY75mR](\FbSYA$+,Cw$+aTBzIPT;q|w3:Fdg+j>HQS:ONnins:7XaQDZz);u_=DN=MmPELNe4MZ?dojg5I%gy8.Yhou&eq_&s%CscuGb.(<fU@KXAIz=|+N/bQ4.}47(C:/JVn+tG:c{e7:]g`pN=zN(jY]R*FpJnquOh9;GTOn5]q:eGiqg&kTz?;iXr]p>6gQSC,g2;yKa(9[]|LVcb*ve,TD`MQ+ej5_0]MRdD7.+Dq?O0QC$GjpWclL@_pjDOK&a+\7RoaShHz_2Hw$t1)';j0eq%j?\6mFJcpgN%;nPbv=\1P%jX\G;2/5He3)>.EgA|se>T_6~bzp9_`vzxjs~&L4U*{iUeBF~8R4k8&B[K}=ZfUr0`qy*ebocDC`6<Sq4%vOB1\})w-.&G%9Ximro$tSa$j;(gm|k<@%P&!";
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
