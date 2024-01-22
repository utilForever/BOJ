#include <iostream>
#include <random>

int main()
{
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<int> distribution(0, 2);

    std::cout << distribution(gen) << '\n';

    return 0;
}
