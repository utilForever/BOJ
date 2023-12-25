#include <iostream>
#include <string>
#include <vector>

int main(int argc, char* argv[])
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    int n;
    std::cin >> n;

    std::vector<int> costs(1001, 0);
    int cnt_expensive = 0;
    int ret = 0;

    for (int i = 0; i < n; ++i)
    {
        std::string d;
        long long int c;

        std::cin >> d >> c;

        if (d == "jinju")
        {
            ret = c;
        }
        else if (c > 1000)
        {
            cnt_expensive += 1;
        }
        else
        {
            costs[c] += 1;
        }
    }

    int sum_expensive_costs = 0;

    for (int i = ret + 1; i < 1001; ++i)
    {
        sum_expensive_costs += costs[i];
    }

    std::cout << ret << '\n';
    std::cout << cnt_expensive + sum_expensive_costs << '\n';

    return 0;
}
