#define private public
#include <bitset>
#undef private
#include <iostream>
#include <string>
#include <vector>

#include <x86intrin.h>

std::string s, t;

// Reference: https://gist.github.com/cgiosy/a441de545c9e96b1d7b02cc7a00561f9?fbclid=IwAR0N3Woe8GwzAsxMapbEE9b7rrE_XArl50BRdQ9ZOTCxk-2X5BRrm-HBVpo
template<std::size_t _Nw> void _M_do_sub(std::_Base_bitset<_Nw>& A, const std::_Base_bitset<_Nw>& B)
{
    for (std::size_t i = 0, c = 0; i < _Nw; i++)
    {
        c = _subborrow_u64(c, A._M_w[i], B._M_w[i], (unsigned long long*) & A._M_w[i]);
    }
}

template<> void _M_do_sub(std::_Base_bitset<1>& A, const std::_Base_bitset<1>& B)
{
    A._M_w -= B._M_w;
}
template<size_t _Nb> std::bitset<_Nb>& operator-=(std::bitset<_Nb>& A, const std::bitset<_Nb>& B)
{
    return _M_do_sub(A, B), A;
}

template<size_t _Nb> inline std::bitset<_Nb> operator-(const std::bitset<_Nb>& A, const std::bitset<_Nb>& B)
{
    std::bitset<_Nb> C(A);
    return C -= B;
}

// Reference: https://www.secmem.org/blog/2020/02/19/dpopt-ch3/
template<size_t sz>
std::vector<int> ProcessLCSWithBitset(int s1, int s2, int t1, int t2, bool reversed)
{
    std::bitset<sz> D, Match[26];
    std::vector<int> ret;
    ret.reserve(t2 - t1 + 3);

    if (reversed)
    {
        for (int i = t2; i >= t1; --i)
        {
            Match[t[i] - 'A'][t2 - i] = 1;
        }

        for (int i = s2; i >= s1; --i)
        {
            auto x = Match[s[i] - 'A'] | D;
            auto y = D << 1;
            y[0] = 1;

            D = x ^ (x & (x - y));
        }

        for (int i = t2; i >= t1; --i)
        {
            ret[i - t1 + 1] = ret[i - t1 + 2] + D[t2 - i];
        }
    }
    else
    {
        for (std::size_t i = t1; i <= t2; ++i)
        {
            Match[t[i] - 'A'][i - t1] = 1;
        }

        for (std::size_t i = s1; i <= s2; ++i)
        {
            auto x = Match[s[i] - 'A'] | D;
            auto y = D << 1;
            y[0] = 1;

            D = x ^ (x & (x - y));
        }

        for (int i = t1; i <= t2; ++i)
        {
            ret[i - t1 + 1] = ret[i - t1] + D[i - t1];
        }
    }

    return ret;
}

std::vector<int> ProcessLCS(int s1, int s2, int t1, int t2, bool reversed)
{
    int len = t2 - t1 + 1;
    
    if (len <= 64)
    {
        return ProcessLCSWithBitset<64>(s1, s2, t1, t2, reversed);
    }
    if (len <= 128)
    {
        return ProcessLCSWithBitset<128>(s1, s2, t1, t2, reversed);
    }
    if (len <= 256)
    {
        return ProcessLCSWithBitset<256>(s1, s2, t1, t2, reversed);
    }
    if (len <= 512)
    {
        return ProcessLCSWithBitset<512>(s1, s2, t1, t2, reversed);
    }
    if (len <= 1024)
    {
        return ProcessLCSWithBitset<1024>(s1, s2, t1, t2, reversed);
    }
    if (len <= 2048)
    {
        return ProcessLCSWithBitset<2048>(s1, s2, t1, t2, reversed);
    }
    if (len <= 4096)
    {
        return ProcessLCSWithBitset<4096>(s1, s2, t1, t2, reversed);
    }
    if (len <= 8192)
    {
        return ProcessLCSWithBitset<8192>(s1, s2, t1, t2, reversed);
    }
    if (len <= 16384)
    {
        return ProcessLCSWithBitset<16384>(s1, s2, t1, t2, reversed);
    }
    if (len <= 32767)
    {
        return ProcessLCSWithBitset<32767>(s1, s2, t1, t2, reversed);
    }

    return ProcessLCSWithBitset<65536>(s1, s2, t1, t2, reversed);   
}

// Reference: https://www.secmem.org/blog/2019/12/15/dpopt-ch2/
std::string ProcessLCSWithHirschburg(int s1, int s2, int t1, int t2)
{
    if (s1 > s2)
    {
        return "";
    }

    std::string ret = "";

    if (s1 == s2)
    {
        for (int i = t1; i <= t2; ++i)
        {
            if (s[s2] == t[i])
            {
                ret += t[i];
                return ret;
            }
        }

        return "";
    }

    int midS = (s1 + s2) / 2;

    auto LCS1 = ProcessLCS(s1, midS, t1, t2, false);
    auto LCS2 = ProcessLCS(midS + 1, s2, t1, t2, true);

    int max = -1;
    int midT = 0;

    for (int i = t1; i <= t2 + 1; ++i)
    {
        if (LCS1[i - t1] + LCS2[i - t1 + 1] > max)
        {
            max = LCS1[i - t1] + LCS2[i - t1 + 1];
            midT = i;
        }
    }

    return ProcessLCSWithHirschburg(s1, midS, t1, midT - 1)
        + ProcessLCSWithHirschburg(midS + 1, s2, midT, t2);
}

int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    std::cin >> s;
    std::cin >> t;

    std::string lcs = ProcessLCSWithHirschburg(0, s.size() - 1, 0, t.size() - 1);

    std::cout << lcs.size() << '\n';
    std::cout << lcs << '\n';
}