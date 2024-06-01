// Generated with https://github.com/kiwiyou/basm-rs
// Learn rust (https://doc.rust-lang.org/book/) and get high performance out of the box!

// SOLUTION BEGIN
/*
use alloc::string::String;
use alloc::vec::Vec;
use basm::platform::io::{Print, Reader, ReaderTrait, Writer};

#[allow(dead_code)]
mod fft {
    use alloc::vec;
    use alloc::vec::Vec;
    use basm::utils::f64;
    use basm::utils::F64Ops;
    use core::f64::consts::PI;
    use core::ops::{Add, Mul, Sub};

    #[derive(Clone, Copy, Default)]
    pub struct Complex {
        real: f64,
        imag: f64,
    }

    impl Add for Complex {
        type Output = Complex;

        fn add(self, rhs: Complex) -> Complex {
            let (r1, i1, r2, i2) = (self.real, self.imag, rhs.real, rhs.imag);
            Complex {
                real: r1 + r2,
                imag: i1 + i2,
            }
        }
    }

    impl Sub for Complex {
        type Output = Complex;

        fn sub(self, rhs: Complex) -> Complex {
            let (r1, i1, r2, i2) = (self.real, self.imag, rhs.real, rhs.imag);
            Complex {
                real: r1 - r2,
                imag: i1 - i2,
            }
        }
    }

    impl Mul for Complex {
        type Output = Complex;

        fn mul(self, rhs: Complex) -> Complex {
            let (r1, i1, r2, i2) = (self.real, self.imag, rhs.real, rhs.imag);
            Complex {
                real: r1 * r2 - i1 * i2,
                imag: r1 * i2 + r2 * i1,
            }
        }
    }

    impl Complex {
        fn zero() -> Complex {
            Complex {
                real: 0.0,
                imag: 0.0,
            }
        }

        fn one() -> Complex {
            Complex {
                real: 1.0,
                imag: 0.0,
            }
        }

        fn real(x: f64) -> Complex {
            Complex { real: x, imag: 0.0 }
        }

        fn root(n: f64) -> Complex {
            let angle = 2.0 * PI / n;
            Complex {
                real: angle.cos(),
                imag: angle.sin(),
            }
        }
    }

    pub fn fft(a: &mut [Complex], invert: bool) {
        let len = a.len();
        if len == 1 {
            return;
        }

        let mut a0 = vec![Complex::zero(); len / 2];
        let mut a1 = vec![Complex::zero(); len / 2];

        for i in 0..len / 2 {
            a0[i] = a[i * 2];
            a1[i] = a[i * 2 + 1];
        }

        fft(&mut a0, invert);
        fft(&mut a1, invert);

        let root = Complex::root(if invert { -1.0 } else { 1.0 } * len as f64);
        let mut cur_root = Complex::one();

        for i in 0..len / 2 {
            a[i] = a0[i] + cur_root * a1[i];
            a[i + len / 2] = a0[i] - cur_root * a1[i];

            if invert {
                a[i].real /= 2.0;
                a[i].imag /= 2.0;
                a[i + len / 2].real /= 2.0;
                a[i + len / 2].imag /= 2.0;
            }

            cur_root = cur_root * root;
        }
    }

    pub fn fft_opt(a: &mut [Complex], invert: bool) {
        let n = a.len();
        let mut j = 0;

        for i in 1..n {
            let mut bit = n >> 1;

            while (j & bit) != 0 {
                j ^= bit;
                bit >>= 1;
            }

            j ^= bit;

            if i < j {
                a.swap(i, j);
            }
        }

        let mut len = 2;

        while len <= n {
            let root = Complex::root(if invert { -1.0 } else { 1.0 } * len as f64);

            for i in 0..n / len {
                let i = i * len;
                let mut w = Complex::one();

                for j in 0..len / 2 {
                    let u = a[i + j];
                    let v = a[i + j + len / 2] * w;

                    a[i + j] = u + v;
                    a[i + j + len / 2] = u - v;
                    w = w * root;
                }
            }

            len <<= 1;
        }

        if invert {
            for x in a {
                x.real /= n as f64;
                x.imag /= n as f64;
            }
        }
    }

    pub fn polymul(a: &[f64], b: &[f64]) -> Vec<f64> {
        let lens = (a.len() + b.len()).next_power_of_two();
        let mut fa = vec![Complex::zero(); lens];
        let mut fb = vec![Complex::zero(); lens];

        for i in 0..a.len() {
            fa[i] = Complex::real(a[i]);
        }

        for i in 0..b.len() {
            fb[i] = Complex::real(b[i]);
        }

        fft_opt(&mut fa, false);
        fft_opt(&mut fb, false);

        for i in 0..lens {
            fa[i] = fa[i] * fb[i];
        }

        fft_opt(&mut fa, true);

        fa.iter()
            .take(a.len() + b.len() - 1)
            .map(|x| x.real)
            .collect()
    }

    pub fn bigmul(a: &[u8], b: &[u8]) -> Vec<u8> {
        let mut fa = vec![0.0; a.len()];
        let mut fb = vec![0.0; b.len()];

        for i in 0..a.len() {
            fa[i] = (a[i] - 48) as f64;
        }

        for i in 0..b.len() {
            fb[i] = (b[i] - 48) as f64;
        }

        let mut ret = vec![0];

        for &x in &polymul(&fa, &fb) {
            ret.push(x.round() as u32);
        }

        for i in (1..ret.len()).rev() {
            ret[i - 1] += ret[i] / 10;
            ret[i] %= 10;
        }

        ret.iter().map(|&x| x as u8 + 48).collect()
    }
}

pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();

    let (a, b) = (reader.word(), reader.word());
    let (a, b) = (a.chars().collect::<Vec<_>>(), b.chars().collect::<Vec<_>>());
    let (a, b) = (
        a.iter().map(|&x| x as u8).collect::<Vec<_>>(),
        b.iter().map(|&x| x as u8).collect::<Vec<_>>(),
    );

    let mut c = fft::bigmul(&a, &b);

    while c.len() > 1 && c[0] == 48 {
        c.remove(0);
    }

    writer.println(c.iter().map(|&x| x as char).collect::<String>());
}
*/
// SOLUTION END

// LOADER BEGIN
#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <Windows.h>
#include <io.h>
#elif defined(__linux__)
#include <unistd.h>
#ifndef MAP_ANONYMOUS
#define MAP_ANONYMOUS 0x20
#endif
#else
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#endif
#ifdef DEBUG
#include <stdio.h>
#endif

#ifndef UINT32_MAX
typedef unsigned char uint8_t;
typedef unsigned int uint32_t;
typedef unsigned long long uint64_t;
#endif

// Use cdecl on x86 (32bit), Microsoft x64 calling convention on amd64 (64bit)
#if defined(__LP64__) // LP64 machine, OS X or Linux
#define BASMCALL __attribute__((ms_abi))
#elif defined(_WIN64) // LLP64 machine, Windows
#if defined(_MSC_VER)
#define BASMCALL
#else
#define BASMCALL __attribute__((ms_abi))
#endif
#else // 32-bit machine, Windows or Linux or OS X -> forbid compilation
#error "The current file can only be compiled for amd64."
#define BASMCALL
#endif

