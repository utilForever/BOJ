#include <iostream>
#include <set>

int main(int argc, char* argv[])
{
    int t;
    std::cin >> t;

    for (int i = 0; i < t; ++i)
    {
        int k;
        std::cin >> k;

        std::multiset<int> queue;

        for (int j = 0; j < k; ++j)
        {
            char c;
            int n;

            std::cin >> c >> n;

            if (c == 'I')
            {
                queue.insert(n);
            }
            else if (c == 'D')
            {
                if (n == 1)
                {
                    if (!queue.empty())
                    {
                        auto iter = queue.end();
                        --iter;
                        queue.erase(iter);
                    }
                }
                else if (n == -1)
                {
                    if (!queue.empty())
                    {
                        queue.erase(queue.begin());
                    }
                }
            }
        }

        if (queue.empty())
        {
            std::cout << "EMPTY\n";
        }
        else
        {
            auto iter = queue.end();
            --iter;

            std::cout << *iter << ' ' << *queue.begin() << '\n';
        }
    }

    return 0;
}