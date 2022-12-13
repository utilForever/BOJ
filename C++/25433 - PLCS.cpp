#define private public
#include <bitset>
#undef private
#include <algorithm>
#include <cmath>
#include <iostream>
#include <string>

#include <x86intrin.h>

std::string s, t;
char x, y;

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

bool IsPrime(int num)
{
    if (num < 2)
    {
        return false;
    }
    
	int limit = static_cast<int>(std::sqrt(num));
    
	for (int i = 2; i <= limit; ++i)
    {
        if (num % i == 0)
        {
            return false;
        }
    }

	return true;
}

// Reference: https://www.secmem.org/blog/2020/02/19/dpopt-ch3/
std::size_t ProcessLCSWithBitset(std::string s1, std::string s2)
{
    std::bitset<50000> D, Match[26];

    for (std::size_t i = 0; i < s2.size(); ++i)
    {
        Match[s2[i] - 'a'][i] = 1;
    }

    for (std::size_t i = 0; i < s1.size(); ++i)
    {
        auto x = Match[s1[i] - 'a'] | D;
        auto y = D << 1;
        y[0] = 1;
        
        D = x ^ (x & (x - y));
    }

    return D.count();
}

int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    std::cin >> s;
    std::cin >> t;
    std::cin >> x;
    std::cin >> y;

    s.erase(std::remove(s.begin(), s.end(), y), s.end());
    t.erase(std::remove(t.begin(), t.end(), y), t.end());

    std::size_t pos_s = s.find(x), pos_t = t.find(x);

    std::string s1 = s.substr(0, pos_s), s2 = s.substr(pos_s + 1);
    std::string t1 = t.substr(0, pos_t), t2 = t.substr(pos_t + 1);

    int ret1 = static_cast<int>(ProcessLCSWithBitset(s1, t1));
    int ret2 = static_cast<int>(ProcessLCSWithBitset(s2, t2));

    for (int i = ret1 + ret2 + 1; i >= 0; --i)
    {
        if (IsPrime(i))
        {
            std::cout << i << '\n';
            return 0;
        }
    }

    std::cout << -1 << '\n';
}
