#include <iostream>
#include <vector>

int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    int n;
    std::cin >> n;

    std::vector<int> a(n);

    for (int i = 0; i < n; ++i)
    {
        std::cin >> a[i];
    }

    std::vector<int> len(101);
    int longest = 0;

    for (int i = -99; i <= 99; ++i)
    {
        std::fill(len.begin(), len.end(), 0);
        
        for (int j = 0; j < n; ++j)
        {
            if (a[j] - i < 1 || a[j] - i > 100)
            {
                len[a[j]] = 1;
            }
            else
            {
                len[a[j]] = len[a[j] - i] + 1;
            }

            longest = std::max(longest, len[a[j]]);
        }
    }

    std::cout << longest << '\n';

    return 0;
}