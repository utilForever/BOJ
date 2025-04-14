#include <iostream>
#include <ext/rope>

int main()
{
    std::ios_base::sync_with_stdio(false);
    std::cin.tie(nullptr);

    int m;
    std::string s;
    std::cin >> m >> s;

    __gnu_cxx::rope<char> rope(s.c_str());

    int n;
    std::cin >> n;

    for (int i = 0; i < n; ++i)
    {
        int a, b, c;
        std::cin >> a >> b >> c;

        rope.insert(c, rope.substr(a, b - a));

        int len = rope.size();
        rope.erase(m, len - m + 1);
    }

    std::cout << rope;

    return 0;
}