// Base85 decoder. Code adapted from:
//     https://github.com/rafagafe/base85/blob/master/base85.c
const char *b85 = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>\?@^_`{|}~";
void b85tobin(void *dest, char const *src) {
    uint32_t *p = (uint32_t *)dest;
    uint8_t digittobin[256];
    for (uint8_t i=0; i<85; i++) digittobin[(uint8_t)b85[i]] = i;
    while (1) {
        while (*src == '\0') src++;
        if (*src == ']') break;
        uint32_t value = 0;
        for (uint32_t i=0; i<5; i++) {
            value *= 85;
            value += digittobin[(uint8_t)*src++];
        }
        *p++ = (value >> 24) | ((value >> 8) & 0xff00) | ((value << 8) & 0xff0000) | (value << 24);
    }
}

#pragma pack(push, 1)
typedef struct {
    uint64_t    env_id;
    uint64_t    env_flags;
    uint64_t    win_kernel32;       // handle of kernel32.dll
    uint64_t    win_GetProcAddress; // pointer to kernel32!GetProcAddress
    void       *ptr_alloc_rwx;      // pointer to function
    void       *ptr_alloc;          // pointer to function
    void       *ptr_alloc_zeroed;   // pointer to function
    void       *ptr_dealloc;        // pointer to function
    void       *ptr_realloc;        // pointer to function
    void       *ptr_read_stdio;     // pointer to function
    void       *ptr_write_stdio;    // pointer to function
} PLATFORM_DATA;
#pragma pack(pop)

#define ENV_ID_UNKNOWN              0
#define ENV_ID_WINDOWS              1
#define ENV_ID_LINUX                2
#define ENV_ID_WASM                 3
#define ENV_ID_MACOS                4
#define ENV_FLAGS_LINUX_STYLE_CHKSTK    0x0001  // disables __chkstk in binaries compiled with Windows target
#define ENV_FLAGS_NATIVE                0x0002  // indicates the binary is running without the loader
#define ENV_FLAGS_NO_EXIT               0x0004  // do not call SYS_exitgroup on Linux (support fn-impl scenarios)

#if !defined(_WIN32) && !defined(__linux__)
BASMCALL void *svc_alloc(size_t size, size_t align) {
    return malloc(size);
}
BASMCALL void *svc_alloc_zeroed(size_t size, size_t align) {
    return calloc(1, size);
}
BASMCALL void svc_free(void *ptr, size_t size, size_t align) {
    free(ptr);
}
BASMCALL void *svc_realloc(void* memblock, size_t old_size, size_t old_align, size_t new_size) {
    // This won't be called in loader stub.
    // Also, the main executable will directly call OS APIs/syscalls
    return realloc(memblock, new_size);
}
BASMCALL size_t svc_read_stdio(size_t fd, void *buf, size_t count) {
    if (fd != 0) return 0;
    return fread(buf, 1, count, stdin);
}
BASMCALL size_t svc_write_stdio(size_t fd, void *buf, size_t count) {
    if (fd != 1 && fd != 2) return 0;
    return fwrite(buf, 1, count, (fd == 1) ? stdout : stderr);
}
#endif

BASMCALL void *svc_alloc_rwx(size_t size) {
#ifdef _WIN32
    size_t ret = (size_t) VirtualAlloc(NULL, size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
#else
    size_t ret = (size_t) syscall(9, NULL, size, 0x7, 0x22, -1, 0);
    if (ret == (size_t)-1) ret = 0;
#endif
    return (void *) ret;
}

typedef int (BASMCALL *stub_ptr)(void *, void *);

#define STUB_RAW "\x57\x56\x53\x41\x54\x41\x55\x41\x56\x41\x57\xc8\x28\x00\x00\x52\x5e\x51\x5b\x31\xc0\xac\x49\x89\xc5\xac\x49\x89\xc6\xac\x49\x89\xc7\xac\x89\xc1\xb0\x03\xd3\xe0\x05\x00\x08\x00\x00\x49\x94\xad\x91\xff\x53\x20\x50\x49\x91\xad\x49\x89\xf0\x96\x53\x55\x48\x8d\x7c\x24\xfe\x4c\x29\xe4\x4c\x29\xe4\x44\x89\xe1\x66\xb8\x00\x04\xfd\xf3\x66\xab\xfc\x54\x5d\x83\xc8\xff\x48\x83\xc0\x02\x50\x50\x56\x6a\xff\x51\x93\xe8\x49\x00\x00\x00\x52\xe8\x2e\x00\x00\x00\x0f\xb7\x44\x75\x00\x8b\x55\xe4\xc1\xea\x0b\x0f\xaf\xd0\x29\x55\xe4\x29\x55\xe8\x73\x0c\x89\x55\xe4\x01\x55\xe8\x99\x2d\xe1\x07\x00\x00\xc1\xe8\x05\x66\x29\x44\x75\x00\xf7\xda\x5a\xc3\x80\x7d\xe7\x00\x75\x0e\x48\xc1\x65\xe4\x08\x41\x8a\x00\x49\xff\xc0\x88\x45\xe8\xc3\x5f\x4c\x89\xc9\x44\x88\xf0\x88\xc7\x5e\x56\x20\xcf\x44\x21\xe9\xc1\xe6\x05\x8d\x74\x4e\x40\xff\xd7\x99\x58\x72\x46\x44\x89\xf9\xd3\xe3\xb3\x00\x8d\x8c\x5b\x00\x08\x00\x00\x8d\x5a\x01\x04\xfd\x18\xd2\x20\xd0\x3c\x07\x73\xf6\x50\x3c\x04\x72\x10\xb7\x01\x8b\x45\xfc\x48\xf7\xd8\x41\x32\x14\x01\x30\xde\x20\xf7\xd1\xe2\x89\xde\x21\xd6\x01\xde\x01\xce\xff\xd7\x10\xdb\x73\xec\x99\xe9\x03\x01\x00\x00\x89\xf3\x8d\x74\x82\x10\x04\xf9\x18\xc0\x24\x03\x50\xff\xd7\x72\x0c\x0f\x10\x45\xf0\x0f\x11\x45\xec\xb2\x5b\xeb\x32\xff\xc6\xff\xd7\x72\x10\x8d\x73\x01\xff\xd7\x72\x1f\x83\x4d\xd8\x09\xe9\xc5\x00\x00\x00\xb2\x03\x8b\x5d\xfc\xff\xc6\xff\xca\x87\x5c\x95\xf0\x74\x04\xff\xd7\x72\xf2\x89\x5d\xfc\x83\x4d\xd8\x08\xb2\x94\x8d\x34\xd2\x99\xff\xd7\xff\xc6\x8d\x1c\xce\xb1\x03\x73\x11\xb2\x01\xff\xd7\x73\x08\x8d\x5e\x7f\xb1\x08\x83\xc2\xe2\x83\xeb\x80\x6a\x01\x5e\x56\x56\x01\xde\xff\xd7\x5e\x11\xf6\xe2\xf6\x8d\x5c\xd6\xf9\x83\x7d\xd8\x04\x5a\x53\x73\x6e\x83\x45\xd8\x07\x83\xeb\x04\x19\xc0\x21\xc3\x8d\x5c\xda\x4f\x52\x8d\x34\xda\xff\xd7\x11\xd2\x89\xd1\x83\xe9\x40\x72\xf2\x5b\x83\xf9\x04\x72\x43\x89\xde\xd1\xe9\xd3\xd3\xff\xc9\xf6\xd2\xb6\x02\x01\xda\x83\xf9\x06\x72\x1e\xff\xc9\xe8\xc1\xfe\xff\xff\xd1\x6d\xe4\x8b\x55\xe4\x39\x55\xe8\x72\x06\x29\x55\xe8\x0f\xab\xcb\x83\xf9\x04\x75\xe3\x99\x56\x01\xd6\xff\xd7\x5e\x11\xf6\xe2\xf6\x11\xc9\xd1\xee\x75\xfa\x01\xd9\xff\xc1\x89\x4d\xfc\x74\x1b\x5a\x8b\x4d\xfc\x48\xf7\xd9\x41\x0f\xb6\x1c\x09\x41\x88\x19\x49\xff\xc1\xff\xca\x79\xeb\xe9\x8c\xfe\xff\xff\x4a\x8d\x64\x64\x30\x5d\x59\x58\x49\x03\x41\xf8\xff\xd0\xc9\x41\x5f\x41\x5e\x41\x5d\x41\x5c\x5b\x5e\x5f\xc3\x00"
#if defined(__GNUC__)
__attribute__ ((section (".text#"))) const char stub_raw[] = STUB_RAW;
stub_ptr get_stub() {
    return (stub_ptr) stub_raw;
}
#else
const char stub_raw[] = STUB_RAW;
stub_ptr get_stub() {
    char *stub = (char *) svc_alloc_rwx(4096);
    for (size_t i = 0; i < sizeof(stub_raw); i++) stub[i] = stub_raw[i];
    return (stub_ptr) stub;
}
#endif
char payload[][4096] = {
"000087-;|iME#Z^UYE&9xrUCgeEM_ZnyG~yppPfI@=F!Aqc&bIK%mog0dP)y+<9d8%9b%IRKOQ2sb_3Ng$@~XM=&Evs5%6Lq(@x^3(Y`xbsEOY7YDMdX0487v-=<*5p5ZAsCYA83>S{VJ3L6gPG736zizf}{Ia+`ol~gpA%(;@CHM;tJy&X*T(WZ5K9G5e)_v`nB^&>L0j+3_rCncS_>u3eq%9Sa7a4i!z)b+}ON%lD^PuT7kn7B)PVw6fi%U3qaqBntGn%mpK;TV7n%yw&n+}n`6SUV)=wxG$x1FU$U97O*UVb\?Rt`QR0M5qU)E-rnYZS5Q$lzqy\?3#BrL<-J=oMutrKU2GX<6u$z1FVBkC>Bndm`;Am#{;m!}sDM$J=skLj_Xd7;_m%t;8xiOjz|L9KpQ%|<aZ%xi06CF!ww#BPSfY\?GCK>-^W6fJC3j8)i7~Db*Ml\?E|Q7Cc);Ut5sgZiBYkb=x+QIAF9PlpfyU&(9^)^g<#-canjxfZ(xH9Q2adNP2d;w7<3UW04V9l}1Yg&Mx0Dim@^kGZ>b4T}=WH%gkM%SzERWrNl&_)iB%9TJ&|g-(t9PZ8cOp~2kbPnEf_)8O9j485K4X}+%`ewc32$K^Bt&=Am*J7\?YvDF5LlxmbH*JL!}Jk770*-\?%nuxN6K!^C|J{oorKSK)jHFGoUF{7gyl4V{Bhq5=C~W9FU7Zj@oyrdpPg1bHMw$0qF2k\?V-hbpYmz=V(1qQvppq`=&\?k|%shxT<|}-K&)Z%Bl;8sMwcg#;L\?0IJ3R@BSLjOYg`TkePpDovNwG~gy\?JCJnkhAhN#-XT&=Van6{qvvpevvM56u$7qbzB>{$V{Ji<y_P|o{01X)0=hXmshxBPey7>BB%^2zQNB@c4oB4#\?kD4rekpNpE~BFS-nqf^14l`H&%xLq8n2okEAMTr\?kxRD6HPm1IQ2yOS*0`kHBqD)3whEG@a<0)xK#MJL{OOC3(JpyBC0Rm4*b_S%*F^jYB<kFttx\?iub8Key{^KfF0s96emnE7i<K0k0;KotN(Jj-Fy$<b^J0roP@MP4>)~p%fyPcGSMX^31+t+TzO2zJ#\?XBmU5C8JT#M7J$fS;-GTjhRCfW#1r2Zwj%}K<gxd\?j;oz@rz9iv6FY^z$vs~jq!i*z(@3LbD-4&B_9Q@!1lcMYMGDT$VZAt~>mo)cc!\?VWb^\?$Y20U#<1*Of)~9bKUSArV^G_3(OO^a75g8X!d<x#i|0^re<z8CJ)pNUJ$TafDi$%3^-Cp+08jQY#PE=2;%*o*Es2h_wMj_uXrew2y@Yi6=mNG_fOpX6HExU10hANpzBkon`m7>GnbZYlDhq9w;l@jWVerEhWA3!c6M9Q!LXp4WIVmHhNa`0{VN-nO^8RMRhP!_1S%XAytz5_oaBROT3{daqL4iFu35)wuQ}+\?I@\?kj~HUXg!Mi)+@6_(Tq${^9Vbg(Xk~4Pw|S=GQ+\?VqP;I=GvDCM2WPYYD#Y65U6<=C*cKIVk1yi}r6yPm<%3`P3ZJPg9248paOMB5wE}Dv+x;S`U{DO<t`Llki8UgtDtHZ4xRc)25)grEha2Rk=av^0A;=8fNZ;Ld>G5<|v-UV2A^`fCBTR-Sh<8ZFUTU\?Dg|G}DsXC~|gLss3`;sr8mZSQM4;B;koKQUa9Q`8L*GCU$*R0Ac!BF&|f7miQvzx1s_8p@M_i4;t~\?1*DJ@l*BkI(SxiAB7DGm36wOV4@jc@NN$~10{;~rXF6=8y@&zr>}ehSO}BNu\?;Q8X4!N@UTE)#GXcWOuk\?DLL@j7-LOM>LNH+;PD~C{Z%S;(B{D#-!`5`evO7bM5mRZP+#rr5+YT{xudt7I~L=^@Jt6LX7I|b-5cu2tS+~I!qZE3&x(^>4lJ`upMwGjLdB)os41V-ilEk@fIBD$2OXbBjVcDStnC@WuDVM{iD9@EUI#DmTpuBUhg!P)YwgyQ#~#8=^0odv7!aPbrR4Rm4ym@0C3NMmI_=>c&CP$qr&-ACc#={tJrWtrH+&7ai}r9jLLn4FdfJ!`vT=zBClyCXiw&VEZ49{JOR0xsiD=^^Eo#B|kW2OrfJkZ@s;Eqx$W-5opOeitOt5YU4vy|CtErl>ksS&4!$bRDV!#D=W0c%8PhOKH+O-qDmPQoe+)8paZNm-y_4X&MX&lLIg9fpVG^UUEdxVAT-kJO*ZFYz+tle)yH8PMPhhs+*#+@`&gwuyixl-MegDJJX8G7e<xsp^{e%9nq28#ea8JbIRKn8y4T<iuEl2nu1w)K~OgZvA\?Ja$y<1_JDa5mmUVja)&|eZ-}^|D`xpnmtdg`bKKlpM!0nZnHu>\?H7r;!+nhZ@35MIWY5kH|AZ&+b~)B|*}ZSyOm;$qaX7ZPp~>SJ+@B$>_GL-y$5V=nmizmbhlrl~q#k=i^$$%j5h))j|l&5SGU6Jm0QpiC2SY{2TlGc|S}s5y7QbYjvDL<S$xi5o-K391\?;_xDDE&<Hq=`+>=Ht\?FUge|9OJ5qnSp@}lmbAMraFJs#TnXwC08(PHU-0Ai;K3pfun&}bxP{~*Po%m_#{l0>^zV9M)j5<n2|45c6~-J8))c~^bHiMw^O_-WP+sUGekEop\?Mp&)DIE^{a(ApSeS3lmEAq;7AL(2AN0LXd5=nP5b~Gc_{3&|QnG(euPVe(t+oO+kP\?u)x~<>YMvM=ki~r5uwj<t3EMAqP\?OV)w+{1ZNLpZ*@;qyx*3m<!@8crG>aWnIq98>Zvz\?WJ;YCs9<B$#feA\?cnv1O5=bCTC^D8Kx6w}^#pMjJ8`}sh;MzkURE7b$|B(Rg3Er>YRlsKM)fAOta1%vg1sa9X}+#|sO=jXV0&RWR6Y)Y%3>_&i>3@Y-Xnm;XB4x^od-c&J<yD^e|IUpR5TA<_en_;ZuNr_S|GfLdG#uvB<)xQIbKv)Xt%(-WC#3_JKQdWBE<#;~Ot8s<BJ5Y6_*B%Ln0l$84dGl@_JU0t{@JEdTZIk5|rk|S65hI@sEmcrGFvJ8Bb;fWSiJiAj7#&bjo4hoq7\?kY3NuRah)>$IH-}ei6LlbBsvF*\?GuU~*B0c#NLyoo*c0Kv{ut&p!ff+oV4d+)p><+UouMiN{)Dmo_(qVV4=^w(Xidhj;U%^85RxXK67hC9tK^obohVXI>m=24c~KjBo_3Iz|_-$OwA8SHU>zHg%3twh$j%f0UwYDw(N$zX-7fBF!raUTS5|C*XWM`%L(gY}Xd&D%JrntuqOq4L5^19p>u6|`)x(}es5zK6Hk7fwttP`d^$ou7o_qWB4<YO8MlzZ97uxhhom{ge&i++56&LXC0*TeimUZZOoK{a&9Z69bL*32m|9r\?#U5`DRScBKP0qZ;+C<\?e0xt*qxZao~OB{${REeh(v`v1ZTQKrjnKEkmACKp8dV;z}=peQEQb\?0$x*{_FABJQ&}fTug02n+~=F`GM$Th#&+H8TuAbGWj5UtD2L{iJs->~(z=%R92QzWeGbYiLz*k|`GD9<Ss}v$pBO8|I>aB9f`a%XwQiUeS=nI5o-md4-8ckonvTZH2^t+La}F-5g0;1aj%*HZQm51K0}N6^-{xae5f0nx<OS7N+V8GC;2OI<X{pPDzh3mBW<(`76ZheWKQt!UUQN!A_G*|Qw}i-gE;*s0dDYk}0W8Uq@<RBL$Ua%+H^}w;h_dMxkJM>}FXpFUv`-EHp\?&KTX9Zn$=xIHHXG1NX25kqsSyKcp8atEOH)f(i#8pa~wz\?CZ9L{ETvRNGk5wnH5O*-qT&trt\?tIxk`JER-nSo4-3OTRh*c<Blub3@~6#r(1+hBA8qQ{ZT5)#gAzJJw{h<H1BXTb2E*B2Uv8hC8D_m#rPsDS+bZ_2&o@>z;\?Q3Nw4kf)^s_AQ)qWPGO%2k-I|jUnhzfEjYO`On=kT*u=IsJWMZQWFq46UN17y5P3HEgs7}QhY$|IfggfhEEEAP<VAnUeVRBhD0MbDxVVaDuJg6(#k9cev-oaV\?lzW3KBlnmLS-b8W~Nh\?=9Ya#=)r}wCI7of4%rY)j$4(G+0fm|J;)#>v6HQ%=Fw^NFlJ*PQMkOrkm06ZDEw+6{Y)3lpJ_}034pmofSY\?m_m1b2AjfG-(SQn%Fk7|vXGIq*^C4`-R!veV0c@T)QU(h;7L&(Jz4_kF%un(icZcqT",
"Gb{UyNM2WEUIzY6-8EU7<WUWh&yRM`5SZt^y>>daCVuFIW@ld!`RiM1m#@N3Y-@KAhd8A)0R%vnp<6gRwr6W#nt;cCvP!t@xeu(JPa+=U@Y!Ig4+R1Tbl=G=Vr-_F^soFx@|Zjd=J\?RzRJ@u2cutnlhvk27&5d*agREPIwG=2|04Z&R(s}7shj;}mN2M0sZ)_6gBR+}#&xA(iXmpy0&O}!|&SSjFGQ@|-me3dJ^yP(((28dEh!3uirqY7NI3&2Mj\?\?{bidq_;L+`0#9L2DIVQA51QmY9;ftu-}0(2vLDX$8A`Px9ydIj7a84&@pxs}I#L;c{DV&q<cJSM3x5`I5Wvf(>62gtqD9(X(~W_Tj_eq7nyI1BZ&WS4!6\?%KUqMj6qb;ZF*kW!1c}>pE>}-GM#wVf$1;g5LRABo~4YA^<aEPZ(y870\?p<d)sHP\?m<j5BBnd-a\?E80Se7@onnP#7fVotF-5`@Qc<cWV\?+mxK>ND2pOLmG<UnI`ZYut(p1fr@WOdpb-_=Ov7ZwsY&LU4N|$br$ph)(FAYYp1LsMOaoc#0ucehOGju^>Q$WRc\?$I_HSAbtsQ@w+s*u2PSprYGf<J#>ibok&xi8ZVTv%8wT51Iw6+RgI1`;SIGPE+Nx8U$+s}exV@OkvjuV}r=o)g77v-Nz#RCTso83|OcgF@S*#WS4p`De))wPK&L-~kaGIJdzu|W9c%~XzUT6%a**9ANDJLJ*jxZs<Wx{<Q!A$)wREaj0IL=}LH(9@6Lg$KywK{}VpcIdTI~Wl67\?Qn{@Hc-WFPuHQECW4lj^ty-(j6C+-<u4XkfAm}1gG#^@a8)1x~`!t1$X4w3B\?8r$hquKmslfg(+2wOcT8>iE!>ek^4E`J#h8a9RAYke8$oFrkFo$oejf*WeY_G*q5Jl@n>GY;g(sX)+$7&DPIrOaKZs;E#Ze`igy!1Bh{ip;bw#c`=Q1{=APS!%nK$mcoRksawFfgyK12kY5=Y1Ei_do5SH+-{e`m~$0#$|v0ECTrDl@!ELqp<NFN|;2go1F)X=}H)K<XX4>V@fpVPzt<BsKe<<6EgX#(OwuPzwV<N-AyT7;}&Vd$L0}v;zNST(ghuD%r>!VZg|82J*jgzsO6C4BHY{V_>@Z@`X!kaznz)nANp*ZVL}a8I%-CH-jSKmin=W@JCr{Ry5X5aDvuPMJ+NVH@mydj(;(xgTDsyzw-1dY1kvDC-K5@s$+H(Wy+tXR>rtH65|5&I|PzUUl8<8`$ihdG@TX4fGoy9LDC=XOu}C`h(+lbQ26{#-W+uf8V|Yqq|N`g)$5\?Gj\?3f~XIWlH<Cr>oJ^j#*OcQ>\?tTg4MbkX{**D9~FqB_^snNZgL>%Zi;UNTis5+jEoVSgE8ukk9y1BODXoDk&S<l5XnuL7&y$jf&Bz6p@wr8fjYH!zK|(xP}`i6)!UD0j&@8_J%~Z6R4zLO@S^PfJS7_xL*RNX8ltE8%N#ONqR`PW7mPRCvtP^rYvWt4I0{3}JO~RKcMP#XkhbvVXY;ji0{Hc*kwZfWEU(6lSI(o;SaMap~>=<H)4i-Y6lkt|qJ!|C{|\?eq)rbx|t@VIEeRbF132eM@*\?tci52oj5dvA{pO+29vn5z>GZJHJn*bgebt2~XLZQ>uhGQ\?3Lt{+FJwtf<I2aVc%HT&t7)+-b3|LuFC;^hun1|7-Y2M+AGu+0%0M*t%nTGrT+eKAaJb`QU@-Pp(dxKcJB*s;U7$fy`d{r#nDwD$oq&gEpZFCW`FXeR#P*B>G6XH)e00OfC&6wGuD`1kqQz40`DD4{wuz6tr8HLyve>4=1O0k>6}sbIj=s\?FBhuRvXrBWRn!GAKAf*AxEhC`v=<^J70DgEtrP*I!3HVbFOyLzssm|2mb9Po(ho+Yo{taNkOFYx{)bOg5CG--rs}{3867KK2`JeGhwRa}LE3Gwt\?BNRXg8h(66!umJ{~9z{t7x4F4Cg)j5nYNDE`%&ZTy}QVu7t5(e%7mSK(Ap*AF3<=f0r_s5%pF-5KQK(5X|d^!HFz67#tmacjZ*#VCHggclUwgxP;xU7\?8-218Js}G\?ETQdI=6GAgaF\?D4%t9404^EVC4o_@hv^h6MKhS4Jsw|GtAGhl#v!Bm4#\?`baC;60s7pQcjI9;H#(~gy<sj6JU9ywr0ncGV!p_^Tsp%hq|7LAq#{Xaif_Z{sSBgO;Ot(<MC>n\?x6TdwmMa9o;hHMFq%%<0k|qr=`NQJw7<A1V3sJX@b2$|4XVucIUF6S\?<N=z4c1~sr$\?3B@_y9hC\?+6E6T>UIZ&`63+47u3+fg8RRs|$GzRG20mehX(T+%K!0T@WxX!f%)Rp5kTRK!+;J&ix=&3ihyWJ6g}^usd{<-N7$89T*XB*Vq5G<-EgM\?6xfTsVKa*09s(RJUj9`2C\?k-|1u0lUD*!|Dk64RbvG;P+bS0uD@l5M(Gtvo@)CH6p9VxGT{MO33<7v_sQZJ_8as<$k4S2~3qu_=Fi#|^h1<B=#PJuhMVMGi5FHm>2!Ni*qRNz<>wCcG{A9i7{tQ0G8jeVC;2DOtsn(O&M6T_GA<0\?zntlzN-n{1L(7=y=y<=}hC{6$&(ATvh`0o)0qazJ)32G(X_hHI7MdciVV*}w#_T\?1!$TQ}Pq=N6Q\?0wy6xq7};\?77&U{Y<3@bRQttA%<SF2Vg&9`&c@cYEUGbe$sqdb68#(YsOaOl&B;>2S~dvcO{L\?5~k#&A`}IQ^wuIjy28gdZkhM_sT{0$1{vIH3)Lm*@DIITgU)q)r^9(Xb#pjuNzG;`A~kUg8xi<NLjLtH=Xwfg)e2G!bg<|H@!EsJvG9HPX()3!>3}TbZWT>~IUvCHM%ho-IDH9AF_ra2VRF*mQoga_^ubLc`<G#ok|K%1tC~x2=l@CAIPIzBd6-Fq_Wl_&M=K}28\?Ez1toLAUFQ_bLJfn6e|9DOx-@709Spkeb01CnAAP=wWPLU-5q2(Ucl(r}b_wT0Z3WL8*ZyaTJJ|<v6qyn#(TR<C*{gwD4Go%p`BtIplhyNWVYf)Q_gIy+>SiD(z&<C#DU|K*<(0Oy2*Zvq++=ouO#F\?i}2Iwk\?wt1hvtD+Xc(a#KffC<UXLs3\?beRJ!o`|s\?g5h|qM^f*dT$<H\?j>w\?PHIjJxrP+o\?_$DT|>W<Tbr(*_mV`L;*>s_x-A|1D55#+62n;5DMCR#t=z9SeVAMPM\?`P=uJ3E%VE3%lCc~($I=J9=I*}fDx3XeMJ(7VgXM7d73q\?uAw7NF+x%N%|Na=@Ve4}*so_QEC6jIKY#U;D2&F+n7UC&S\?<a!&5XCj^}BuAcG2kgAkMDQtU;jVISzU9yRPt>x\?_g-0\?^PMj3dyU0e2h<d`ITmH+cR-IO3!!3t^e5%bG6!hBC3gnR~IS2dVn1z+Pt(dZDlCd&Xp>pxVnv!J!fi(cyW-gR6y9X<9hEvf+=;W0O%Z1IKW>{RN>Q{ywQoONCyVh@!g3wKDwyKEA{mWR7+I\?eCw9@1bIacc#Xj1IZxa=8OcBBqyS1PCkeKPe4(9CAs|HzD#F@x*brSP`f`C1diQbJN)cqrQ=jfV;uVXmhCIOEFx8lcg@^(!L+MPVED(djhgP{dHzGsLPN4\?ez3~uWYKsS2*|!11BZTzzhAvJ!!#)#wbW>T_874}b)H)*){(W<59q{onj34X4s;ERtr4~~lkVw_k`gm%BJ3@iovcDfSN7<#F\?=#~y@*rN5FthWi|Hh<I-jh(1ne(6FCD}fSkFYJg&~i6_o^r}PiK#xh18z#fb09>gBipF%vbfw0omDH9k5Gs+Ynll^k!wVfSg2Ez}GC0m!UPqFO7qWi{-Mxrqw3w_Aho3{r}HYB69R7S_b#-)Rf%UxVp4!%b@|@7`CJqeAdanpWYKjb0WstOxt-VG})po=+n~4sf;L!p2)tYU($sT#{A4pI;EPU1)<RWlZdJQ(9!R4wf~DkFjip%xLUM3EhOxXpty_Q^X\?9BPbgSmb~{Bt1Tj_l(iN<PwfjnSy!<z5cPm}|9>cW6aHqcf=|_}74i`jvnzM{j12NZ14lCX(Ei$*=4*O0Nr=8pB^Iv-L5}6F^D%~<Ea`L(bFXw6+&_N\?`<j2w&(XLT96%\?`^{QnvHKH;xb>XUje{1Uh9lqbLApy8gfNwc}cLf&k#ckJ!Gg(}Z;",
"r>PSe-Refrym5lr`5e5M<R;$*`$FBQ+y#=ULS-aaT4$5MT\?>B&jKn8$q}^jwfPveK@BbpE$f6W3n8VU)`qCGGXt#v`VM$leGu&XV9Y;\?@X4UzBb`>bHTi+<=4sjnKkP{{Nj|0AO0ywtKa!RwKu@`LR9;W_UL%>*c=Gqv=x+mqihTjPWRc!pLV>sR=pWH*~7UyFWoRfw14uMnoCGtAU_-E6!6Nlh)@yz!I#CK=nLtjKl5gUhUI|CUkJggvkd;nEZb~I@e+1ZS^K}IV6TIj&%UXtYNi~s7rp\?BzTuwGl3ty&!kX;L-4>JNWjupBX~f{uhgux=K003a_altjwsUCo@-A(zp=Dw61~fzOf-_|-x7@5&UkGh7Lw90NR`Q#jq`sLG^CD\?tDljgn}T^o~!UiK~u+X*pOkz3Py8pkd>^8ZSPGCp((%7e-3#L~T|{HQ(Nw+7La{Axc>eFQW}M#ipDqBk17h2JkJ^y8~m#^R+2sanVjwPnjmk<{Zf}hTRz@\?Y$=Jdfo>WV)V`!jX1Y^sqGd`e2M*M*iujkHX78)Qk!m^@#IAObJ#N05eRbcUKDPrQ^Qs0=HDkCq_keOU6`{0\?W>&>F_N8++v%iWdrSoy`zZ<Pg-O=p<}xH8G3HwR1~O^4@a=cGHx5v)AK&!_Po#JB30(H~{#2!aeD$;S7JeWu2GO|i3-Du0k4IMYT<g9SL<|*+<%6%`bd5=nG7IByDaXZ\?1JRdzi5oh%Co}U3q%qS+-hfta`ERD@wg!zXus0*4*_HA1Ld=@1t#Mu)cFPL88vOU$42-I>4gD`6\?HXtxeIt3P5wiRA=DC!XhhpqTD804Fk5!Wz1neHIaA;kjskx6RD3D3-nA8#!)4j7\?B2wxdFUxVDl03YKG=7xsg&aXrZ%OmZhjT9i>Y72ezgh-4\?Oi;Ih3uz)W`oWfyb7~^%b=kY3zN_uC{7@(48OBe\?dREZ<k5X8;R$T;In!\?!9eW~)%)dI(82TYWv^`n(iR~)B;z2g2dNl*~h8dHgiNl!(dxZTmiGz;I8##QwKR{9-mXfDp#>1!hBJI6Y>z~ELQJQTjX}an{x{qlOyiV|G8(H$Uu0PxJe7C`8QqYrhB%zK3>W*+T+IoaeYPII!$5y&afVNPO8XZ~!WEVm%6qcO;Bg2V6A>#p#t-)k@egCp_obI=euWuty&`;BJveSPjlGD+KmD0zz!lvI;qjAGwHctF1zuS|0%oAFPi+@V$l(dO-{F^=(YmA^&7;C(QKJVj;eIe;bZ9Z(k)2qa\?#uzV)Fn3*Sp@^v=55qiSZ~t%#J3c<qaoji<8PbG@y_)}#$e>xno<eoio1}bCwSqs3u`\?``h^B@ZTz$pMD48P$x`)a_+BrNw=pCxRZH#oZ8n6<lm%8;^c9z0*eK@A+j0O|7&a+gWrP7jlD#F{Twy-EZkNUf;n+WP7HKZM|<fivsIgSxe9X{dAimUM{@yncmSvpDbW!6*t7E+~hX}SJ|$\?sQnO8x2Mc5cZ=e0!bG7pMcHg+Z\?sz!kOAo=|hb*Um7~kfba3VA(}\?V93UEU>&1N77M1!!^`<On^>`hg!HhJJm#LJ=uypbvg_EryqQJ@JxqE-Md)E)o(NZ+QIAjT@Zl#bjrKVvBrUorE@7iJ&OzF_a`hZ|=LuG_Y~<5oSXtKBy|\?Sc9Q+KMgA^8hLuq9md%iHlQ}RMI=N#Vg6fCP6jJuFw>_5kKbiTU>M^uD<w|9U1Rx+Rwg614d&EnxG%Lik~5g76fv|{8-@SR79wD&cB%<}fm2%iuHhNb5usad5lrMNdyqS+Kuk##W^_EeL461g#YlGA)j6c$P)KGgXE02;cfDZ6sxM5s8tmFd*}s%RL7wviZ+)Q~yC=)_|&b*1x%$_%U6P4Z$qimx=YBg)0M=euU_V0HRb$D-v3gF!nR4xuLJNiOhANrH06tj+X+!wP0Ik924xO*Dr-S`TulT\?iY|rNObq^-rKp@i\?xQDapH9F6dVQ|BQOpu!NG8j@U79t&h)1ynfh~t58_!heVq%VXiz\?B9$+cKK4`K^eB9##S}rXD>tDo*\?ch-Kg<\?6#5mLo8{xpmMIPvcYoX}#wokXcTr2yePkd}e!%Z5nA2V;PkWG)@*>u$^4$yQB{CZB%&FJ-6eWA\?@_XA`fdLIG|e4~0dN=+Eg3*|c{xC=8c$nDtDnj13p-_o-^dy)}QjO^g3hq!@5K3fvWt6J_1NTzJY1SFlPaV2G&y{3>wPcvNI#w}n>+43-dAxVy|-WE2~l%#EzJm`;VMJg&istr-xO3Wl(XFV>(TndO{8tyI`6+Ho3Q;oP@AH4nFnd<gxHze9bGU%7@4clC_eCW`dA&V+2Rt^$<yUc!5*FQ-s(stqt{\?X$UR}UB#isLIJdemq0@-*KlB7ao|eMfSLfE>R@MM^3TjfwbS!T9JVXfkkYWj0x~>+Fi*QYOuF&5|Wk%iJ^Om)Un_@9~z#HfrdQETlDY=Q07`IN~l<8pyXT%w#ty3w\?nsN1adqP9`{\?ip4<=-C`qm1vdv=T~I49S#m8+V9X==$$BgcgDpjH<QL1#d0t!vO}4!hD~aS((Qz5rBIQN~-cD*P9*Z;xv5hOHNxh2rITk6w`StukJa8e)7E0>gepYMIL9$RPyl2ac)iq>G6hGqO0eB9+AiiH+eLDsJsi%}Hf4*Z6(;_p`Rjvv=tEW<dLwM9Cq<(ac1pqio@@~J>tmgbVf1&;`Pm9|cS><9EDO*AkqVkG@fbY*;J#UpP9ibhDBLu^Vub}lhrtR_OQ8i+4o~hJOwP@&g34K{g2VDTjdhRIGuqxay27r~5U@(%JXJ>SdqmbK#N@&EU){b~7{7Og80cma%jI&k7jr%iN+y91)kE5^L=I9hV@bvQ&V\?qp39LxP}4gbUz^d$r7*kf2;I6d9NP--Juh*U6*18#kMY~&r2aQ%*lzyBz|-N97Qs*q!X83TdnvSP4~F=|l73g&*DL3NV(#P(HJy-qWx@26Ffnplah4#uMv7pDjPMDQ;WO^}(b\?Ih4j!$fwjVJHe6-#@0$y0Qt(>H%di`~sYSO_y3#eP-7HXF;n459PT0Y5`_cxkf$Vaqn}zdmJN&F$&WO3RG}+N`4UMG8YrYb48lWs*_4F$-gh7^*YSDzJQK3miVC^9n^ufp1j`b*lH-tT~y+!`E~Nj0u}$9Y4M@|`wYvWO{u8<I)f`<<>yuV9q0_MS2Q%3bQce<jW(|mI{P!dEq{hqc@mg<GnyY(6i;<<3y>>vV(z1Ih6p-32N<9^vEJ}&wl7TB^NtG<YvnWrw~-UUqNc#A^F__4^(9Y44+pLw$1VU%LU-HmN*iAc=s9P8h4&@d`(+y88{MP|y|V~*y&cddBfLU(lwPe3rfK`0jdeKk^pz>x{K~e~U=vbFpq{AcP5=V6-g1xPSwySH5x)HDm+m7Uw>;}+Y\?OQNAhS3&eSsHF)CjVzA4o3JPu!3\?49~nFB`pv3p84@|U7j+2jJPa}+OSleu5!CDGVy<}hkq>wSDl_\?>i2W+QGnwugV=x!e&B8x7rq3!*bkBkofx7ph!=+apz>i(al4R98wp!`EMgz`QVao_b;b#Cl~h}&p<53;R>8`N*x|vj4<iC)Bg4~Gsb;x0a0*H3!wyE!Mm$alD$DE<f~JiQ{mH#3M>*PzuF<DL;a|0\?neO;U_#\?WCz3Rd$rBoXX^^m_KFh2U!yOxI#05b=p2fUo>4cv{Ns~S!li<#4U*I8Nk&|#{xq={|Jzo!&2#MT!SQ)fWL=\?6N{VbPw`Cv35|bglNDyD@Es^Pohm82T)@%lOp*j0%7e>2Ggap3K10Ib5mxrcOnGc<p|lRrzA35|C@=nH3t;921xZS{Aq%35_hlXm3!L4i<h<8^cvHbI5n9@nEPt`Zpk{XCpZSE$%GN%E6C-QDop~Qe)T|MmnUNIv6>&$(-^zvISmXLAY&TA~LpXkzAWv%Ze$XDpSWSEZB)fZ3{k{U@cn-sXs8U%16sp$n<a9(B8+%M_MW<\?AxuIimWv+2P#K%#UClq^\?fF-YpB\?B\?1J=xw<CN@$R0XoewRF%s(m@^d%#e=2s%B4x1s+aWx!KP@xjVoeK@dFuk&LpLmy4+kY0(9ZDT#qvcQExO>Bn9gEKx-sE7a-N*K;Q#Em{Zq83O6x8cGlq1<N`39|6GuE+RzeIQdI7!I<s",
"l>Xu;Ni@Gs=L+nPJvBU(l(\?@^h_O0fX(2T$*9Au4|9B9@5~!Uvk7hz\?9^jMXQcFU@)Z2;lqP5fPHOlWP_GUGDPHyS;s0W0A-@~ETIM$3ZT6FIQpsCVa6X\?|V0Zu`CV~&PCax#KqFG*ilR%Rg3e82`$SgLd3>i13}(DVeei=>TT((vDho#te_\?EN>QZ;\?Wbl+8MI-u{JSC%SBq2^0jqe)_unIICP!e6mR~*zprmNa`>Z08SQDiJ4Bg3l0rw@;gAurr_1wv)<pdFn(Tz>+pO1w1cQWDAv;fR}T{UEwzp8N7w=-7a`S1JvhLnhmZ8W0F0!&K0S{B`M`HyZ>B@Kt6<mvE&gwWGV$~\?fa04*hX@&V5e5*dc;zYG;nG\?@JNmyCNN;aol@ZsjzWXB57DNN05~TCvJ)-WUXsiVBHsv|s>k2Hq6+n-VZOL\?twjO3cs4P)GhO1YuK)U%3v|7h(-MG4flX}Ua>sOi2uG_\?RJQeyit$IUkl$U3YlUC_pp)0)$\?)z=0$7}me6d\?&CD6rOwwT&(GfVWOj4El%syb|7u=e%=W<aMqfcW5@T08zg~knB_HCOOwBYN|RMn!3BAf(Sh3Z$Xiaiu#|oIvgcM!AJYUGfFY8OPk~Du)QWPhbwCdn|\?mOC|@5HItU-U5WTItrb~#*^J~VRJ3II_bqv%V_rO9bH4*Y2na|%_0mgp7*!JrYD\?XhF{)dO20T5a9l;oWL(Of_j8JjW4se\?pIF3gXG^#|mLEes1O$C#gFcu;G>t$wBY)Wduky3z+v4<y(hcK;NPs|Q<46<2kOPy%5XklEnZYVKTzTNTAfC*GG4DPy5F4pKnwdKOklnz2Ci7\?X_SMp06n6MVNMmA-HNU\?ye1ZU+05GDQS}fLD0$LF%H;CB(}I+tmq@Xs5@oQQuBXl+94X6Ew!2Bpu)PnPZ}OzuOOu&qYFa6Ar4bpoEQACva1ZKaQH;`$jM6BfS!M0a!K#WA0<M8Ph&#oDk;`Y~PvtDlv8JrQu``CtgHc<lG#l3PeI7o0)-eE=Mg18nFc\?oj$@E*%1HlC@&PSE|kR(sPY$P(E{!V+vcu`wLO;0ekAjlw&NzE\?ycwYv7$(1^OWaJ;v4!1L@w$=p$Q<C!ElVkl~RxP=nJF8@Rr1cYA~2VJ5ig;mx+g+E5vnnn_S%TiQ>x~e<CV3s5yquf5!RMU|5VtreVRxGd~_;XwCZyd)x2FzX2m4^v_4$$vX;~gH2>$^$k8df4Qk>NH6ULHd6v4Jrc^k3Ndh<p+Xe92$`P!E@m3Xqy(81pY0%2tBJc)&sG|olc==R=@hG(O+anzDBWaDK8{V-x;`qWL@h98o04>;{Y~yf000s#<0V2(Bg;{eZJPG<m@vbv9a|us&S5|Zi#cO<8+!|NfnFS4mFxMa$gUW5Cl>@RqR\?AF*5*Z&@ItPPn9;Jb;u~*<&e#oZ{ICUl*ptF%eknGP+kedEl2YqVddl6>$87##gZJ<7hgaWEtv-$4iD!D&XA`RaqDp&CjLK|>G8ZdiGUJIwrq)uU0|JS(6wROYB4&4)(`F%63IRhP&`j3SFtZTY_^\?a4Sz66SkCw5K==_gLkG-j&Q*L_#|EHIT&=r6<r39qqpN!y#xH`|gm6LJSg^Am(tbk>7K#~lKSOCCFIKoj%>XS@S4EyBc8wnTr&g%5=E{XKY)0\?c%t!=7rtBquvI3~+kjc6+|Mu9xU`;i7MDw;R;qucNt@O|)^2H$bl9ZRdY1X2C2ZLbOh%GlcSZ*|>;4mqmHLX@adD!wkvdn%ReiMzd=GFtXD{BXray%nDsArGZq0A0>Xyb*9|c!meV#)|x$!X<}GQb<9zOB9V;KHE\?#pP<+KT$0$#!cmrki\?-YHNvz$>AOpWv&r)UQdDuJ-Q~n@CtJSmmJ=A$Ty6%jo@}AVqWh7qM5ZE)_x\?13<DLbX3&PmQsq=G<YG)zdF@EWkstu4z*jl1k!O__HpXgnj_rb@MzE(CnwyY^h-4O<_sfc9L_ShUKg34@pI(Je79(ij{KM5a}qYg7Z^og~5xx*9nue_y^^q`btlt)rfI_nBd+nz!3l{|rJ6dNcG*t$`}&b6|z)Q8lQtQqXj*DIK91Kp2gi6{wkUq^nB(1ebKK$Mcj5ntn9L=D-a;9gzMjao~7U2$\?kw0\?VX#pmUDrB)pKU7y(<EAGqYge^Svy0iizKcX<s@I!{pB<B7vWCItwCW5<E!JA@;!N&y1UhQ)YdFYCWV_PGRn{*T9)MgNp6Vbf2axJ_|#5My!QV6_+Eg^3\?oCvWgchf2U*JG(w`e+c@P58`VvkUA9NDF^NHYHy7&4LvIIkT@iRUN;ydTIu;\?%C-eNWD0QicaS7aGg7|^NmZ_5W_oolNZIhRm0va<QmKG~^StlXxQJo!f#s9q`}ui*26kF)mF6h1*qt=)%NX08!bql9e\?^Jd7jmJVkVAZ5|MKi|gDdjXN)<2&4)mVH3wTPKKWIQPs0oEJj=3-pfBwRjH`=3{h*aWJTH+-ruIe*fN#|7O_g0;`fs(D(wP=p)It~BEl%^reD9t<DEgGV;U8qh&q_Y+ZF6m)h)IWPQFfE>1T~)tmL3bXAFvPbX@7>%nY\?-O)Yg37Mz$EvV^0ks#eAVOSz<z(p&g34^TBsR{@e=2}*s6ro\?dpX3X4N{Pi9!>c)yT%45@b4$-GB&z6vTlQD-5gnsnVa;\?<P3oyJ@%%Tk}pLKrfQ2Y{3ydD)(`B>sG|MFCH(O0L{FFw=wjoeZzuOvfvqLG74&I|6PMg1N}B4Q7~\?$mlQdK&(v_jF}vr^FODv\?6B!vt*nIA(^!B5ALKwy\?2t^oin(s4e0KLWZ0TRDFXzT4aQ4DzbTV=mj52)X\?rv0}_L;y_Fb#FGO^jvwFUHZNtUV\?*~iD6rtk{MPNx6&Jvngmr++v=zJ1Z3|OU+_Z=Uyp75<OHu5=R|mXCf0Sa<dNzmcIZ~6tCEdwq{2~wWdx|er>1*tkCun=h_A2viP%mKdZ`=|6rhiil`r<\?$(-a-81O2fJ{Py0JY*3t*D~IN\?r-B^NpjGnmFP6OgE2K$+rT1qi0w~t<1@Fs(mGe}2kxLCo4zB|S4tu6XV76$P-8I2Nrh)~5qXQ_X)ZY2E;Rbo0JR~eJR87&nXY0BiA`TSRd1rt(!}3RH_hqB4(fFoiSg17xlT0T3*W+MEh*1czS3d;7d$vlwOE^#>c=5rZ*RI1AZJ#^A|k(baKK13f`(<l)Xp9rEi>9*<m}|#Ejw@I_Er<)@)03DWg`TI8|S|rSXX7EAoJsXRB+\?a9WoGlb8\?a&9H~6yCxrO-65CNOKABj04+jVI$2WeHItnhVAtiT<@yr2=CD\?kxoJsvl!RWoX(&Q$H60us;E=iMwbmNnok$1GPqZjv$-nS6Ozj@-|$Be_YxH&=sZ)jw@Ld|%%B4RmvIPWIMSmr|kG^YQ&9unyEOt5vE-M)wmfjDD0V8OBmiRd!W96)+OiE7eZ)0Fi*8PO}7i$V*|%ck\?LqSzs_{WJ!N&*hv=(-eM9VW7Uai_d\?a=YfMs$j6+(`JTHvGJU_#dZb=`$lX1tJJ%|n@&a^#U{ER&8^y;6)7ig)FGRjL\?!<D|WS+_n=*xx0^eIXS;=s*)Vhf\?L-)4p2a-g1u=9;bz=2SiZ!^kM9o(G~j67F#%MMN8I5yH`V1_{%(yg!Dx^dQY~TW-VGoVoPh6PdFp0P;yock=s81\?qKp)_lVX+OkDbX&2KXGAud+^FI@i(5P*2SuySf2U>M0Xx_BKi9CZFg6r+P!3i!YP+Hqz6=hHb2RuenECv45^<6%Y;UoKZAd^cr)eJGjEESCU=a1zbl~1$hjD%~d8%~6OY~NKVl!bLaTjlI3UUDvqrx7dR8z2$)BRq@034e3vx;6RvG)Ln|vaKsVR-@()N&uV#GH|$b^RyUUHYSV>ZB3;RC#1P8$U;CgC2g1$5--Mu5@(I^z<L8jFt*P~\?B_nIGioa2A5OQ-MoJ\?1U(MZ(Hy6QDN7$ACDe+Mo<m!;yB^Qt6r-qu#A`SF(N;Ji{^eaN|adJL<ZT7G*r2Rb3Ov<4)AMHqBW0R3a{K`ho(G(*FRDFMyl_`nxXW$gW<dg5X6CI2D4;rJM-xs94#\?BQb<Y|Vg21mOB8GuM62@#GiPnG*e013{M!e{=gpXuk~7-QiUME\?aG7+R~voJHT$",
">%WAk^dX9V2Qgz1jHy2usKyP=nik\?a\?MP$fN()*H{+F~${6q&m1I`22BM<g7>1ZZAwoa-=Q6S_`0OE)%kE_CceLa!3#%{V@F94-enRcDq6SkIR_V0cS+XFkF!mF}!Do5(rzZuYgzsO!L(v=B9Pis;`uCCDivKaRVwk$Rlv#_=IBAjJA9Q^=~e~UV3exnqKs_X24U&T=kg|NfP!kBOy-O<1PMv#&Ht_Z8PCIf)Nj+a`|W((|3|D2MzO}y^F#}3rBRMW~j*M<s;2C9&j\?1R#=(3G=X5<zeN`5HAlmE{1e^cN*;*qwYLBK@^4CiX7bjQo@jZxwVzAvHm}TrIKl@BdEA{fU$<8Mnf*Sn1PmOFLKeY7>4US&(HX(U3@f<@oKc0TC$!F~%U|s=|em$#Vf;qbG!V%r4UL!75ll>3BswdeL5TKG1XFZ6#_PuBAO4a7ms^voDgV8VLX8eTnn+57Pmivr`(xV2p$~p`Y4%(H#~aQ-cn6Vi^J_paggxp~lJ+=<H|kOAob6s~yYArsX^<+#rDbJ\?2VolW0+3n8YbjJ9HcMy6*E\?iU9n~H7ZY@uCptJ>VX$ujOLcsMgbP;rwSoPW^co1_J5gibTUk9iwj}PgOvwFU^-qFDhug;7sbHv{wC(V1JOd6UrsvCJO9&Y>XmcKvrG*m6p~agy+;|(j-c{Lk;ivUuU$i\?O5yhmth1t9$1V-6Z;^U75-^YZ$Q2J0tY;6_k0WRIaX&)>ND2SWyhkvIYk|IBIr\?=D$j*q!`<!f`WZXdspF4fks8JffReO}nq0T<|{&zmMkcR_*GS56%=1CD3@j6jXgdA8w!KM)WWlsXjACoFxIkJSq\?B~B((9NjM\?6C7GI@hgmT-x=amfcjm6SI*{j-zli7x&zVa|+9s++$^>aqgi#4G2Qp>ix&@hz(LfCqq\?mD`*9=Vt7HXF`+vhWh2)`VAXNr2vd!lz@~(Ri_iat@(2SX#-f>icCV-v%S(n6JcgOz`|(V%jr8cWnaeb|;{L+lUdF5_q02Qj%}bw=0HI$~k}WU*ovmsEf>BeZTsn=qD-3LRWQ29-P559Hb}#xaF8jTsH`pY;yXc(YA!\?YHlYG{+myDiv_rz\?}T7Mv*AKlG;dXAr5d+eH`o{vPO#->09{z1r#y>};JNIfFj*Ns1W@$;VYPe3n$sW-9K=fn|p{M5N\?2(nkm7VNudjX<CBjTUDutHcOF5EB^<hlmpYsZW|PJG)(o*P=4hMOEfPn%2}^vVKxLL+z>B\?i|;g7~_K}@j1G>^}iGX`Z<dj))fPNn=}>ao0!qfRocQx!u3-mx3^j+`jM-xn1*)gQGJ<16^E2E3_SnEfd5Ip_*}D|&a!P=Z(Otm$8-y;rOnXpViu!53PZ(LJO~%HXwpKu2&i7=g`HRDRmAmmDqpJtMPS0OZ#p|YAiIU7Ky-R\?pf@Knrb#mhaD%OPa^!oEPacSUrE|XSbK{Z$1u+)4Z0!kakN@=&2_ygj]",
};

