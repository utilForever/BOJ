#include <iostream>
#include <string>

int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    int n;
    std::cin >> n;

    std::string s;
    std::cin >> s;

    for (int i = 0; i < n; ++i)
    {
        std::cout << s;
    }

    return 0;
}