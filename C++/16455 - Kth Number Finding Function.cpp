#include <algorithm>
#include <vector>

int kth(std::vector<int>& a, int k)
{
	std::nth_element(a.begin(), a.begin() + k - 1, a.end());
    
	return a[k - 1];
}
