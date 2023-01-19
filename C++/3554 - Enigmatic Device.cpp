#include <iostream>
#include <vector>

int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    std::size_t n;
    std::cin >> n;

    std::vector<int> sequence(n + 1);

    for (std::size_t i = 1; i <= n; ++i)
    {
        std::cin >> sequence[i];
    }

    int m;
    std::cin >> m;

    for (int i = 0; i < m; ++i)
    {
        int k;
        std::size_t l, r;
        std::cin >> k >> l >> r;

        if (k == 1) {
            for (std::size_t j = l; j <= r; ++j)
            {
                sequence[j] = (sequence[j] * sequence[j]) % 2010;
            }
        }
        else
        {
            int sum = 0;

            for (std::size_t j = l; j <= r; ++j)
            {
                sum += sequence[j];
            }

            std::cout << sum << '\n';
        }
    }
}
