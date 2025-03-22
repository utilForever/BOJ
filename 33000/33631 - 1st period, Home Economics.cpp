#include <iostream>

int main(int argc, char *argv[])
{
    int fs, cs, es, bs;
    std::cin >> fs >> cs >> es >> bs;

    int fn, cn, en, bn;
    std::cin >> fn >> cn >> en >> bn;

    int q;
    std::cin >> q;

    int cntCookie = 0;

    for (int i = 0; i < q; ++i)
    {
        int cmd, val;
        std::cin >> cmd >> val;

        if (cmd == 1)
        {
            if (fs >= val * fn && cs >= val * cn && es >= val * en && bs >= val * bn)
            {
                cntCookie += val;
                std::cout << cntCookie << '\n';

                fs -= val * fn;
                cs -= val * cn;
                es -= val * en;
                bs -= val * bn;
            }
            else
            {
                std::cout << "Hello, siumii\n";
            }
        }
        else if (cmd == 2)
        {
            fs += val;
            std::cout << fs << '\n';
        }
        else if (cmd == 3)
        {
            cs += val;
            std::cout << cs << '\n';
        }
        else if (cmd == 4)
        {
            es += val;
            std::cout << es << '\n';
        }
        else
        {
            bs += val;
            std::cout << bs << '\n';
        }
    }

    return 0;
}
