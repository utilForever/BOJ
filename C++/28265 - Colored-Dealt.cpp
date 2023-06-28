#include "colored_dealt.h"

#include <string>

std::string guess(int N)
{
    std::string s(N, 'B'), t(N, 'B');
    int scoreEstimate = 3 * N;

    for (int i = 0; i < N; ++i)
    {
        s[i] = 'R';

        int scoreQuery = design(s);

        t[i] = "BGR"[scoreEstimate - scoreQuery];

        scoreEstimate = scoreQuery;
    }

    return t;
}
