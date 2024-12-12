#include <bits/stdc++.h>

using namespace std;

struct Point
{
    long long x, y;

    bool operator==(const Point &other) const
    {
        return x == other.x && y == other.y;
    }

    bool operator<(const Point &other) const
    {
        if (x == other.x)
        {
            return y < other.y;
        }

        return x < other.x;
    }
};

struct Segment
{
    size_t idx;
    Point p, q;

    long double eval(long long x) const
    {
        long double dy = (long double)(q.y - p.y);
        long double dx = (long double)(q.x - p.x);
        long double t = ((long double)(x - p.x)) / dx;

        return (long double)p.y + dy * t;
    }

    bool operator==(const Segment &other) const
    {
        return p == other.p && q == other.q;
    }
};

struct SegmentComparator
{
    bool operator()(const Segment &a, const Segment &b) const
    {
        long long x = (a.p == b.p) ? min(a.q.x, b.q.x) : max(a.p.x, b.p.x);
        long double va = a.eval(x);
        long double vb = b.eval(x);
        long double diff = va - vb;
        const long double EPS = 1e-18L;

        if (fabsl(diff) <= EPS)
        {
            return a.idx < b.idx;
        }

        return diff < 0.0L;
    }
};

struct Event
{
    size_t idx;
    int direction;
};

int ccw(const Point &a, const Point &b, const Point &c)
{
    __int128 x1 = b.x - a.x;
    __int128 y1 = b.y - a.y;
    __int128 x2 = c.x - a.x;
    __int128 y2 = c.y - a.y;
    __int128 ret = x1 * y2 - x2 * y1;

    if (ret == 0)
    {
        return 0;
    }

    return (ret > 0) ? 1 : -1;
}

bool is_cross(const Segment &A, const Segment &B)
{
    const Point &a = A.p;
    const Point &b = A.q;
    const Point &c = B.p;
    const Point &d = B.q;

    if (a == c && ccw(a, b, d) != 0)
    {
        return false;
    }

    if (b == d && ccw(c, d, a) != 0)
    {
        return false;
    }

    if (a == d || b == c)
    {
        return false;
    }

    int t1 = ccw(a, b, c) * ccw(a, b, d);
    int t2 = ccw(c, d, a) * ccw(c, d, b);

    if (t1 < 0 && t2 < 0)
    {
        return true;
    }
    else if (t1 == 0 && t2 == 0)
    {
        return !(B.q < A.p || A.q < B.p);
    }
    else
    {
        return t1 <= 0 && t2 <= 0;
    }
}

bool is_intersect(vector<Segment> &segments)
{
    vector<Event> events;
    events.reserve(segments.size() * 2);

    for (size_t i = 0; i < segments.size(); i++)
    {
        Event startEvent{i, 0};
        Event endEvent{i, 1};

        events.push_back(startEvent);
        events.push_back(endEvent);
    }

    sort(events.begin(), events.end(), [&](const Event &a, const Event &b)
         {
        Point p1 = (a.direction == 0) ? segments[a.idx].p : segments[a.idx].q;
        Point p2 = (b.direction == 0) ? segments[b.idx].p : segments[b.idx].q;

        if (p1.x != p2.x)
        {
            return p1.x < p2.x;
        }
             
        if (-a.direction != -b.direction)
        {
            return (-a.direction) < (-b.direction);
        }
             
        return p1.y < p2.y; });

    set<Segment, SegmentComparator> active;

    for (auto &event : events)
    {
        if (event.direction == 0)
        {
            auto insertResult = active.insert(segments[event.idx]);
            if (!insertResult.second)
            {
                return true;
            }

            auto it = insertResult.first;
            if (it != active.begin())
            {
                auto prev = it;
                prev--;

                if (is_cross(*prev, *it))
                {
                    return true;
                }
            }

            auto next = it;
            next++;

            if (next != active.end())
            {
                if (is_cross(*it, *next))
                {
                    return true;
                }
            }
        }
        else
        {
            auto it = active.find(segments[event.idx]);

            if (it != active.end())
            {
                auto prev = it;
                auto next = it;
                bool hasPrev = false, hasNext = false;

                if (prev != active.begin())
                {
                    prev--;
                    hasPrev = true;
                }

                next++;

                if (next != active.end())
                {
                    hasNext = true;
                }

                if (hasPrev && hasNext)
                {
                    if (is_cross(*prev, *next))
                    {
                        return true;
                    }
                }

                active.erase(it);
            }
        }
    }

    return false;
}

int main()
{
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;
    cin >> n;

    vector<pair<Point, Point>> points(n);
    set<long long> slopes;

    for (int i = 0; i < n; i++)
    {
        long long x1, y1, x2, y2;
        cin >> x1 >> y1 >> x2 >> y2;

        points[i] = {{x1, y1}, {x2, y2}};

        if (x1 != x2)
        {
            long long slope = (y2 - y1) / (x2 - x1);
            slopes.insert(slope);
        }
    }

    long long k = 2000000001LL;

    vector<Segment> segments;
    segments.reserve(n);

    for (int i = 0; i < n; i++)
    {
        Point p = points[i].first;
        Point q = points[i].second;

        long long x1 = k * p.x - p.y;
        long long y1 = p.x + k * p.y;
        long long x2 = k * q.x - q.y;
        long long y2 = q.x + k * q.y;

        Point P{x1, y1}, Q{x2, y2};

        if (Q < P)
        {
            swap(P, Q);
        }

        Segment s;
        s.idx = i;
        s.p = P;
        s.q = Q;

        segments.push_back(s);
    }

    cout << (is_intersect(segments) ? 1 : 0) << "\n";

    return 0;
}
