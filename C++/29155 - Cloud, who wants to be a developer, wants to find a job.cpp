#include <algorithm>
#include <iostream>
#include <tuple>
#include <vector>

int main(int argc, char* argv[])
{
    std::ios::sync_with_stdio(0);
    std::cin.tie(0);
    std::cout.tie(0);

    int n;
    std::cin >> n;

    std::vector<std::tuple<int, int>> problems;
    std::vector<int> countsToSolve;
    std::vector<int> counts(5, 0);

    for (int i = 0; i < 5; ++i)
    {
        int count;
        std::cin >> count;

        countsToSolve.emplace_back(count);
    }

    for (int i = 0; i < n; ++i)
    {
        int difficulty, time;
        std::cin >> difficulty >> time;

        problems.emplace_back(std::make_tuple(difficulty, time));
    }

    std::sort(std::begin(problems), std::end(problems));

    int ret = std::get<1>(problems[0]);
    std::tuple<int, int> prev = problems[0];

    counts[std::get<0>(prev) - 1] += 1;

    for (int i = 1; i < n; ++i)
    {
        auto& [difficulty, time] = problems[i];

        if (counts[difficulty - 1] == countsToSolve[difficulty - 1])
        {
            continue;
        }

        ret += time + (difficulty == std::get<0>(prev) ? (time - std::get<1>(prev)) : 60);

        counts[difficulty - 1] += 1;
        prev = problems[i];
    }

    std::cout << ret << '\n';

    return 0;
}
