#include <algorithm>
#include <iostream>
#include <string>

int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    int n, p;
    std::cin >> n >> p;

    std::string s;
    std::cin >> s;

    std::string ret = s;
    std::size_t pos = 0;

    while ((pos = ret.find('(', pos)) != std::string::npos)
    {
        ret.insert(pos, "(");
        pos += 2;
    }

    pos = 0;

    while ((pos = ret.find(')', pos)) != std::string::npos)
    {
        ret.insert(pos + 1, ")");
        pos += 2;
    }

    pos = 0;

    while ((pos = ret.find('+', pos)) != std::string::npos)
    {
        ret.replace(pos, 1, ")+(");
        pos += 3;
    }

    ret.insert(0, "(");
    ret.append(")");

    std::cout << ret.length() << '\n';
    std::cout << ret << '\n';

    return 0;
}
