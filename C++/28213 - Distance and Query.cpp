#pragma GCC optimize("O3,unroll-loops")
#pragma GCC target("avx,avx2,fma")

#include <iostream>
#include <tuple>
#include <vector>

int main(int argc, char *argv[])
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    int n, m;
    std::cin >> n >> m;

    std::vector<int> arr(n);

    for (auto& elem : arr)
    {
        std::cin >> elem;
    }

    std::vector<std::tuple<int, int, int>> queries(m);

    for (auto& [l, r, d] : queries)
    {
        std::cin >> l >> r >> d;
    }

    for (int block = 0; block * 512 < n; ++block)
    {
        for (auto& [l, r, d] : queries)
        {
            for (int i = std::max(block * 512, l - 1); i < std::min(std::min((block + 1) * 512, r), n); ++i)
            {
                arr[i] = std::abs(arr[i] - d);
            }
        }
    }

    for (auto& val : arr)
    {
        std::cout << val << ' ';
    }

    std::cout << '\n';
}
