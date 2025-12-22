#include <iostream>

int main(int argc, char *argv[])
{
    int x;
    std::cin >> x;

    std::cout << (x >= 6 ? "Success!" : "Oh My God!") << '\n';

    return 0;
}
