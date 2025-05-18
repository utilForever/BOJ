#include <iostream>
#include <vector>

int main(int argc, char *argv[])
{
    int n;
    std::cin >> n;

    std::vector<int> nums(n);

    for (int i = 0; i < n; ++i)
    {
        std::cin >> nums[i];
    }

    long long ret = nums[n - 1];

    for (int i = n - 2; i >= 0; --i)
    {
        if (nums[i] > nums[i + 1])
        {
            nums[i] = nums[i + 1];
        }

        ret += nums[i];
    }

    std::cout << ret << '\n';

    return 0;
}
