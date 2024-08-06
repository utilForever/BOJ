#include <algorithm>
#include <iostream>
#include <set>
#include <tuple>
#include <vector>

std::set<std::tuple<int, int>> in;
std::vector<std::tuple<int, int>> fair[500'001];
int n, u, d, s;

int GetDist(int f, int t)
{
    return (f <= t) ? (t - f) * d : (f - t) * u;
}

int Query(int location)
{
    int res1, res2;
    auto iter = in.lower_bound(std::make_tuple(location, -2'000'000'000));

    res1 = (iter == in.end())
               ? -2'000'000'000
               : std::get<1>(*iter) - GetDist(std::get<0>(*iter), location);
    res2 = (iter == in.begin())
               ? -2'000'000'000
               : std::get<1>(*(--iter)) - GetDist(std::get<0>(*iter), location);

    return std::max(res1, res2);
}

void Update(int location, int val)
{
    if (Query(location) >= val)
    {
        return;
    }

    auto iter = std::get<0>(in.insert(std::make_tuple(location, val)));

    ++iter;

    while (iter != in.end() &&
           std::get<1>(*iter) <= val - GetDist(location, std::get<0>(*iter)))
    {
        iter = in.erase(iter);
    }

    --iter;

    while (iter != in.begin() &&
           std::get<1>(*(--iter)) <=
               val - GetDist(location, std::get<0>(*iter)))
    {
        iter = in.erase(iter);
    }
}

int main()
{
    std::cin >> n >> u >> d >> s;

    for (int i = 1; i <= n; ++i)
    {
        int t, l, m;
        std::cin >> t >> l >> m;

        fair[t].emplace_back(std::make_tuple(l, m));
    }

    Update(s, 0);

    for (int i = 1; i < 500'001; ++i)
    {
        if (fair[i].empty())
        {
            continue;
        }

        std::vector<std::tuple<int, int>> tmp;

        std::sort(fair[i].begin(), fair[i].end());

        for (std::size_t j = 0; j < fair[i].size(); ++j)
        {
            int res = std::get<1>(fair[i][j]) + Query(std::get<0>(fair[i][j]));
            tmp.emplace_back(std::make_tuple(res, res));
        }

        for (std::size_t j = 1; j < fair[i].size(); ++j)
        {
            std::get<0>(tmp[j]) = std::max(
                std::get<0>(tmp[j]), std::get<0>(tmp[j - 1]) -
                                         GetDist(std::get<0>(fair[i][j - 1]),
                                                 std::get<0>(fair[i][j])) +
                                         std::get<1>(fair[i][j]));
        }

        for (int j = fair[i].size() - 2; j >= 0; --j)
        {
            std::get<1>(tmp[j]) = std::max(
                std::get<1>(tmp[j]), std::get<1>(tmp[j + 1]) -
                                         GetDist(std::get<0>(fair[i][j + 1]),
                                                 std::get<0>(fair[i][j])) +
                                         std::get<1>(fair[i][j]));
        }

        for (std::size_t j = 0; j < fair[i].size(); ++j)
        {
            Update(std::get<0>(fair[i][j]),
                   std::max(std::get<0>(tmp[j]), std::get<1>(tmp[j])));
        }
    }

    std::cout << Query(s) << '\n';

    return 0;
}