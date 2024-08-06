#include <algorithm>
#include <iostream>
#include <string>

int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    int ret1[3];
    std::pair<int, std::string> ret2[3];

    for (int i = 0; i < 3; ++i)
    {
        int problems, year;
        std::string name;

        std::cin >> problems >> year >> name;

        ret1[i] = year % 100;
        ret2[i] = std::make_pair(problems, name);
    }

    std::sort(ret1, ret1 + 3);
    std::sort(
        ret2, ret2 + 3,
        [](const std::pair<int, std::string>& a,
           const std::pair<int, std::string>& b) { return a.first > b.first; });

    std::cout << ret1[0] << ret1[1] << ret1[2] << '\n';
    std::cout << ret2[0].second[0] << ret2[1].second[0] << ret2[2].second[0]
              << '\n';

    return 0;
}
