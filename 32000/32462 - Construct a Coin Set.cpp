#include <iostream>
#include <vector>

int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    int t;
    std::cin >> t;

    for (int i = 0; i < t; ++i)
    {
        int n;
        std::cin >> n;

        if (n < 6)
        {
            std::cout << -1 << '\n';
            continue;
        }
        
        if (n == 6)
        {
            std::cout << "3\n";
            std::cout << "1 3 4\n";
            continue;
        }

        std::cout << 4 << '\n';
        std::cout << "1 3 ";
        std::cout << n - 3 << ' ' << n - 2;
        std::cout << '\n';
    }

    return 0;
}
