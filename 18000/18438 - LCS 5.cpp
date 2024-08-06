#include <iostream>
#include <string>

int LCS1[2][7001] = { 0, };
int LCS2[2][7001] = { 0, };
std::string s, t;

// Reference: https://www.secmem.org/blog/2019/12/15/dpopt-ch2/
std::string ProcessLCSWithHirschburg(int s1, int s2, int t1, int t2)
{
    if (s1 == s2)
    {
        return "";
    }

    std::string ret = "";

    if (s1 + 1 == s2)
    {
        for (int i = t1 + 1; i <= t2; ++i)
        {
            if (s[s2] == t[i])
            {
                ret += t[i];
                return ret;
            }
        }

        return "";
    }

    int midS = (s1 + s2) / 2;

    for (int i = t1; i <= t2; ++i)
    {
        LCS1[0][i] = 0;
        LCS1[1][i] = 0;
        LCS2[0][i] = 0;
        LCS2[1][i] = 0;
    }

    for (int i = s1 + 1; i <= midS; ++i)
    {
        for (int j = t1 + 1; j <= t2; ++j)
        {
            if (s[i] == t[j])
            {
                LCS1[i % 2][j] = LCS1[(i + 1) % 2][j - 1] + 1;
            }
            else
            {
                LCS1[i % 2][j] = std::max(LCS1[(i + 1) % 2][j], LCS1[i % 2][j - 1]);
            }
        }
    }

    for (int i = s2 - 1; i >= midS; --i)
    {
        for (int j = t2 - 1; j >= t1; --j)
        {
            if (s[i + 1] == t[j + 1])
            {
                LCS2[i % 2][j] = LCS2[(i + 1) % 2][j + 1] + 1;
            }
            else
            {
                LCS2[i % 2][j] = std::max(LCS2[(i + 1) % 2][j], LCS2[i % 2][j + 1]);
            }
        }
    }

    int max = -1;
    int midT = 0;

    for (int i = t1; i <= t2; ++i)
    {
        if (LCS1[midS % 2][i] + LCS2[midS % 2][i] > max)
        {
            max = LCS1[midS % 2][i] + LCS2[midS % 2][i];
            midT = i;
        }
    }

    return ProcessLCSWithHirschburg(s1, midS, t1, midT)
        + ProcessLCSWithHirschburg(midS, s2, midT, t2);
}

int main()
{
    std::cin >> s;
    std::cin >> t;

    s.insert(s.begin(), 0);
    t.insert(t.begin(), 0);

    std::string lcs = ProcessLCSWithHirschburg(0, s.size() - 1, 0, t.size() - 1);

    std::cout << lcs.size() << '\n';
    std::cout << lcs << '\n';
}