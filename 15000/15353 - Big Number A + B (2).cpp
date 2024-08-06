#include <algorithm>
#include <iostream>
#include <string>

int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    std::string s, t;
    std::cin >> s >> t;

    int lenMax = s.size() > t.size() ? s.size() : t.size();
    std::string a = std::string(lenMax - s.size(), '0') + s;
    std::string b = std::string(lenMax - t.size(), '0') + t;

    std::string ret;
    int carry = 0;

    for (int i = lenMax - 1; i >= 0; --i)
    {
        int digit = (a[i] - '0') + (b[i] - '0') + carry;
        carry = digit / 10;
        ret += (digit % 10) + '0';
    }

    if (carry == 1)
    {
        ret += '1';
    }

    std::reverse(ret.begin(), ret.end());

    std::cout << ret << '\n';

    return 0;
}
