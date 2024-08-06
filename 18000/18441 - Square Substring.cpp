#define private public
#include <bitset>
#undef private
#include <iostream>
#include <string>
#include <vector>

#include <x86intrin.h>

std::string a, b;

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
std::string ProcessLCSWithBitset()
{
    std::bitset<sz> Match[26];
    std::vector<std::bitset<sz>> D(a.size() + 1);

    for (std::size_t i = 0; i < b.size(); ++i)
    {
        Match[b[i] - 'a'][i] = 1;
    }

    for (std::size_t i = 0; i < a.size(); ++i)
    {
        auto x = Match[a[i] - 'a'] | D[i];
        D[i + 1] = D[i] << 1;
        D[i + 1][0] = 1;

        D[i + 1] = x ^ (x & (x - D[i + 1]));
    }

    std::string ret;

    for (int i = a.size(), j = b.size() - 1; i > 0; --i)
    {
        while (j >= 0 && !D[i][j])
        {
            --j;
        }

        if (j < 0)
        {
            break;
        }

        while (i > 0 && D[i - 1][j])
        {
            --i;
        }

        ret.push_back(b[j--]);
    }

    std::reverse(ret.begin(), ret.end());

    return ret;
}

std::string ProcessLCS()
{
    int len = b.size();
    
    if (len <= 128)
    {
        return ProcessLCSWithBitset<128>();
    }
    if (len <= 256)
    {
        return ProcessLCSWithBitset<256>();
    }
    if (len <= 384)
    {
        return ProcessLCSWithBitset<384>();
    }
    if (len <= 512)
    {
        return ProcessLCSWithBitset<512>();
    }
    if (len <= 640)
    {
        return ProcessLCSWithBitset<640>();
    }
    if (len <= 768)
    {
        return ProcessLCSWithBitset<768>();
    }
    if (len <= 896)
    {
        return ProcessLCSWithBitset<896>();
    }
    if (len <= 1024)
    {
        return ProcessLCSWithBitset<1024>();
    }
    if (len <= 1152)
    {
        return ProcessLCSWithBitset<1152>();
    }
    if (len <= 1280)
    {
        return ProcessLCSWithBitset<1280>();
    }
    if (len <= 1408)
    {
        return ProcessLCSWithBitset<1408>();
    }
    
    return ProcessLCSWithBitset<1536>();
}

int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    int t;
    std::cin >> t;

    for (int i = 1; i <= t; ++i)
    {
        std::string s;
        std::cin >> s;

        std::string ans;

        for (std::size_t j = 1; j < s.size(); ++j)
        {
            auto str1 = s.substr(0, j);
            auto str2 = s.substr(j, s.size());

            if (str1.size() > str2.size())
            {
                a = str1;
                b = str2;
            }
            else
            {
                a = str2;
                b = str1;
            }

            auto ret = ProcessLCS();

            if (ans.size() < ret.size())
            {
                ans = ret;
            }
        }

        std::cout << "Case #" << i << ": " << 2 * ans.size() << '\n';
        if (!ans.empty())
        {
            std::cout << ans << ans << '\n';
        }
    }

    return 0;
}