#include <algorithm>
#include <iostream>
#include <set>
#include <tuple>
#include <vector>

int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    int n;
    std::cin >> n;

    std::vector<std::tuple<long long, long long, long long, bool>> foods;
    foods.reserve(n * 2);

    for (int i = 0; i < n; ++i)
    {
        long long xl, xr, y;
        char type;

        std::cin >> xl >> xr >> y >> type;

        foods.emplace_back(xl, y, xr - xl + 1, type == 'S');
        foods.emplace_back(xr + 1, y, xl - xr - 1, type == 'S');
    }

    std::sort(foods.begin(), foods.end());

    std::set<std::pair<long long, bool>> sets;
    long long ret = 0;
    long long sum = 0;
    long long cnt = 0;
    long long pos = 0;

    for (const auto& [pos_x, pos_y, dist, is_sausage] : foods)
    {
        if (pos != pos_x && cnt == 0)
        {
            ret = std::max(ret, sum);
        }

        sum += dist;
        pos = pos_x;

        auto iter = sets.lower_bound({ pos_y, 0 });

        if (dist > 0)
        {
            iter = sets.insert({ pos_y, is_sausage }).first;
        }

        if (iter != sets.begin() && std::prev(iter)->second == iter->second)
        {
            cnt += dist > 0 ? 1 : -1;
        }

        if (std::next(iter) != sets.end() &&
            iter->second == std::next(iter)->second)
        {
            cnt += dist > 0 ? 1 : -1;
        }

        if (iter != sets.begin() && std::next(iter) != sets.end() &&
            std::prev(iter)->second == std::next(iter)->second)
        {
            cnt -= dist > 0 ? 1 : -1;
        }

        if (dist < 0)
        {
            sets.erase(iter);
        }
    }

    std::cout << ret << '\n';

    return 0;
}
