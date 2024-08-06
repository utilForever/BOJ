#include <algorithm>
#include <iostream>
#include <queue>
#include <tuple>
#include <vector>

struct Partial
{
    bool operator<(const Partial& p) const
    {
        if (time != p.time)
        {
            return time > p.time;
        }

        if (numPhoto != p.numPhoto)
        {
            return numPhoto < p.numPhoto;
        }

        return sumStart < p.sumStart;
    }

    long long sumStart;
    int time, curIdx, numPhoto, deadlineIdx;
    bool afterPhoto;
};

int main(int argc, char* argv[])
{
    int n, t;
    std::cin >> n >> t;

    std::vector<std::tuple<int, int>> workingTimes;
    workingTimes.resize(n);

    for (int i = 0; i < n; ++i)
    {
        std::cin >> std::get<0>(workingTimes[i]) >>
            std::get<1>(workingTimes[i]);
    }

    std::sort(workingTimes.begin(), workingTimes.end());

    bool ret = false;

    std::priority_queue<Partial> pq;
    std::vector<std::priority_queue<int>> deadlines(1);

    Partial start;
    start.afterPhoto = false;
    start.time = start.curIdx = start.numPhoto = start.deadlineIdx = 0;

    pq.push(start);

    while (!pq.empty())
    {
        Partial p = pq.top();

        while (!pq.empty() && pq.top().time == p.time)
        {
            pq.pop();
        }

        if (p.numPhoto == n)
        {
            ret = true;
            break;
        }

        if (p.afterPhoto)
        {
            deadlines[p.deadlineIdx].pop();
        }
        else
        {
            deadlines.emplace_back(deadlines[p.deadlineIdx]);
            p.deadlineIdx = deadlines.size() - 1;
        }

        std::priority_queue<int>& deadline = deadlines[p.deadlineIdx];

        for (; p.curIdx < workingTimes.size() &&
               std::get<0>(workingTimes[p.curIdx]) <= p.time;
             ++p.curIdx)
        {
            deadline.push(-std::get<1>(workingTimes[p.curIdx]));
        }

        if (!deadline.empty() && -deadline.top() < p.time + t)
        {
            continue;
        }

        if (p.curIdx < workingTimes.size() &&
            (deadline.empty() ||
             std::get<0>(workingTimes[p.curIdx]) < p.time + t))
        {
            p.afterPhoto = false;

            int tmp = p.time;
            p.time = std::get<0>(workingTimes[p.curIdx]);
            pq.push(p);
            p.time = tmp;
        }

        if (!deadline.empty())
        {
            p.afterPhoto = true;

            ++p.numPhoto;
            p.sumStart += p.time;
            p.time += t;
            pq.push(p);
        }
    }

    std::cout << (ret ? "yes\n" : "no\n");

    return 0;
}