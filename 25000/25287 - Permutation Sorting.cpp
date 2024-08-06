#include <iostream>
#include <vector>

int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    int t;
    std::cin >> t;

    for (int i = 0; i < t; ++i)
    {
        int n;
        std::cin >> n;

        std::vector<int> arr(n);

        for (int j = 0; j < n; ++j)
        {
            std::cin >> arr[j];
        }

        bool check = true;

        for (int j = 0; j < n; ++j)
        {
            int val_min = std::min(arr[j], n - arr[j] + 1);
            int val_max = std::max(arr[j], n - arr[j] + 1);

            if (j == 0)
            {
                arr[j] = val_min;
            }
            else
            {
                if (val_min < arr[j - 1])
                {
                    if (val_max < arr[j - 1])
                    {
                        check = false;
                        break;
                    }
                    else
                    {
                        arr[j] = val_max;
                    }
                }
                else
                {
                    arr[j] = val_min;
                }
            }
        }

        if (check)
        {
            std::cout << "YES\n";
        }
        else
        {
            std::cout << "NO\n";
        }
    }
}