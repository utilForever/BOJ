#include <iostream>

int main()
{
    std::ios::sync_with_stdio(0);
    std::cout.tie(0);

    int n, q;
    std::cin >> n >> q;

    for (int i = 0; i < q; ++i)
    {
        if (i & 1)
        {
            n++;
        }
        else
        {
            n--;
        }

        std::cout << n << '\n';
    }
}