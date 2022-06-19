#include <iostream>
#include <string>
#include <vector>

std::string s, t;

// Reference: https://www.secmem.org/blog/2019/12/15/dpopt-ch2/
void ProcessWithHirschburg(int s1, int s2, int t1, int t2)
{
    if (s1 > s2 || t1 > t2)
    {
        return;
    }

    if (s1 == s2)
    {
        for (int i = t1; i < t2; ++i)
        {
            std::cout << "a " << t[i] << "\n";
        }

        return;
    }

    if (t1 == t2)
    {
        for (int i = s1; i < s2; ++i)
        {
            std::cout << "d " << s[i] << "\n";
        }

        return;
    }

    std::vector<int> ret1(t2 - t1 + 1), ret2(t2 - t1 + 1);
    std::vector<int> prev;
    const int mid = (s2 - s1) / 2;

    for (int i = 0; i <= t2 - t1; ++i)
    {
        ret1[i] = i;
        ret2[i] = t2 - t1 - i;
    }

    for (int i = 1; i <= mid; ++i)
    {
        prev = ret1;

        for (int j = 0; j <= t2 - t1; ++j)
        {
            int min = 1'000'000;
            const int flag = i && j && s[s1 + i - 1] == t[t1 + j - 1];

            if (i > 0)
            {
                min = std::min(min, prev[j] + 1);
            }

            if (j > 0)
            {
                min = std::min(min, ret1[j - 1] + 1);
            }

            if (i > 0 && j > 0)
            {
                min = std::min(min, prev[j - 1] + !flag);
            }

            ret1[j] = min;
        }
    }

    for (int i = s2 - s1 - 1; i > mid; --i)
    {
        prev = ret2;

        for (int j = t2 - t1; j >= 0; --j)
        {
            int min = 1'000'000;
            const int flag = s[s1 + i] == t[t1 + j];

            if (i < s2 - s1)
            {
                min = std::min(min, prev[j] + 1);
            }

            if (j < t2 - t1)
            {
                min = std::min(min, ret2[j + 1] + 1);
            }

            if (i < s2 - s1 && j < t2 - t1)
            {
                min = std::min(min, prev[j + 1] + !flag);
            }

            ret2[j] = min;
        }
    }

    int min = 1'000'000;
    int left = -1, right = -1;

    for (int i = 0; i <= t2 - t1; ++i)
    {
        if (min > ret1[i] + ret2[i] + 1)
        {
            min = ret1[i] + ret2[i] + 1;
            left = right = i;
        }

        if (i < t2 - t1)
        {
            const bool flag = s[s1 + mid] == t[t1 + i];

            if (min > ret1[i] + ret2[i + 1] + !flag)
            {
                min = ret1[i] + ret2[i + 1] + !flag;
                left = i;
                right = i + 1;
            }
        }
    }

    ProcessWithHirschburg(s1, s1 + mid, t1, t1 + left);

    if (left != right)
    {
        if (s[s1 + mid] != t[t1 + left])
        {
            std::cout << "m " << t[t1 + left] << "\n";
        }
        else
        {
            std::cout << "c " << t[t1 + left] << "\n";
        }
    }
    else
    {
        std::cout << "d " << s[s1 + mid] << "\n";
    }

    ProcessWithHirschburg(s1 + mid + 1, s2, t1 + right, t2);
}

int main()
{
    std::cin >> s;
    std::cin >> t;

    ProcessWithHirschburg(0, static_cast<int>(s.size()), 0,
                          static_cast<int>(t.size()));
}