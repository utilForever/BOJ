#include <algorithm>
#include <iostream>
#include <set>
#include <vector>

enum class Direction : int
{
    LEFT = 0,
    RIGHT = 1,
};

enum class EventType : int
{
    RIGHT_UP,
    RIGHT_DOWN,
    LEFT_UP,
    LEFT_DOWN,
};

struct Point
{
    Point() = default;
    Point(long long _x, long long _y) : x(_x), y(_y)
    {
        // Do nothing
    }

    long long x, y;
};

struct Interval
{
    Interval(int _idx, const Point& p1, const Point& p2) : idx(_idx)
    {
        if (p1.x <= p2.x)
        {
            l = p1;
            r = p2;
        }
        else
        {
            l = p2;
            r = p1;
        }

        if (l.y < r.y)
        {
            direction = Direction::LEFT;
        }
        else
        {
            direction = Direction::RIGHT;
        }
	}

    bool operator==(const Interval& interval) const
    {
        return idx == interval.idx;
    }

	bool operator<(const Interval& interval) const
    {
        if (idx == interval.idx)
        {
            return false;
        }

        if (IsBetween(interval.l))
		{
            return IsBelow(interval.l);
        }

        if (interval.IsBetween(l))
		{
            return !interval.IsBelow(l);
        }
        
        return idx < interval.idx;
    } 

    bool IsBetween(const Point& p) const
    {
        return (l.x <= p.x) && (p.x <= r.x);
    }

    bool IsBelow(const Point& p) const
    {
        return ((r.x - l.x) * (p.y - l.y) - (r.y - l.y) * (p.x - l.x)) < 0;
    }

	Point l, r;
	Direction direction;
	int idx;
};

struct Event
{
    Event(const Interval& interval, Direction side) : num(interval.idx)
    {
        if (side == Direction::LEFT)
        {
            x = interval.l.x;
        }
        else
        {
            x = interval.r.x;
        }
        
        if (interval.direction == Direction::LEFT && side == Direction::LEFT)
        {
            type = EventType::LEFT_DOWN;
        }
        else if (interval.direction == Direction::RIGHT && side == Direction::LEFT)
        {
            type = EventType::LEFT_UP;
        }
        else if (interval.direction == Direction::LEFT && side == Direction::RIGHT)
        {
            type = EventType::RIGHT_UP;
        }
        else if (interval.direction == Direction::RIGHT && side == Direction::RIGHT)
        {
            type = EventType::RIGHT_DOWN;
        }
    }

	bool operator<(const Event& e) const
    {
        if (x != e.x)
        {
            return x < e.x;
        }
        else
        {
            return type > e.type;
        }
	}

    EventType type;
    long long x;
    int num;
};

int main()
{
    int n;
    std::cin >> n;

    std::vector<Interval> input;
    std::vector<Event> events;
	
	for (int i = 0; i < n; ++i)
    {
		long long x1, y1, x2, y2;
        std::cin >> x1 >> y1 >> x2 >> y2;
		
		Interval interval{ i, Point{ x1, y1 }, Point{ x2, y2 } };
		input.emplace_back(interval);
		events.emplace_back(Event{ interval, Direction::LEFT });
		events.emplace_back(Event{ interval, Direction::RIGHT });
	}

	long long x0;
	std::cin >> x0;
	
	Interval interval{ n, Point{ x0 - 1, 2'000'001 }, Point{ x0, 2'000'000 } };
	input.emplace_back(interval);
    events.emplace_back(Event{ interval, Direction::LEFT });
    events.emplace_back(Event{ interval, Direction::RIGHT });
	
	std::sort(events.begin(), events.end());

    std::set<Interval> sweep;
	std::vector<int> succ(n + 1, -2);

	for (int i = 0; i < events.size(); ++i)
    {
		Event e = events[i];

		if (e.type == EventType::LEFT_UP)
        {
			sweep.insert(input[e.num]);
		}
		else if (e.type == EventType::RIGHT_UP)
        {
			sweep.erase(input[e.num]);
		}
		else if (e.type == EventType::LEFT_DOWN)
        {
			sweep.insert(input[e.num]);

			auto iter = sweep.find(input[e.num]);
            ++iter;

			if (iter == sweep.end())
            {
                succ[e.num] = -1;
            }
            else
            {
                succ[e.num] = iter->idx;
            }
		}
		else if (e.type == EventType::RIGHT_DOWN)
        {
			auto iter = sweep.find(input[e.num]);
            ++iter;

			if (iter == sweep.end())
            {
                succ[e.num] = -1;
            }
            else
            {
                succ[e.num] = iter->idx;
            }

			sweep.erase(input[e.num]);
		}
	}

	int query = n;
	long long xPos = 0;

	while (succ[query] >= 0)
    {
        query = succ[query];
    }

	if (input[query].direction == Direction::LEFT)
    {
        xPos = input[query].l.x;
    }
    else
    {
        xPos = input[query].r.x;
    }

	std::cout << xPos << '\n';

	return 0;
}