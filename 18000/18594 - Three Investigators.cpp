#include <iostream>
#include <map>

class YoungTableaux
{
public:
    void clear()
    {
        for (int i = 0; i < 5; ++i)
        {
            tableaux[i].clear();
        }
    }

    void insert(int idx, int num, long long cnt, long long &sum)
    {
        if (idx == 5)
        {
            return;
        }

        tableaux[idx][num] += cnt;
        sum += cnt;

        for (auto iter = tableaux[idx].upper_bound(num); iter != tableaux[idx].end();)
        {
            int x = iter->first;
            long long& y = iter->second;
            long long rest = std::min(y, cnt);

            insert(idx + 1, x, rest, sum);

            sum -= rest;
            y -= rest;

            if (y == 0)
            {
                tableaux[idx].erase(iter++);
            }
            else
            {
                iter++;
            }

            cnt -= rest;

            if (cnt == 0)
            {
                break;
            }
        }
    }

private:
    std::map<int, long long> tableaux[5];
};

// Reference: https://codeforces.com/blog/entry/98167
// Reference: https://youngyojun.github.io/secmem/2021/09/19/young-tableaux/
int main()
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    int t;
    std::cin >> t;

    YoungTableaux yt;

    for (int i = 0; i < t; ++i)
    {
        int n;
        std::cin >> n;

        yt.clear();

        long long ret = 0;

        for (int j = 0; j < n; ++j)
        {
            int x;
            std::cin >> x;

            yt.insert(0, x, x, ret);

            std::cout << ret << " ";
        }

        std::cout << "\n";
    }

    return 0;
}