#if defined(__linux__) && (defined(BOJ) || defined(BASM_CI))
int main() {}
#ifdef __cplusplus
extern "C"
#endif
int __libc_start_main(
    void *func_ptr,
    int argc,
    char* argv[],
    void (*init_func)(void),
    void (*fini_func)(void),
    void (*rtld_fini_func)(void),
    void *stack_end) {
#else
int main(int argc, char *argv[]) {
#endif
    PLATFORM_DATA pd;
    if (sizeof(size_t) != 8) {
        // Cannot run amd64 binaries on non-64bit environment
        return 1;
    }
    pd.env_flags            = 0; // necessary since pd is on stack
#if defined(_WIN32)
    pd.env_id               = ENV_ID_WINDOWS;
#elif defined(__linux__)
    pd.env_id               = ENV_ID_LINUX;
    // Linux's stack growth works differently than Windows.
    // Hence, we disable the __chkstk mechanism on Linux.
    pd.env_flags            |= ENV_FLAGS_LINUX_STYLE_CHKSTK;
#else
    pd.env_id               = ENV_ID_UNKNOWN;
#endif
#if defined(_WIN32)
    pd.win_kernel32         = (uint64_t) GetModuleHandleW(L"kernel32");
    pd.win_GetProcAddress   = (uint64_t) GetProcAddress;
#endif
    pd.ptr_alloc_rwx        = (void *) svc_alloc_rwx;
#if !defined(_WIN32) && !defined(__linux__)
    pd.ptr_alloc            = (void *) svc_alloc;
    pd.ptr_alloc_zeroed     = (void *) svc_alloc_zeroed;
    pd.ptr_dealloc          = (void *) svc_free;
    pd.ptr_realloc          = (void *) svc_realloc;
    pd.ptr_read_stdio       = (void *) svc_read_stdio;
    pd.ptr_write_stdio      = (void *) svc_write_stdio;
#endif

    stub_ptr stub = get_stub();
#if defined(__linux__)
    uint8_t stubbuf[68 + 580] = "QMd~L002n8@6D@;XGJ3cz5oya01pLO>naZmS5~+Q0000n|450>x(5IN07=KfA^-pYO)<bp|Hw@-$qxlyU&9Xz]";
    b85tobin(stubbuf, (char const *)stubbuf);
    /* prepend thunk and relocate stub onto stack */
    for (size_t i = 0; i < 580; i++) stubbuf[68 + i] = (uint8_t)stub_raw[i];
    size_t base = ((size_t)stub_raw) & 0xFFFFFFFFFFFFF000ULL; // page-aligned pointer to munmap in thunk
    size_t len = (((size_t)stub_raw) + sizeof(stub_raw)) - base;
    len = ((len + 0xFFF) >> 12) << 12;
    *(uint64_t *)(stubbuf + 0x08) = (uint64_t) base;
    *(uint32_t *)(stubbuf + 0x11) = (uint32_t) len;
    base = ((size_t)stubbuf) & 0xFFFFFFFFFFFFF000ULL;
    len = (((size_t)stubbuf) + 68 + 580) - base;
    len = ((len + 0xFFF) >> 12) << 12;
    syscall(10, base, len, 0x7); // mprotect: make the stub on stack executable
    pd.ptr_alloc_rwx = (void *) (stubbuf + 0x1c); // thunk implements its own svc_alloc_rwx
    stub = (stub_ptr) stubbuf;
#endif
    b85tobin(payload, (char const *)payload);
    return stub(&pd, payload);
}
// LOADER END
