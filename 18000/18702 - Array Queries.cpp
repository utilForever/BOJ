#pragma GCC optimize("O3")
#pragma GCC optimize("unroll-loops")
#pragma GCC target("avx,avx2,sse")

#include <cmath>
#include <cstdio>
#include <cstdint>
#include <vector>

inline int readChar();

template <class T = int>
inline T readInt();

template <class T>
inline void writeInt(T x, char end = 0);

inline void writeChar(int x);

inline void writeWord(const char *s);

constexpr static int BUF_SIZE = 1 << 18;

inline int getChar()
{
    static char buf[BUF_SIZE];
    static int len = 0, pos = 0;

    if (pos == len)
    {
        pos = 0, len = fread(buf, 1, BUF_SIZE, stdin);
    }

    if (pos == len)
    {
        return -1;
    }

    return buf[pos++];
}

inline int readChar()
{
    int c = getChar();

    while (c <= 32)
    {
        c = getChar();
    }

    return c;
}

template <class T>
inline T readInt()
{
    int s = 1, c = readChar();
    T x = 0;

    if (c == '-')
    {
        s = -1;
        c = getChar();
    }

    while ('0' <= c && c <= '9')
    {
        x = x * 10 + c - '0';
        c = getChar();
    }

    return s == 1 ? x : -x;
}

static int writePos = 0;
static char writeBuf[BUF_SIZE];

inline void writeChar(int x)
{
    if (writePos == BUF_SIZE)
    {
        fwrite(writeBuf, 1, BUF_SIZE, stdout);
        writePos = 0;
    }

    writeBuf[writePos++] = x;
}

template <class T>
inline void writeInt(T x, char end)
{
    if (x < 0)
    {
        writeChar('-');
        x = -x;
    }

    char s[24];
    int n = 0;
    
    while (x || !n)
    {
        s[n++] = '0' + x % 10;
        x /= 10;
    }

    while (n--)
    {
        writeChar(s[n]);
    }

    if (end)
    {
        writeChar(end);
    }
}

inline void writeWord(const char *s)
{
    while (*s)
    {
        writeChar(*s++);
    }
}

struct Flusher
{
    ~Flusher()
    {
        if (writePos)
        {
            fwrite(writeBuf, 1, writePos, stdout);
            writePos = 0;
        }
    }
} flusher;

struct Node
{
    std::int64_t min = 0;
    std::int64_t max = 0;
    std::int64_t sum = 0;
};

struct LazySegmentTree {
    std::size_t size;
    std::vector<Node> data;
    std::vector<std::int64_t> lazyAdd;
    std::vector<std::int64_t> lazySqrt;   

    LazySegmentTree(std::size_t n) : size(n)
    {
        std::size_t realN = 1;

        while (realN < n) {
            realN *= 2;
        }

        data.resize(realN * 4);
        data.clear();
        lazyAdd.resize(realN * 4);
        lazySqrt.resize(realN * 4);
    }

    Node merge(Node a, Node b)
    {
        Node ret;
        ret.min = std::min(a.min, b.min);
        ret.max = std::max(a.max, b.max);
        ret.sum = a.sum + b.sum;

        return ret;
    }

    void construct(std::vector<std::int64_t>& arr, std::size_t start, std::size_t end)
    {
        construct(arr, 1, start, end);
    }

    Node construct(std::vector<std::int64_t>& arr, std::size_t node, std::size_t start, std::size_t end)
    {
        if (start == end)
        {
            data[node].min = arr[start];
            data[node].max = arr[start];
            data[node].sum = arr[start];

            return data[node];
        }
        else
        {
            std::size_t mid = (start + end) / 2;
            Node left = construct(arr, node * 2, start, mid);
            Node right = construct(arr, node * 2 + 1, mid + 1, end);

            data[node] = merge(left, right);

            return data[node];
        }
    }

    void setSize(std::size_t n)
    {
        size = n;

        std::fill(data.begin(), data.end(), Node());
        std::fill(lazyAdd.begin(), lazyAdd.end(), 0);
        std::fill(lazySqrt.begin(), lazySqrt.end(), 0);
    }

    void propagate(std::size_t node, std::size_t start, std::size_t end)
    {
        if (lazyAdd[node] == 0 && lazySqrt[node] == 0)
        {
            return;
        }

        if (lazySqrt[node] == 0)
        {
            data[node].min += lazyAdd[node];
            data[node].max += lazyAdd[node];
            data[node].sum += lazyAdd[node] * (end - start + 1);

            if (start != end)
            {
                lazyAdd[node * 2] += lazyAdd[node];
                lazyAdd[node * 2 + 1] += lazyAdd[node];
            }
        }
        else
        {
            data[node].min = lazyAdd[node] + lazySqrt[node];
            data[node].max = lazyAdd[node] + lazySqrt[node];
            data[node].sum = (lazyAdd[node] + lazySqrt[node]) * (end - start + 1);

            if (start != end)
            {
                lazyAdd[node * 2] = lazyAdd[node];
                lazyAdd[node * 2 + 1] = lazyAdd[node];
                lazySqrt[node * 2] = lazySqrt[node];
                lazySqrt[node * 2 + 1] = lazySqrt[node];
            }
        }

        lazyAdd[node] = 0;
        lazySqrt[node] = 0;
    }

