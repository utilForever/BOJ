#include <iostream>
#include <vector>
#include <string>

int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    std::size_t n, m;
    std::cin >> n >> m;

    std::vector<int64_t> roads(n, 0);
    std::vector<int64_t> capacities(n - 1);

    for (std::size_t i = 0; i < n - 1; ++i)
    {
        std::cin >> capacities[i];
    }

    for (std::size_t i = 0; i < m; ++i)
    {
        std::size_t u, v;
        std::int64_t x;
        std::cin >> u >> v >> x;

        --u;
        --v;

        roads[u] += x;
        roads[v] -= x;
    }

    std::int64_t traffic = 0;

    for (std::size_t i = 0; i < n - 1; ++i)
    {
        traffic += roads[i];

        auto each = traffic / capacities[i];
        auto rem = traffic % capacities[i];
        auto val = (capacities[i] - rem) * each * each + rem * (each + 1) * (each + 1);

        std::cout << val << '\n';
    }

    return 0;
}
