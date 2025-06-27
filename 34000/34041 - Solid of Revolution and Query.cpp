#include <cmath>
#include <iomanip>
#include <iostream>
#include <vector>

const long double PI = std::acosl(-1.0);

struct Point
{
    long long x, y;
};

long long cross(const Point& a, const Point& b)
{
    return static_cast<long long>(a.x) * b.y -
           static_cast<long long>(a.y) * b.x;
}

int main()
{
    std::ios_base::sync_with_stdio(false);
    std::cin.tie(nullptr);
    std::cout.tie(nullptr);

    int n;
    std::cin >> n;

    std::vector<Point> polygon(n);

    for (int i = 0; i < n; ++i)
    {
        std::cin >> polygon[i].x >> polygon[i].y;
    }

    polygon.insert(polygon.end(), polygon.begin(), polygon.end());

    std::vector<long long> prefix_sum_area(2 * n + 1, 0);
    std::vector<long double> prefix_sum_moment_x(2 * n + 1, 0);
    std::vector<long double> prefix_sum_moment_y(2 * n + 1, 0);

    for (int i = 0; i < 2 * n; ++i)
    {
        long long c = cross(polygon[i], polygon[(i + 1) % (2 * n)]);

        prefix_sum_area[i + 1] = prefix_sum_area[i] + c;
        prefix_sum_moment_x[i + 1] =
            prefix_sum_moment_x[i] +
            static_cast<long double>(polygon[i].x +
                                     polygon[(i + 1) % (2 * n)].x) *
                c;
        prefix_sum_moment_y[i + 1] =
            prefix_sum_moment_y[i] +
            static_cast<long double>(polygon[i].y +
                                     polygon[(i + 1) % (2 * n)].y) *
                c;
    }

    long double area_total =
        std::fabsl(static_cast<long double>(prefix_sum_area[n])) * 0.5;
    long double center_x_total =
        prefix_sum_moment_x[n] / (3.0 * prefix_sum_area[n]);
    long double center_y_total =
        prefix_sum_moment_y[n] / (3.0 * prefix_sum_area[n]);

    auto dist = [&](const Point& a, const Point& b, long double cx,
                    long double cy) -> long double {
        long double dx = b.x - a.x, dy = b.y - a.y;
        long double num = dy * cx - dx * cy +
                          static_cast<long double>(a.y) * b.x -
                          static_cast<long double>(a.x) * b.y;

        return std::fabsl(num) / std::hypotl(dx, dy);
    };

    int q;
    std::cin >> q;
    std::cout << std::fixed << std::setprecision(12);

    while (q--)
    {
        int i, j;
        std::cin >> i >> j;

        --i;
        --j;

        if ((i + 1) % n == j || (j + 1) % n == i)
        {
            std::cout << "0\n";
            continue;
        }

        if (i > j)
        {
            std::swap(i, j);
        }

        long long area = (prefix_sum_area[j] - prefix_sum_area[i]) +
                         cross(polygon[j], polygon[i]);

        if (area == 0)
        {
            std::cout << "0\n";
            continue;
        }

        long long sum = prefix_sum_area[j] - prefix_sum_area[i] +
                        cross(polygon[j], polygon[i]);
        long double area1 = std::fabsl(sum) * 0.5;
        long double center_x1 =
            (prefix_sum_moment_x[j] - prefix_sum_moment_x[i] +
             static_cast<long double>(polygon[i].x + polygon[j].x) *
                 cross(polygon[j], polygon[i])) /
            (3.0 * sum);
        long double center_y1 =
            (prefix_sum_moment_y[j] - prefix_sum_moment_y[i] +
             static_cast<long double>(polygon[i].y + polygon[j].y) *
                 cross(polygon[j], polygon[i])) /
            (3.0 * sum);

        long double area2 = area_total - area1;
        long double center_x2 =
            (area_total * center_x_total - area1 * center_x1) / area2;
        long double center_y2 =
            (area_total * center_y_total - area1 * center_y1) / area2;

        if (std::min(area1, area2) < 1e-12)
        {
            std::cout << 0.0 << '\n';
            continue;
        }

        long double volume1 =
            area1 * 2.0L * PI *
            dist(polygon[i], polygon[j], center_x1, center_y1);
        long double volume2 =
            area2 * 2.0L * PI *
            dist(polygon[i], polygon[j], center_x2, center_y2);

        std::cout << std::min(volume1, volume2) << '\n';
    }

    return 0;
}