    void updateAdd(std::size_t start, std::size_t end, std::int64_t value)
    {
        updateAdd(start, end, value, 1, 1, size);
    }

    void updateAdd(std::size_t start, std::size_t end, std::int64_t value, std::size_t node, std::size_t nodeStart, std::size_t nodeEnd)
    {
        propagate(node, nodeStart, nodeEnd);
        
        if (end < nodeStart || nodeEnd < start)
        {
            return;
        }

        if (start <= nodeStart && nodeEnd <= end)
        {
            lazyAdd[node] = value;
            propagate(node, nodeStart, nodeEnd);
            return;
        }

        std::size_t mid = (nodeStart + nodeEnd) / 2;
        updateAdd(start, end, value, node * 2, nodeStart, mid);
        updateAdd(start, end, value, node * 2 + 1, mid + 1, nodeEnd);

        data[node] = merge(data[node * 2], data[node * 2 + 1]);
    }

    void updateSqrt(std::size_t start, std::size_t end)
    {
        updateSqrt(start, end, 1, 1, size);
    }

    void updateSqrt(std::size_t start, std::size_t end, std::size_t node, std::size_t nodeStart, std::size_t nodeEnd)
    {
        propagate(node, nodeStart, nodeEnd);

        if (end < nodeStart || nodeEnd < start)
        {
            return;
        }

        if (start <= nodeStart && nodeEnd <= end)
        {
            if (std::floor(std::sqrt(data[node].min)) == std::floor(std::sqrt(data[node].max)))
            {
                lazySqrt[node] = std::floor(std::sqrt(data[node].max));
                propagate(node, nodeStart, nodeEnd);
                return;
            }
            
            if (data[node].min + 1 == data[node].max)
            {
                lazyAdd[node] = std::floor(std::sqrt(data[node].min)) - data[node].min;
                propagate(node, nodeStart, nodeEnd);
                return;
            }
        }

        std::size_t mid = (nodeStart + nodeEnd) / 2;
        updateSqrt(start, end, node * 2, nodeStart, mid);
        updateSqrt(start, end, node * 2 + 1, mid + 1, nodeEnd);

        data[node] = merge(data[node * 2], data[node * 2 + 1]);
    }

    std::int64_t query(std::size_t start, std::size_t end)
    {
        return query(start, end, 1, 1, size);
    }

    std::int64_t query(std::size_t start, std::size_t end, std::size_t node, std::size_t nodeStart, std::size_t nodeEnd)
    {
        propagate(node, nodeStart, nodeEnd);

        if (end < nodeStart || nodeEnd < start)
        {
            return 0;
        }

        if (start <= nodeStart && nodeEnd <= end)
        {
            return data[node].sum;
        }

        std::size_t mid = (nodeStart + nodeEnd) / 2;
        return query(start, end, node * 2, nodeStart, mid) + query(start, end, node * 2 + 1, mid + 1, nodeEnd);
    }
};

// Reference: https://justicehui.github.io/hard-algorithm/2019/10/10/segment-tree-beats/
// Reference: https://justicehui.github.io/ps/2019/10/29/BOJ17476/
// Reference: https://justicehui.github.io/icpc/2020/05/12/BOJ18702/
// Reference: https://www.secmem.org/blog/2019/10/19/Segment-Tree-Beats/
int main()
{
    LazySegmentTree tree(100000);
    std::int64_t t = readInt();

    for (std::int64_t i = 0; i < t; ++i)
    {
        std::int64_t n = readInt();
        std::int64_t q = readInt();
        std::vector<std::int64_t> arr(n + 1);

        for (std::int64_t j = 1; j <= n; ++j)
        {
            arr[j] = readInt();
        }

        tree.setSize(n);
        tree.construct(arr, 1, n);

        for (std::int64_t j = 0; j < q; ++j)
        {
            std::int64_t command = readInt();

            if (command == 1)
            {
                std::int64_t l = readInt();
                std::int64_t r = readInt();
                tree.updateSqrt(l, r);
            }
            else if (command == 2)
            {
                std::int64_t l = readInt();
                std::int64_t r = readInt();

                writeInt(tree.query(l, r), '\n');
            }
            else
            {
                std::int64_t l = readInt();
                std::int64_t r = readInt();
                std::int64_t value = readInt();
                tree.updateAdd(l, r, value);
            }
        }
    }

    Flusher();

    return 0;
}