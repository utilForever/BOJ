#include <algorithm>
#include <vector>

int random(std::vector<int>& a, int start, int end, int k)
{
    if (start == end)
    {
        return a[start];
    }

    std::swap(a[start + rand() % (end - start + 1)], a[end]);

    int pivot_left = start;
    int pivot_right = end - 1;
    int index = start;

    while (index <= pivot_right)
    {
        if (a[index] < a[end])
        {
            std::swap(a[pivot_left], a[index]);
            pivot_left++;
            index++;
        }
        else if (a[index] > a[end])
        {
            std::swap(a[pivot_right], a[index]);
            pivot_right--;
        }
        else
        {
            index++;
        }
    }

    std::swap(a[pivot_right + 1], a[end]);

    if (pivot_left > k)
    {
        return random(a, start, pivot_left - 1, k);
    }
    else if (pivot_right < k)
    {
        return random(a, pivot_right + 1, end, k);
    }
    else
    {
        return a[pivot_left];
    }
}

int kth(std::vector<int>& a, int k)
{
    srand(1004);
    return random(a, 0, a.size() - 1, k - 1);
}
