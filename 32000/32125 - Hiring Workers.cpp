#include <iostream>
#include <map>
#include <vector>
#include <unordered_map>
#include <utility>

int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    int t;
    std::cin >> t;

    for (int i = 0; i < t; ++i)
    {
        int n, k;
        std::cin >> n >> k;

        std::vector<int> c(n), w(n);

        for (int j = 0; j < n; ++j)
        {
            std::cin >> c[j];
        }

        for (int j = 0; j < n; ++j)
        {
            std::cin >> w[j];
        }

        int count_a = 0, count_b = 0;
        long long sum_a = 0, sum_b = 0;
        long long ret = 0;

        std::unordered_map<int, std::map<long long, int>> map;
        map[0][0] = 1;

        for (int j = 0; j < n; ++j)
        {
            if (c[j] == 1)
            {
                count_a++;
                sum_a += w[j];
            }
            else
            {
                count_b++;
                sum_b += w[j];
            }

            int diff_count = count_a - count_b;
            long long diff_sum = sum_a - sum_b;

            if (map.find(diff_count) != map.end())
            {
                for (auto it = map[diff_count].lower_bound(diff_sum - k);
                     it != map[diff_count].end() && it->first <= diff_sum + k; ++it)
                {
                    ret += it->second;
                }
            }

            map[diff_count][diff_sum]++;
        }

        std::cout << ret << "\n";
    }

    return 0;
}
