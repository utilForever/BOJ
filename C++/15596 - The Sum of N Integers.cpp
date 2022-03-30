#include <vector>

long long sum(std::vector<int>& a)
{
    long long ret = 0;

    for (int i = 0; i < a.size(); ++i)
    {
        ret += a[i];
    }
    
    return ret;
}