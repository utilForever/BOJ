#include <cmath>
#include <iostream>

int main()
{
    long double a, b, c;
    std::cin >> a >> b >> c;
    
    int curRepeat = 0;
    long double left = (c - b) / a - 0.001L, right = (c + b) / a + 0.001L, x = 0;
    
    while (true)
    {
        x = (left + right) / 2;
        
        long double res = a * x + b * std::sin(x);

        if (curRepeat > 10'000'000 || std::abs(res - c) < 1e-15)
        {
            break;
        }

        if (res < c)
        {
            left = x;
        }
        else
        {
            right = x;
        }

        ++curRepeat;
    }
        
    long long ans = x * 10'000'000;
    ans = ans / 10 + (ans % 10 > 4 ? 1 : 0);
    
    printf("%lld.%06lld", ans / 1'000'000, ans % 1'000'000);

    return 0;
}