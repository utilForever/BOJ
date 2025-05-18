#include <iostream>

int main(int argc, char *argv[])
{
    int bs, bc, bo, bn;
    std::cin >> bs >> bc >> bo >> bn;

    int cntC = bo * 2 + bc;
    int cntN = bs + bn;

    std::cout << std::min(cntC / 6, cntN / 3) << '\n';

    return 0;
}
