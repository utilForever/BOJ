#include <iostream>
#include <stack>
#include <vector>

int main(int argc, char* argv[])
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    int c;
    std::cin >> c;

    std::vector<int> nums(c);

    for (auto& num : nums)
    {
        std::cin >> num;
    }

    std::stack<std::pair<int, int>> stack;
    stack.push(std::make_pair(0, 1));

    std::vector<int> ret;
    ret.reserve(c);

    for (auto& num : nums)
    {
        auto& [symbol, number] = stack.top();

        if (num > symbol + 1)
        {
            std::cout << "-1\n";
            return 0;
        }

        if (num == symbol + 1)
        {
            stack.push(std::make_pair(num, 1));
            ret.emplace_back(1);
        } else
        {
            while (stack.top().first > num)
            {
                stack.pop();
            }

            auto& [symbol2, number2] = stack.top();
            number2++;

            ret.emplace_back(number2);
        }
    }

    for (auto& num : ret)
    {
        std::cout << num << ' ';
    }

    std::cout << '\n';

    return 0;
}
