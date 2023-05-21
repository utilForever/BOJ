#include <iostream>
#include <string>

int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    int n, a, b;
    std::cin >> n >> a >> b;

    if (a < b)
    {
        std::cout << "Bus\n";
    }
    else if (a > b)
    {
        std::cout << "Subway\n";
    }
    else
    {
        std::cout << "Anything\n";
    }

    return 0;
}
