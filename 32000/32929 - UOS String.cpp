#include <iostream>
#include <string>

int main()
{
    int x;
    std::string uos = "UOS";

    std::cin >> x;
    std::cout << uos[(x - 1) % 3] << '\n';

    return 0;
}
