#include <iostream>
#include <string>

int main()
{
    std::ios_base::sync_with_stdio(false);
    std::cin.tie(nullptr);

    int ret = 0;

    while (true)
    {
        std::string s;
        std::getline(std::cin, s);

        if (!std::cin)
        {
            break;
        }

        int len = s.size();

        for (int i = 0; i < len - 3; ++i)
        {
            if (s.substr(i, 4) == "joke")
            {
                ret++;
            }
        }
    }

    std::cout << ret << '\n';

    return 0;
}
